use crate::ray::Intersection;
use crate::ray::{intersects, Ray, prepare_computations, Computation};
use crate::sphere::Sphere;
use crate::Color;
use crate::Matrix4;
use crate::PointLight;
use crate::Tuple;
use crate::lighting::{lighting};

pub struct World {
    objects: Vec<Sphere>,
    light: PointLight,
}

impl World {
    pub fn default() -> World {
        let light = PointLight::new(
            Tuple::point(-10_f32, 10_f32, -10_f32),
            Color::new(1_f32, 1_f32, 1_f32),
        );

        let mut s1 = Sphere::new(1);
        s1.material.color = Color::new(0.8_f32, 1_f32, 0.6_f32);
        s1.material.diffuse = 0.7_f32;
        s1.material.specular = 0.2_f32;

        let mut s2 = Sphere::new(2);
        s2.set_transformation(Matrix4::identity().scale(0.5_f32, 0.5_f32, 0.5_f32));
        World {
            objects: vec![s1, s2],
            light,
        }
    }
}

pub fn intersect_world<'a>(world: &'a World, ray: &Ray) -> Vec<Intersection<'a>> {
    let mut intersections: Vec<Intersection> = vec![];
    for obj in world.objects.iter() {
        intersections.extend(intersects(&obj, ray));
    }
    intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    intersections
}

pub fn shade_hit(w: &World, computation: &Computation) -> Color {
    lighting(&computation.object.material, &w.light, &computation.point, &computation.eye_direction, &computation.surface_normalv)
}

#[test]
fn intersect_a_world_with_ray() {
    let w = World::default();
    let ray = Ray::default();
    let xs = intersect_world(&w, &ray);
    assert_eq!(xs.len(), 4);
    assert_eq!(xs[0].t, 4_f32);
    assert_eq!(xs[1].t, 4.5_f32);
    assert_eq!(xs[2].t, 5.5_f32);
    assert_eq!(xs[3].t, 6_f32);
}

#[test]
fn shading_an_interection() {
    let w = World::default();
    let ray = Ray::default();
    let i = Intersection {
        obj: &w.objects[0],
        t: 4_f32,
    };

    let comps = prepare_computations(&i, &ray);
    let color = shade_hit(&w, &comps);
    assert_eq!(color, Color::new(0.38066_f32, 0.47583_f32, 0.2855_f32));
}

#[test]
fn shading_an_interection_from_the_inside() {
    let mut w = World::default();
    w.light.position = Tuple::point(0_f32, 0.25_f32, 0_f32);
    let mut ray = Ray::default();
    ray.origin = Tuple::point(0_f32, 0_f32, 0_f32);
    let i = Intersection {
        obj: &w.objects[1],
        t: 0.5_f32,
    };

    let comps = prepare_computations(&i, &ray);
    let color = shade_hit(&w, &comps);
    assert_eq!(color, Color::new(0.90498_f32, 0.90498_f32, 0.90498_f32));
}
