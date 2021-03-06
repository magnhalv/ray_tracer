use crate::lighting::lighting;
use crate::ray::schlick;
use crate::ray::Intersection;
use crate::ray::{hit, prepare_computations, Computation, Ray};
use crate::shape::Shape;
use crate::Color;
use crate::PointLight;
use crate::Tuple;
use crate::BLACK;

pub struct World {
    pub objects: Vec<Box<dyn Shape>>,
    pub light: PointLight,
}

impl World {
    pub fn new(light: PointLight) -> Self {
        World {
            objects: vec![],
            light,
        }
    }
}

pub fn intersect_world<'a>(world: &'a World, ray: &Ray) -> Vec<Intersection<'a>> {
    let mut xs: Vec<_> = world
        .objects
        .iter()
        .flat_map(|obj| (*obj).intersections_by(ray))
        .collect();
    xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    xs
}

fn shade_hit(world: &World, computation: &Computation, remaining: u32) -> Color {
    let is_shadowed = is_shadowed(world, &computation.over_point);
    let surface = lighting(
        &computation.object.get_material(),
        computation.object,
        &world.light,
        &computation.over_point,
        &computation.eye_direction,
        &computation.surface_normalv,
        is_shadowed,
    );

    let reflected = reflected_color(&world, &computation, remaining);
    let refracted = refracted_color(&world, &computation, remaining);

    let material = computation.object.get_material();
    if material.reflective > 0.0 && material.transparency > 0.0 {
        let reflectance = schlick(computation);
        return surface + reflected * reflectance + refracted * (1.0 - reflectance);
    }

    surface + reflected + refracted
}

pub fn color_at(world: &World, ray: &Ray, remaining: u32) -> Color {
    let intersections = intersect_world(world, ray);
    let hit = hit(&intersections);

    let color;
    match hit {
        Some(h) => {
            //println!("Hit object: {}", h.obj.get_id());
            let comp = prepare_computations(h, ray, &intersections);
            color = shade_hit(world, &comp, remaining);
        }
        None => color = Color::new(0.0, 0.0, 0.0),
    }
    //println!("Color {}, remaining: {}", color, remaining);
    color
}

pub fn reflected_color<'a>(world: &'a World, comps: &Computation, remaining: u32) -> Color {
    let reflective = comps.object.get_material().reflective;
    if remaining <= 0 || comps.object.get_material().reflective == 0.0 {
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

pub fn refracted_color<'a>(world: &'a World, comps: &Computation, remaining: u32) -> Color {
    if remaining == 0 {
        return BLACK;
    }
    if comps.object.get_material().transparency == 0.0 {
        return BLACK;
    }

    let n_ratio = comps.n1 / comps.n2;
    let cos_i = comps.eye_direction.dot(&comps.surface_normalv);
    let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));
    if sin2_t > 1.0 {
        return BLACK;
    }

    let cos_t = (1.0 - sin2_t).sqrt();

    let direction =
        comps.surface_normalv * (n_ratio * cos_i - cos_t) - comps.eye_direction * n_ratio;
    let refract_ray = Ray::new(comps.under_point, direction);

    let color =
        color_at(world, &refract_ray, remaining - 1) * comps.object.get_material().transparency;
    return color;
}

#[cfg(test)]
mod tests {

    use crate::ray::{Intersection, Ray};
    use crate::world::{intersect_world, prepare_computations, shade_hit, color_at, is_shadowed, reflected_color, refracted_color, World};    
    use crate::sphere::Sphere;
    use crate::Tuple;
    use crate::Shape;
    use crate::lighting::{PointLight};
    use crate::Matrix4;
    use crate::color::{BLACK, Color};
    use crate::plane::Plane;
    use crate::pattern::TestPattern;

