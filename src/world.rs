use crate::Plane;
use crate::BLACK;
use crate::lighting::lighting;
use crate::ray::Intersection;
use crate::ray::{hit, prepare_computations, Computation, Ray};
use crate::sphere::Sphere;
use crate::Color;
use crate::Matrix4;
use crate::PointLight;
use crate::Tuple;
use crate::shape::Shape;

pub struct World<'a> {
    pub objects: Vec<&'a dyn Shape>,
    pub light: PointLight,
}

impl <'a>World<'a> {    
    pub fn default(s1: &'a dyn Shape, s2: &'a dyn Shape) -> Self {
        let light = PointLight::new(
            Tuple::point(-10_f32, 10_f32, -10_f32),
            Color::new(1_f32, 1_f32, 1_f32),
        );    
        
        World {
            objects: vec![s1, s2],
            light,
        }
    }

    pub fn default_spheres() -> (Sphere, Sphere) {
        let mut s1 = Sphere::new(1);
        s1.material.color = Color::new(0.8_f32, 1_f32, 0.6_f32);
        s1.material.diffuse = 0.7_f32;
        s1.material.specular = 0.2_f32;

        let mut s2 = Sphere::new(2);        
        s2.set_transformation(Matrix4::identity().scale(0.5_f32, 0.5_f32, 0.5_f32));
        (s1, s2)
    }

    pub fn new(light: PointLight) -> Self {
        World {
            objects: vec![],
            light,
        }
    }
}

fn intersect_world<'a>(world: &'a World, ray: &Ray) -> Vec<Intersection<'a>> {
    let mut intersections: Vec<Intersection> = vec![];
    for obj in world.objects.iter() {
        intersections.extend((*obj).intersections_by(ray));
    }
    intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    intersections
}

fn shade_hit(world: &World, computation: &Computation, remaining: u32) -> Color {
    let is_shadowed = is_shadowed(world, &computation.over_point);
    
    let surface_color = lighting(
        &computation.object.get_material(),
        computation.object,
        &world.light,
        &computation.point,
        &computation.eye_direction,
        &computation.surface_normalv,
        is_shadowed,
    );

    let reflected = reflected_color(&world, &computation, remaining);

    surface_color + reflected
}

pub fn color_at(world: &World, ray: &Ray, remaining: u32) -> Color {
    let intersections = intersect_world(world, ray);
    let hit = hit(&intersections);

    match hit {
        Some(h) => {
            let comp = prepare_computations(h, ray);
            return shade_hit(world, &comp, remaining);
        }
        None => Color::new(0_f32, 0_f32, 0_f32),
    }
}

pub fn reflected_color<'a>(world: &'a World, comps: &Computation, remaining: u32) -> Color {
    let reflective = comps.object.get_material().reflective;
    if remaining <= 0 || comps.object.get_material().reflective == 0_f32 {
        return BLACK;
    }
    
    let reflected_ray = Ray::new(comps.over_point, comps.reflectv);
    let color = color_at(world, &reflected_ray, remaining - 1);

    color * reflective
}

pub fn is_shadowed<'a>(world: &'a World, point: &Tuple) -> bool {
    let vector = world.light.position - *point;
    let distance = vector.mag();
    let direction = vector.normalize();

    let ray = Ray::new(*point, direction);
    let intersections = intersect_world(world, &ray);
    let hit = hit(&intersections);

    match hit {
        Some(h) => return h.t < distance,
        None => return false,
    }
}


#[test]
fn intersect_a_world_with_ray() {        
    let default : (Sphere, Sphere) = World::default_spheres();
    let world: World = World::default(&default.0, &default.1);
    let ray = Ray::default();
    let xs = intersect_world(&world, &ray);
    assert_eq!(xs.len(), 4);
    assert_eq!(xs[0].t, 4_f32);
    assert_eq!(xs[1].t, 4.5_f32);
    assert_eq!(xs[2].t, 5.5_f32);
    assert_eq!(xs[3].t, 6_f32);
}

#[test]
fn shading_an_interection() {   
    let default : (Sphere, Sphere) = World::default_spheres();
    let world: World = World::default(&default.0, &default.1); 
    let ray = Ray::default();
    let i = Intersection {
        obj: world.objects[0],
        t: 4_f32,
    };

    let comps = prepare_computations(&i, &ray);
    let color = shade_hit(&world, &comps, 0);
    assert_eq!(color, Color::new(0.38066_f32, 0.47583_f32, 0.2855_f32));
}

