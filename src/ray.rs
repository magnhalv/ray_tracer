use crate::geometry::Tuple;
use crate::matrix::{Matrix4, inverse4};
use crate::transformation::{{translation, scaling}};

pub struct Ray {
    origin: Tuple,
    direction: Tuple,
}

pub struct Sphere {
    pub id: u32,
    pub transformation: Matrix4
}

impl Sphere {
    pub fn new(id: u32) -> Sphere {
        Sphere { id, transformation: Matrix4::identity() }
    }
}

pub struct Intersection {
    obj_id: u32,
    t: f32,
}

pub fn position(ray: &Ray, t: f32) -> Tuple {
    ray.origin + (t * ray.direction)
}

pub fn intersects(sphere: &Sphere, ray: &Ray) -> Vec<Intersection> {
    // A point on a spehere exists so that
    // (x - x0)^2 + (y - y0)^2 + (z - z0)^2 = r^2
    // ||P - C||^2 = r^2
    // dot((P-C), (P-C)) = r^2
    // P = A + tB, where P is the ray
    // dot((tB + (A-C), tB + (A - C))) = r^2
    // <=>
    // t^2 dot(B, B) + 2t dot(B, A-C) + dot(A-C, A-C) - r^2 = 0
    let ray = transform(&ray, &inverse4(&sphere.transformation));
    let mut result = Vec::new();
    let sphere_to_ray = ray.origin - Tuple::new_point(0_f32, 0_f32, 0_f32);
    let a = ray.direction.dot(ray.direction);
    let b = 2_f32 * ray.direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1_f32;

    let discriminant = b.powf(2_f32) - 4_f32 * a * c;

    if discriminant < 0_f32 {
        return result;
    }
    let i1 = Intersection {
        obj_id: sphere.id,
        t: (-b - discriminant.sqrt()) / (2_f32 * a),
    };

    let i2 = Intersection {
        obj_id: sphere.id,
        t: (-b + discriminant.sqrt()) / (2_f32 * a),
    };
    result.push(i1);
    result.push(i2);
    result
}

fn hit(intersections: &Vec<Intersection>) -> Option<&Intersection> {
    intersections
        .iter()
        .filter(|i| i.t >= 0_f32)
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
}

fn transform(ray: &Ray, transformation: &Matrix4) -> Ray {
    Ray { origin: transformation * &ray.origin, direction: transformation * &ray.direction }
}

#[test]
fn ray_position() {
    let ray = Ray {
        origin: Tuple::new_point(2_f32, 3_f32, 4_f32),
        direction: Tuple::new_vector(1_f32, 0_f32, 0_f32),
    };

    assert_eq!(position(&ray, 0_f32), Tuple::new_point(2_f32, 3_f32, 4_f32));
    assert_eq!(position(&ray, 1_f32), Tuple::new_point(3_f32, 3_f32, 4_f32));
    assert_eq!(
        position(&ray, -1_f32),
        Tuple::new_point(1_f32, 3_f32, 4_f32)
    );
    assert_eq!(
        position(&ray, 2.5_f32),
        Tuple::new_point(4.5_f32, 3_f32, 4_f32)
    );
}

#[test]
fn ray_intesects_a_sphere_at_two_points() {
    let ray = Ray {
        origin: Tuple::new_point(0_f32, 0_f32, -5_f32),
        direction: Tuple::new_vector(0_f32, 0_f32, 1_f32),
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
        origin: Tuple::new_point(0_f32, 1_f32, -5_f32),
        direction: Tuple::new_vector(0_f32, 0_f32, 1_f32),
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
        origin: Tuple::new_point(0_f32, 2_f32, -5_f32),
        direction: Tuple::new_vector(0_f32, 0_f32, 1_f32),
    };

    let sphere = Sphere::new(1);

    let xs = intersects(&sphere, &ray);
    assert_eq!(xs.len(), 0);
}

#[test]
fn ray_intersects_when_it_originiates_inside_sphere() {
    let ray = Ray {
        origin: Tuple::new_point(0_f32, 0_f32, 0_f32),
        direction: Tuple::new_vector(0_f32, 0_f32, 1_f32),
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
        origin: Tuple::new_point(0_f32, 0_f32, 5_f32),
        direction: Tuple::new_vector(0_f32, 0_f32, 1_f32),
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
        obj_id: s.id,
        t: 1_f32,
    };

    let i2 = Intersection {
        obj_id: s.id,
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
            obj_id: s.id,
            t: -1_f32,
        },
        Intersection {
            obj_id: s.id,
            t: 4_f32,
        },
        Intersection {
            obj_id: s.id,
            t: 4_f32,
        },
        Intersection {
            obj_id: s.id,
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
            obj_id: s.id,
            t: -1_f32,
        },
        Intersection {
            obj_id: s.id,
            t: -4_f32,
        },
    ];

    let hit = hit(&intersections);
    assert!(hit.is_none());
}

#[test]
fn translate_a_ray() {
    let ray = Ray {
        origin: Tuple::new_point(1_f32, 2_f32, 3_f32),
        direction: Tuple::new_vector(0_f32, 1_f32, 0_f32)
    };

    let m = translation(3_f32, 4_f32, 5_f32);

    let ray2 = transform(&ray, &m);
    assert_eq!(ray2.origin, Tuple::new_point(4_f32, 6_f32, 8_f32));
    assert_eq!(ray2.direction, Tuple::new_vector(0_f32, 1_f32, 0_f32));
}


#[test]
fn scale_a_ray() {
    let ray = Ray {
        origin: Tuple::new_point(1_f32, 2_f32, 3_f32),
        direction: Tuple::new_vector(0_f32, 1_f32, 0_f32)
    };

    let m = scaling(2_f32, 3_f32, 4_f32);

    let ray2 = transform(&ray, &m);
    assert_eq!(ray2.origin, Tuple::new_point(2_f32, 6_f32, 12_f32));
    assert_eq!(ray2.direction, Tuple::new_vector(0_f32, 3_f32, 0_f32));
}

#[test]
fn intersecting_a_scaled_sphere_with_a_ray() {
    let ray = Ray {
        origin: Tuple::new_point(0_f32, 0_f32, -5_f32),
        direction: Tuple::new_vector(0_f32, 0_f32, 1_f32)
    };

    let mut sphere = Sphere::new(1);
    sphere.transformation = scaling(2_f32, 2_f32, 2_f32);
    let xs = intersects(&sphere, &ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 3_f32);
    assert_eq!(xs[1].t, 7_f32);
}