    impl World {
        pub fn default() -> Self {
            let mut s1 = Sphere::new(1);
            s1.material.color = Color::new(0.8, 1.0, 0.6);
            s1.material.diffuse = 0.7;
            s1.material.specular = 0.2;

            let mut s2 = Sphere::new(2);
            s2.set_transformation(Matrix4::identity().scale(0.5, 0.5, 0.5));

            let light =
                PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

            World {
                objects: vec![Box::new(s1), Box::new(s2)],
                light,
            }
        }
    }

    #[test]
    fn intersect_a_world_with_ray() {        
        let world: World = World::default();
        let ray = Ray::default();
        let xs = intersect_world(&world, &ray);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }
    #[test]
    fn shading_an_interection() {
        let world: World = World::default();
        let ray = Ray::default();
        let i = Intersection {
            obj: world.objects[0].as_ref(),
            t: 4.0,
        };

        let comps = prepare_computations(&i, &ray, &vec![]);
        let color = shade_hit(&world, &comps, 0);
        assert_eq!(color, Color::new(0.38066, 0.47583, 0.2855));
    }
    #[test]
    fn shading_an_interection_from_the_inside() {
        let mut world: World = World::default();
        world.light.position = Tuple::point(0.0, 0.25, 0.0);
        let mut ray = Ray::default();
        ray.origin = Tuple::point(0.0, 0.0, 0.0);
        let i = Intersection {
            obj: world.objects[1].as_ref(),
            t: 0.5,
        };

        let comps = prepare_computations(&i, &ray, &vec![]);
        let color = shade_hit(&world, &comps, 0);
        assert_eq!(color, Color::new(0.90498, 0.90498, 0.90498));
    }
    #[test]
    fn the_color_when_a_ray_misses() {
        let world: World = World::default();
        let mut ray = Ray::default();
        ray.direction = Tuple::vector(0.0, 1.0, 0.0);
        let color = color_at(&world, &ray, 0);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn the_color_when_a_ray_hits() {
        let world: World = World::default();
        let ray = Ray::default();
        let color = color_at(&world, &ray, 0);
        assert_eq!(color, Color::new(0.38066, 0.47583, 0.2855));
    }
    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let world: World = World::default();
        let point = Tuple::point(0.0, 10.0, 0.0);
        assert_eq!(is_shadowed(&world, &point), false);
    }
    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let world: World = World::default();
        let point = Tuple::point(10.0, -10.0, 10.0);
        assert_eq!(is_shadowed(&world, &point), true);
    }
    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let world: World = World::default();
        let point = Tuple::point(-20.0, 20.0, -20.0);
        assert_eq!(is_shadowed(&world, &point), false);
    }
    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let world: World = World::default();
        let point = Tuple::point(-2.0, 2.0, -2.0);
        assert_eq!(is_shadowed(&world, &point), false);
    }
    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let light = PointLight::new(
            Tuple::point(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );

        let mut w = World::new(light);

        let s1 = Box::new(Sphere::new(1));
        let mut s2 = Box::new(Sphere::new(2));
        s2.set_transformation(Matrix4::identity().translate(0.0, 0.0, 10.0));
        w.objects.push(s1);
        w.objects.push(s2);
        let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let i = Intersection {
            obj: w.objects[1].as_ref(),
            t: 4.0
        };
        let comps = prepare_computations(&i, &ray, &vec!());
        let color = shade_hit(&w, &comps, 0);
        assert_eq!(color, Color::new(0.1, 0.1, 0.1));
    }
    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let mut world: World = World::default();
        world.objects[1].as_mut().get_mut_material().ambient = 1.0;
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let intersection =  Intersection {
            obj: world.objects[1].as_ref(),
            t: 1.0
        };
        let comps = prepare_computations(&intersection, &ray, &vec!());
        let color = reflected_color(&world, &comps, 1);
        assert_eq!(color, BLACK);
    }
    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut world: World = World::default();
        world.objects[1].as_mut().get_mut_material().ambient = 1.0;
        let mut plane = Plane::new(3);
        plane.material.reflective = 0.5;
        plane.set_transformation(Matrix4::identity().translate(0.0, -1.0, 0.0));
        world.objects.push(Box::new(plane));
        let ray = Ray::new(Tuple::point(0.0, 0.0, -3.0), Tuple::vector(0.0, -(2.0_f32.sqrt()/2f32), 2.0_f32.sqrt()/2f32));
        let intersection =  Intersection {
            obj: world.objects[2].as_ref(),
            t: 2.0_f32.sqrt()
        };
        let comps = prepare_computations(&intersection, &ray, &vec!());
        let color = reflected_color(&world, &comps, 1);
        assert_eq!(color, Color::new(0.19050309, 0.23812884, 0.14287731));
    }
    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut world: World = World::default();
        world.objects[1].as_mut().get_mut_material().ambient = 1.0;
        let mut plane = Plane::new(3);
        plane.material.reflective = 0.5;
        plane.set_transformation(Matrix4::identity().translate(0.0, -1.0, 0.0));
        world.objects.push(Box::new(plane));
        let ray = Ray::new(Tuple::point(0.0, 0.0, -3.0), Tuple::vector(0.0, -(2.0_f32.sqrt()/2f32), 2.0_f32.sqrt()/2f32));
        let intersection =  Intersection {
            obj: world.objects[2].as_ref(),
            t: 2.0_f32.sqrt()
        };
        let comps = prepare_computations(&intersection, &ray, &vec!());
        let color = shade_hit(&world, &comps, 1);
        assert_eq!(color, Color::new(0.87692857, 0.9245543, 0.8293028));
    }
    #[test]
    fn color_at_with_mutually_reflective_surfaces_completes() {
        let light = PointLight::new(Tuple::point(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0));
        let mut world = World::new(light);
        let mut lower = Plane::new(3);
        lower.material.reflective = 1.0;
        lower.set_transformation(Matrix4::identity().translate(0.0, -1.0, 0.0));

        let mut upper = Plane::new(4);
        upper.material.reflective = 1.0;
        upper.set_transformation(Matrix4::identity().translate(0.0, 1.0, 0.0));
        world.objects.push(Box::new(lower));
        world.objects.push(Box::new(upper));
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        color_at(&world, &ray, 5);
    }
    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut world: World = World::default();
        world.objects[1].as_mut().get_mut_material().ambient = 1.0;
        let mut plane = Plane::new(3);
        plane.material.reflective = 0.5;
        plane.set_transformation(Matrix4::identity().translate(0.0, -1.0, 0.0));
        world.objects.push(Box::new(plane));
        let ray = Ray::new(Tuple::point(0.0, 0.0, -3.0), Tuple::vector(0.0, -(2_f32.sqrt()/2f32), 2_f32.sqrt()/2f32));
        let intersection =  Intersection {
            obj: world.objects[2].as_ref(),
            t: 2.0_f32.sqrt()
        };
        let comps = prepare_computations(&intersection, &ray, &vec!());
        let color = reflected_color(&world, &comps, 0);
        assert_eq!(color, BLACK);
    }
    #[test]
    fn a_refracted_color_with_an_opaque_surface() {
        let world: World = World::default();        
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs : Vec<Intersection> = vec!(
            Intersection { t: 4.0, obj: world.objects[0].as_ref()},
            Intersection { t: 6.0, obj: world.objects[0].as_ref()},
        );
        let comps = prepare_computations(&xs[0], &ray, &xs);
        assert_eq!(refracted_color(&world, &comps, 5), BLACK);
    }
    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let world: World = World::default();        
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs : Vec<Intersection> = vec!(
            Intersection { t: 4.0, obj: world.objects[0].as_ref()},
            Intersection { t: 6.0, obj: world.objects[0].as_ref()},
        );
        let comps = prepare_computations(&xs[0], &ray, &xs);
        assert_eq!(refracted_color(&world, &comps, 0), BLACK);
    }
    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let mut world: World = World::default();
        world.objects[0].as_mut().get_mut_material().transparency = 1.0;
        world.objects[0].as_mut().get_mut_material().refractive_index = 1_5f32;
        
        let ray = Ray::new(Tuple::point(0.0, 0.0, 2_f32.sqrt()/2.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs : Vec<Intersection> = vec!(
            Intersection { t: -(2_f32.sqrt()/2.0), obj: world.objects[0].as_ref()},
            Intersection { t: (2_f32.sqrt()/2.0), obj: world.objects[0].as_ref()},
        );
        let comps = prepare_computations(&xs[1], &ray, &xs);
        assert_eq!(refracted_color(&world, &comps, 5), BLACK);
    }
    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let mut world: World = World::default();
        world.objects[0].as_mut().get_mut_material().ambient = 1.0;
        world.objects[0].as_mut().get_mut_material().pattern = Some(Box::new(TestPattern::new()));
        world.objects[1].as_mut().get_mut_material().transparency = 1.0;
        world.objects[1].as_mut().get_mut_material().refractive_index = 1.5;
        
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.1), Tuple::vector(0.0, 1.0, 0.0));
        let xs : Vec<Intersection> = vec!(
            Intersection { t: -0.9899, obj: world.objects[0].as_ref()},
            Intersection { t: -0.4899, obj: world.objects[1].as_ref()},
            Intersection { t:  0.4899, obj: world.objects[1].as_ref()},
            Intersection { t: 0.9899, obj: world.objects[0].as_ref()},
        );
        let comps = prepare_computations(&xs[2], &ray, &xs);
        assert_eq!(refracted_color(&world, &comps, 5), Color::new(0.0, 0.9978715, 0.047472));
    }
    #[test]
    fn shade_hit_with_a_transparent_material() {        
        let mut world: World = World::default();
        let mut floor = Plane::new(3);
        floor.set_transformation(Matrix4::identity().translate(0.0, -1.0, 0.0));
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        world.objects.push(Box::new(floor));
        let mut ball = Sphere::new(4);
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.set_transformation(Matrix4::identity().translate(0.0, -3.5, -0.5));
        world.objects.push(Box::new(ball));
        let ray = Ray::new(Tuple::point(0.0, 0.0, -3.0), Tuple::vector(0.0, -(2_f32.sqrt()/2.0), 2_f32.sqrt()/2.0));
        let xs : Vec<Intersection> = vec!(
            Intersection { t: 2_f32.sqrt(), obj: world.objects[2].as_ref()},
        );
        let comps = prepare_computations(&xs[0], &ray, &xs);
        let color = shade_hit(&world, &comps, 5);
        assert_eq!(color, Color::new(0.93642, 0.68642, 0.68642));
    }
    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {        
        let mut world: World = World::default();
        let mut floor = Plane::new(3);
        floor.set_transformation(Matrix4::identity().translate(0.0, -1.0, 0.0));
        floor.material.transparency = 0.5;
        floor.material.reflective = 0.5;
        floor.material.refractive_index = 1.5;
        world.objects.push(Box::new(floor));
        let mut ball = Sphere::new(4);
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.set_transformation(Matrix4::identity().translate(0.0, -3.5, -0.5));
        world.objects.push(Box::new(ball));
        let ray = Ray::new(Tuple::point(0.0, 0.0, -3.0), Tuple::vector(0.0, -(2_f32.sqrt()/2.0), 2_f32.sqrt()/2.0));
        let xs : Vec<Intersection> = vec!(
            Intersection { t: 2_f32.sqrt(), obj: world.objects[2].as_ref()},
        );
        let comps = prepare_computations(&xs[0], &ray, &xs);
        let color = shade_hit(&world, &comps, 5);
        assert_eq!(color, Color::new(0.93391, 0.69643, 0.69243));
    }
}
