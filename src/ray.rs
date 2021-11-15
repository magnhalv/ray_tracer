use crate::tuple::Tuple;
use crate::matrix::{Matrix4, inverse4};
use crate::transformation::{{translation, scaling}};
use crate::sphere::Sphere;
use crate::math::SHADOW_EPSILON;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

pub struct Intersection<'a> {
    pub obj: &'a Sphere,
    pub t: f32,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }

    pub fn default() -> Ray {
        Ray::new(Tuple::point(0_f32, 0_f32, -5_f32), Tuple::vector(0_f32, 0_f32, 1_f32))
    }

    pub fn position(&self, t: f32) -> Tuple {
        self.origin + (t * self.direction)
    }
}


pub fn intersects<'a>(sphere: &'a Sphere, ray: &Ray) -> Vec<Intersection<'a>> {
    // A point on a spehere exists so that
    // (x - x0)^2 + (y - y0)^2 + (z - z0)^2 = r^2
    // ||P - C||^2 = r^2
    // dot((P-C), (P-C)) = r^2
    // P = A + tB, where P is the ray
    // dot((tB + (A-C), tB + (A - C))) = r^2
    // <=>
    // t^2 dot(B, B) + 2t dot(B, A-C) + dot(A-C, A-C) - r^2 = 0
    let ray = transform(&ray, &sphere.inverse_transformation);
    let mut result = Vec::new();
    let sphere_to_ray = ray.origin - Tuple::point(0_f32, 0_f32, 0_f32);
    let a = ray.direction.dot(&ray.direction);
    let b = 2_f32 * ray.direction.dot(&sphere_to_ray);
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1_f32;

    let discriminant = b.powf(2_f32) - 4_f32 * a * c;

    if discriminant < 0_f32 {
        return result;
    }
    let i1 = Intersection {
        obj: &sphere,
        t: (-b - discriminant.sqrt()) / (2_f32 * a),
    };

    let i2 = Intersection {
        obj: &sphere,
        t: (-b + discriminant.sqrt()) / (2_f32 * a),
    };
    result.push(i1);
    result.push(i2);
    result
}

pub fn hit<'a>(intersections: &'a Vec<Intersection<'a>>) -> Option<&'a Intersection<'a>> {
    intersections
        .iter()
        .filter(|i| i.t >= 0_f32)
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    
}

fn transform(ray: &Ray, transformation: &Matrix4) -> Ray {
    Ray { origin: transformation * &ray.origin, direction: transformation * &ray.direction }
}

pub struct Computation<'a> {
    pub object: &'a Sphere,
    pub t: f32,
    pub point: Tuple,
    pub over_point: Tuple, // used to avoid shadow acne
    pub eye_direction: Tuple,
    pub surface_normalv:  Tuple,    
    pub is_inside: bool
}

// TODO: Find better name for this.
 pub fn prepare_computations<'a>(i: &'a Intersection, ray: &Ray) -> Computation<'a> {
    let point = ray.position(i.t);
    let eye_direction = -ray.direction;
    
    let mut surface_normalv = i.obj.normal_at(&point);
    let mut is_inside = false;
    if eye_direction.dot(&surface_normalv) < 0_f32 {
        is_inside = true;
        surface_normalv = -surface_normalv;
    }

    let over_point = point + (SHADOW_EPSILON * surface_normalv);

    Computation {        
        object: i.obj,
        t: i.t,
        point,
        over_point,
        eye_direction,
        surface_normalv,
        is_inside
    }
} 

#[test]
fn ray_position() {
    let ray = Ray {
        origin: Tuple::point(2_f32, 3_f32, 4_f32),
        direction: Tuple::vector(1_f32, 0_f32, 0_f32),
    };

    assert_eq!(ray.position(0_f32), Tuple::point(2_f32, 3_f32, 4_f32));
    assert_eq!(ray.position(1_f32), Tuple::point(3_f32, 3_f32, 4_f32));
    assert_eq!(
        ray.position(-1_f32),
        Tuple::point(1_f32, 3_f32, 4_f32)
    );
    assert_eq!(
        ray.position(2.5_f32),
        Tuple::point(4.5_f32, 3_f32, 4_f32)
    );
}