#[test]
fn shading_an_interection_from_the_inside() {  
    let default : (Sphere, Sphere) = World::default_spheres();
    let mut world: World = World::default(&default.0, &default.1);
    world.light.position = Tuple::point(0_f32, 0.25_f32, 0_f32);
    let mut ray = Ray::default();
    ray.origin = Tuple::point(0_f32, 0_f32, 0_f32);
    let i = Intersection {
        obj: world.objects[1],
        t: 0.5_f32,
    };

    let comps = prepare_computations(&i, &ray);
    let color = shade_hit(&world, &comps, 0);
    assert_eq!(color, Color::new(0.90498_f32, 0.90498_f32, 0.90498_f32));
}

#[test]
fn the_color_when_a_ray_misses() {    
    let default : (Sphere, Sphere) = World::default_spheres();
    let world: World = World::default(&default.0, &default.1);
    let mut ray = Ray::default();
    ray.direction = Tuple::vector(0_f32, 1_f32, 0_f32);
    let color = color_at(&world, &ray, 0);
    assert_eq!(color, Color::new(0_f32, 0_f32, 0_f32));
}

#[test]
fn the_color_when_a_ray_hits() { 
    let default : (Sphere, Sphere) = World::default_spheres();
    let world: World = World::default(&default.0, &default.1);   
    let ray = Ray::default();
    let color = color_at(&world, &ray, 0);
    assert_eq!(color, Color::new(0.38066_f32, 0.47583_f32, 0.2855_f32));
}

#[test]
fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() { 
    let default : (Sphere, Sphere) = World::default_spheres();
    let world: World = World::default(&default.0, &default.1);   
    let point = Tuple::point(0_f32, 10_f32, 0_f32);
    assert_eq!(is_shadowed(&world, &point), false);
}

#[test]
fn the_shadow_when_an_object_is_between_the_point_and_the_light() {    
    let default : (Sphere, Sphere) = World::default_spheres();
    let world: World = World::default(&default.0, &default.1);
    let point = Tuple::point(10_f32, -10_f32, 10_f32);
    assert_eq!(is_shadowed(&world, &point), true);
}

#[test]
fn there_is_no_shadow_when_an_objet_is_behind_the_light() { 
    let default : (Sphere, Sphere) = World::default_spheres();
    let world: World = World::default(&default.0, &default.1);   
    let point = Tuple::point(-20_f32, 20_f32, -20_f32);
    assert_eq!(is_shadowed(&world, &point), false);
}

#[test]
fn there_is_no_shadow_when_an_object_is_behind_the_point() {  
    let default : (Sphere, Sphere) = World::default_spheres();
    let world: World = World::default(&default.0, &default.1);  
    let point = Tuple::point(-2_f32, 2_f32, -2_f32);
    assert_eq!(is_shadowed(&world, &point), false);
}

#[test]
fn shade_hit_is_given_an_intersection_in_shadow() {
    let light = PointLight::new(
        Tuple::point(0_f32, 0_f32, -10_f32),
        Color::new(1_f32, 1_f32, 1_f32),
    );

    let mut w = World::new(light);    

    let s1 = Sphere::new(1);    
    let mut s2 = Sphere::new(2);
    s2.set_transformation(Matrix4::identity().translate(0_f32, 0_f32, 10_f32));
    w.objects.push(&s1);
    w.objects.push(&s2);

    let ray = Ray::new(Tuple::point(0_f32, 0_f32, 5_f32), Tuple::vector(0_f32, 0_f32, 1_f32));
    let i = Intersection {
        obj: w.objects[1],
        t: 4_f32
    };

    let comps = prepare_computations(&i, &ray);
    let color = shade_hit(&w, &comps, 0);
    assert_eq!(color, Color::new(0.1_f32, 0.1_f32, 0.1_f32));    
}

