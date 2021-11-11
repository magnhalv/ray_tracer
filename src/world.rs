use crate::ray::Intersection;
use crate::ray::{Ray, intersects};
use crate::Matrix4;
use crate::Tuple;
use crate::Color;
use crate::PointLight;
use crate::sphere::Sphere;

pub struct World {
    objects: Vec<Sphere>,
    light: PointLight
}

impl World {
    pub fn default() -> World {
        let light = PointLight::new(Tuple::point(-10_f32, 10_f32, -10_f32), Color::new(1_f32, 1_f32, 1_f32));
        
        let mut s1 = Sphere::new(1);        
        s1.material.color = Color::new(0.8_f32, 1_f32, 0.6_f32);
        s1.material.diffuse = 0.7_f32;
        s1.material.specular = 0.2_f32;

        let mut s2 = Sphere::new(2);
        s2.set_transformation(Matrix4::identity().scale(0.5_f32, 0.5_f32, 0.5_f32));
        
        World {
            objects: vec![s1, s2],
            light
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