#[test]
fn ray_intesects_a_sphere_at_two_points() {
    let ray = Ray {
        origin: Tuple::point(0_f32, 0_f32, -5_f32),
        direction: Tuple::vector(0_f32, 0_f32, 1_f32),
    };

    let sphere = Sphere::new(1);

    let xs = intersects(&sphere, &ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 4_f32);
    assert_eq!(xs[1].t, 6_f32);
}

#[test]
fn ray_intesects_a_sphere_at_a_tangent() {
    let ray = Ray {
        origin: Tuple::point(0_f32, 1_f32, -5_f32),
        direction: Tuple::vector(0_f32, 0_f32, 1_f32),
    };

    let sphere = Sphere::new(1);

    let xs = intersects(&sphere, &ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 5_f32);
    assert_eq!(xs[1].t, 5_f32);
}

#[test]
fn ray_interspects_but_misses_sphere() {
    let ray = Ray {
        origin: Tuple::point(0_f32, 2_f32, -5_f32),
        direction: Tuple::vector(0_f32, 0_f32, 1_f32),
    };

    let sphere = Sphere::new(1);

    let xs = intersects(&sphere, &ray);
    assert_eq!(xs.len(), 0);
}

#[test]
fn ray_intersects_when_it_originiates_inside_sphere() {
    let ray = Ray {
        origin: Tuple::point(0_f32, 0_f32, 0_f32),
        direction: Tuple::vector(0_f32, 0_f32, 1_f32),
    };

    let sphere = Sphere::new(1);

    let xs = intersects(&sphere, &ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -1_f32);
    assert_eq!(xs[1].t, 1_f32);
}

#[test]
fn ray_intersects_when_sphere_is_behind_ray() {
    let ray = Ray {
        origin: Tuple::point(0_f32, 0_f32, 5_f32),
        direction: Tuple::vector(0_f32, 0_f32, 1_f32),
    };

    let sphere = Sphere::new(1);

    let xs = intersects(&sphere, &ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -6_f32);
    assert_eq!(xs[1].t, -4_f32);
}

#[test]
fn hit_when_all_intersections_have_positive_t() {
    let s = Sphere::new(1);
    let i1 = Intersection {
        obj: &s,
        t: 1_f32,
    };

    let i2 = Intersection {
        obj: &s,
        t: 2_f32,
    };

    let intersections = vec![i2, i1];

    let hit = hit(&intersections);
    match hit {
        Some(h) => assert_eq!(h.t, 1_f32),
        None => panic!("Hit is none"),
    }
}

#[test]
fn hit_when_some_intersections_have_negative_t() {
    let s = Sphere::new(1);

    let intersections = vec![
        Intersection {
            obj: &s,
            t: -1_f32,
        },
        Intersection {
            obj: &s,
            t: 4_f32,
        },
        Intersection {
            obj: &s,
            t: 4_f32,
        },
        Intersection {
            obj: &s,
            t: -4_f32,
        },
    ];

    let hit = hit(&intersections);
    match hit {
        Some(h) => assert_eq!(h.t, 4_f32),
        None => panic!("Hit is none"),
    }
}

#[test]
fn hit_when_all_intersections_have_negative_t() {
    let s = Sphere::new(1);

    let intersections = vec![
        Intersection {
            obj: &s,
            t: -1_f32,
        },
        Intersection {
            obj: &s,
            t: -4_f32,
        },
    ];

    let hit = hit(&intersections);
    assert!(hit.is_none());
}