#[test]
fn the_reflected_color_for_a_nonreflective_material() {
    let mut default : (Sphere, Sphere) = World::default_spheres();
    default.1.material.ambient = 1_f32;
    let world: World = World::default(&default.0, &default.1);  

    let ray = Ray::new(Tuple::point(0_f32, 0_f32, 0_f32), Tuple::vector(0_f32, 0_f32, 1_f32));
    let intersection =  Intersection {
        obj: &default.1,
        t: 1_f32
    };

    let comps = prepare_computations(&intersection, &ray);
    let color = reflected_color(&world, &comps, 1);
    assert_eq!(color, BLACK);
}

#[test]
fn the_reflected_color_for_a_reflective_material() {
    let mut default : (Sphere, Sphere) = World::default_spheres();
    default.1.material.ambient = 1_f32;
    let mut world: World = World::default(&default.0, &default.1);  

    let mut plane = Plane::new();
    plane.material.reflective = 0.5_f32;
    plane.set_transformation(Matrix4::identity().translate(0_f32, -1_f32, 0_f32));

    world.objects.push(&plane);

    let ray = Ray::new(Tuple::point(0_f32, 0_f32, -3_f32), Tuple::vector(0_f32, -(2_f32.sqrt()/2f32), 2_f32.sqrt()/2f32));
    let intersection =  Intersection {
        obj: &plane,
        t: 2_f32.sqrt()
    };

    let comps = prepare_computations(&intersection, &ray);
    let color = reflected_color(&world, &comps, 1);
    assert_eq!(color, Color::new(0.19032_f32, 0.2379_f32, 0.14274_f32));
}

#[test]
fn shade_hit_with_a_reflective_material() {
    let mut default : (Sphere, Sphere) = World::default_spheres();
    default.1.material.ambient = 1_f32;
    let mut world: World = World::default(&default.0, &default.1);  

    let mut plane = Plane::new();
    plane.material.reflective = 0.5_f32;
    plane.set_transformation(Matrix4::identity().translate(0_f32, -1_f32, 0_f32));

    world.objects.push(&plane);

    let ray = Ray::new(Tuple::point(0_f32, 0_f32, -3_f32), Tuple::vector(0_f32, -(2_f32.sqrt()/2f32), 2_f32.sqrt()/2f32));
    let intersection =  Intersection {
        obj: &plane,
        t: 2_f32.sqrt()
    };

    let comps = prepare_computations(&intersection, &ray);
    let color = shade_hit(&world, &comps, 1);
    assert_eq!(color, Color::new(0.87677_f32, 0.92436_f32, 0.82918_f32));
}

#[test]
fn color_at_with_mutually_reflective_surfaces_completes() {
    let light = PointLight::new(Tuple::point(0_f32, 0_f32, 0_f32), Color::new(1_f32, 1_f32, 1_f32));
    let mut world = World::new(light);
    
    let mut lower = Plane::new();
    lower.material.reflective = 1_f32;
    lower.set_transformation(Matrix4::identity().translate(0_f32, -1_f32, 0_f32));


    let mut upper = Plane::new();
    upper.material.reflective = 1_f32;
    upper.set_transformation(Matrix4::identity().translate(0_f32, 1_f32, 0_f32));

    world.objects.push(&lower);
    world.objects.push(&upper);

    let ray = Ray::new(Tuple::point(0_f32, 0_f32, 0_f32), Tuple::vector(0_f32, 1_f32, 0_f32));
    color_at(&world, &ray, 5);
}

#[test]
fn the_reflected_color_at_the_maximum_recursive_depth() {
    let mut default : (Sphere, Sphere) = World::default_spheres();
    default.1.material.ambient = 1_f32;
    let mut world: World = World::default(&default.0, &default.1);  

    let mut plane = Plane::new();
    plane.material.reflective = 0.5_f32;
    plane.set_transformation(Matrix4::identity().translate(0_f32, -1_f32, 0_f32));

    world.objects.push(&plane);

    let ray = Ray::new(Tuple::point(0_f32, 0_f32, -3_f32), Tuple::vector(0_f32, -(2_f32.sqrt()/2f32), 2_f32.sqrt()/2f32));
    let intersection =  Intersection {
        obj: &plane,
        t: 2_f32.sqrt()
    };

    let comps = prepare_computations(&intersection, &ray);
    let color = reflected_color(&world, &comps, 0);
    assert_eq!(color, BLACK);
}