#[test]
fn translate_a_ray() {
    let ray = Ray {
        origin: Tuple::point(1_f32, 2_f32, 3_f32),
        direction: Tuple::vector(0_f32, 1_f32, 0_f32)
    };

    let m = translation(3_f32, 4_f32, 5_f32);

    let ray2 = transform(&ray, &m);
    assert_eq!(ray2.origin, Tuple::point(4_f32, 6_f32, 8_f32));
    assert_eq!(ray2.direction, Tuple::vector(0_f32, 1_f32, 0_f32));
}


#[test]
fn scale_a_ray() {
    let ray = Ray {
        origin: Tuple::point(1_f32, 2_f32, 3_f32),
        direction: Tuple::vector(0_f32, 1_f32, 0_f32)
    };

    let m = scaling(2_f32, 3_f32, 4_f32);

    let ray2 = transform(&ray, &m);
    assert_eq!(ray2.origin, Tuple::point(2_f32, 6_f32, 12_f32));
    assert_eq!(ray2.direction, Tuple::vector(0_f32, 3_f32, 0_f32));
}

#[test]
fn intersecting_a_scaled_sphere_with_a_ray() {
    let ray = Ray {
        origin: Tuple::point(0_f32, 0_f32, -5_f32),
        direction: Tuple::vector(0_f32, 0_f32, 1_f32)
    };

    let mut sphere = Sphere::new(1);
    sphere.set_transformation(scaling(2_f32, 2_f32, 2_f32));
    let xs = intersects(&sphere, &ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 3_f32);
    assert_eq!(xs[1].t, 7_f32);
}


#[test]
fn precomputing_the_state_of_an_intersection() {
    let t = 4_f32;
    let obj_id = 1;
    let ray = Ray::default();
    let sphere = Sphere::new(obj_id);
    let i = Intersection {
        t,
        obj: &sphere
    };

    let comps = prepare_computations(&i, &ray);
    assert_eq!(comps.t, t);
    assert_eq!(comps.object.id, obj_id);
    assert_eq!(comps.point, Tuple::point(0_f32, 0_f32, -1_f32));
    assert_eq!(comps.eye_direction, Tuple::vector(0_f32, 0_f32, -1_f32));
    assert_eq!(comps.surface_normalv, Tuple::vector(0_f32, 0_f32, -1_f32)); 
}

#[test]
fn the_hit_when_an_interection_occurs_on_the_outside() {
    let ray = Ray::default();
    let sphere = Sphere::new(1);
    let i = Intersection {
        t: 4_f32,
        obj: &sphere
    };
    let comps = prepare_computations(&i, &ray);
    assert_eq!(comps.is_inside, false)
}

#[test]
fn the_hit_when_an_interection_occurs_on_the_inside() {
    let ray = Ray::new(Tuple::point(0_f32, 0_f32, 0_f32), Tuple::vector(0_f32, 0_f32, 1_f32));
    let sphere = Sphere::new(1);
    let i = Intersection {
        t: 1_f32,
        obj: &sphere
    };
    let comps = prepare_computations(&i, &ray);
    assert_eq!(comps.point, Tuple::point(0_f32, 0_f32, 1_f32));
    assert_eq!(comps.eye_direction, Tuple::vector(0_f32, 0_f32, -1_f32));    
    assert_eq!(comps.is_inside, true);
    assert_eq!(comps.surface_normalv, Tuple::vector(0_f32, 0_f32, -1_f32)); 
}

#[test]
fn the_hit_should_offset_the_point_to_avoid_acne() {
    let ray = Ray::new(Tuple::point(0_f32, 0_f32, -5_f32), Tuple::vector(0_f32, 0_f32, 1_f32));
    let mut sphere = Sphere::new(1);
    sphere.set_transformation(Matrix4::identity().translate(0_f32, 0_f32, 1_f32));
    
    let i = Intersection {
        t: 5_f32,
        obj: &sphere
    };
    
    let comps = prepare_computations(&i, &ray);
    assert!(comps.over_point.z < -SHADOW_EPSILON/2_f32);
    assert!(comps.point.z > comps.over_point.z);
}