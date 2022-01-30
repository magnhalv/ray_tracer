use crate::math::SHADOW_EPSILON;
use crate::matrix::Matrix4;
use crate::shape::Shape;
use crate::tuple::reflect;
use crate::tuple::Tuple;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

pub struct Intersection<'a> {
    pub obj: &'a dyn Shape,
    pub t: f32,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f32) -> Tuple {
        self.origin + (t * self.direction)
    }
}

pub fn hit<'a>(intersections: &'a Vec<Intersection<'a>>) -> Option<&'a Intersection<'a>> {
    intersections
        .iter()
        .filter(|i| i.t >= 0_f32)
        .next()
}

pub fn transform(ray: &Ray, transformation: &Matrix4) -> Ray {
    Ray {
        origin: transformation * &ray.origin,
        direction: transformation * &ray.direction,
    }
}

pub struct Computation<'a> {
    pub object: &'a dyn Shape,
    pub t: f32,
    pub point: Tuple,
    pub over_point: Tuple,  // used to avoid shadow acne
    pub under_point: Tuple, // used to avoid reflection acne?
    pub eye_direction: Tuple,
    pub surface_normalv: Tuple,
    pub is_inside: bool,
    pub reflectv: Tuple,
    pub n1: f32,
    pub n2: f32,
}

// TODO: Find better name for this.
pub fn prepare_computations<'a>(
    hit: &'a Intersection,
    ray: &Ray,
    intersections: &Vec<Intersection<'a>>,
) -> Computation<'a> {
    let mut containers: Vec<&'a dyn Shape> = vec![];

    // TODO: Improve this algorithm
    let mut n1: f32 = 1.0;
    let mut n2: f32 = 1.0;
    for i in intersections.iter() {
        if std::ptr::eq(i, hit) {
            if containers.len() == 0 {
                n1 = 1_f32;
            } else {
                n1 = (*containers.last().unwrap())
                    .get_material()
                    .refractive_index;
            }
        }
        let index = containers.iter().position(|&o| std::ptr::eq(i.obj, o));
        if index.is_some() {
            containers.remove(index.unwrap());
        } else {
            containers.push(i.obj);
        }
        if std::ptr::eq(i, hit) {
            if containers.len() == 0 {
                n2 = 1_f32;
            } else {
                n2 = (*containers.last().unwrap())
                    .get_material()
                    .refractive_index;
            }
            break;
        }
    }
    let point = ray.position(hit.t);
    let eye_direction = -ray.direction;
    let mut surface_normalv = hit.obj.normal_at(&point);
    let mut is_inside = false;
    if eye_direction.dot(&surface_normalv) < 0_f32 {
        is_inside = true;
        surface_normalv = -surface_normalv;
    }

    let over_point = point + (SHADOW_EPSILON * surface_normalv);
    let under_point = point - (SHADOW_EPSILON * surface_normalv);
    let reflectv = reflect(&ray.direction, &surface_normalv);
    Computation {
        object: hit.obj,
        t: hit.t,
        point,
        over_point,
        under_point,
        eye_direction,
        surface_normalv,
        is_inside,
        reflectv,
        n1,
        n2,
    }
}

pub fn schlick<'a>(comps: &Computation<'a>) -> f32 {
    let mut cos = comps.eye_direction.dot(&comps.surface_normalv);
    if comps.n1 > comps.n2 {
        let n = comps.n1 / comps.n2;
        let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
        if sin2_t > 1.0 {
            return 1.0;
        }
        cos = (1.0 - sin2_t).sqrt();
    }
    let r0 = ((comps.n1 - comps.n2) / (comps.n1 + comps.n2)).powi(2);
    r0 + ((1.0 - r0) * ((1.0 - cos).powi(5)))
}

#[cfg(test)]
mod ray_tests {
    use crate::math::EPSILON;
    use crate::ray::hit;
    use crate::ray::prepare_computations;
    use crate::ray::schlick;
    use crate::ray::transform;
    use crate::ray::Intersection;
    use crate::ray::SHADOW_EPSILON;
    use crate::shape::Shape;
    use crate::transformation::scaling;
    use crate::transformation::translation;
    use crate::Matrix4;
    use crate::Plane;
    use crate::Ray;
    use crate::Sphere;
    use crate::Tuple;
    use crate::material::Material;

    impl Ray {
        pub fn default() -> Ray {
            Ray::new(
                Tuple::point(0_f32, 0_f32, -5_f32),
                Tuple::vector(0_f32, 0_f32, 1_f32),
            )
        }
    }

    impl Sphere {
        pub fn new_glass(id: u32) -> Sphere {
            let mut sphere = Sphere {
                id,
                transformation: Matrix4::identity(),
                inverse_transformation: Matrix4::identity(),
                material: Material::default(),
            };
            sphere.material.transparency = 1.0_f32;
            sphere.material.refractive_index = 1.5_f32;
            sphere
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
        assert_eq!(ray.position(-1_f32), Tuple::point(1_f32, 3_f32, 4_f32));
        assert_eq!(ray.position(2.5_f32), Tuple::point(4.5_f32, 3_f32, 4_f32));
    }
    #[test]
    fn ray_intesects_a_sphere_at_two_points() {
        let ray = Ray {
            origin: Tuple::point(0_f32, 0_f32, -5_f32),
            direction: Tuple::vector(0_f32, 0_f32, 1_f32),
        };
        let sphere = Sphere::new(1);
        let xs = sphere.intersections_by(&ray);
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
        let xs = sphere.intersections_by(&ray);
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
        let xs = sphere.intersections_by(&ray);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn ray_intersects_when_it_originiates_inside_sphere() {
        let ray = Ray {
            origin: Tuple::point(0_f32, 0_f32, 0_f32),
            direction: Tuple::vector(0_f32, 0_f32, 1_f32),
        };
        let sphere = Sphere::new(1);
        let xs = sphere.intersections_by(&ray);
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
        let xs = sphere.intersections_by(&ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6_f32);
        assert_eq!(xs[1].t, -4_f32);
    }
    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new(1);
        let i1 = Intersection { obj: &s, t: 1_f32 };

        let i2 = Intersection { obj: &s, t: 2_f32 };
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
            Intersection { obj: &s, t: -1_f32 },
            Intersection { obj: &s, t: 4_f32 },
            Intersection { obj: &s, t: 4_f32 },
            Intersection { obj: &s, t: -4_f32 },
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
            Intersection { obj: &s, t: -1_f32 },
            Intersection { obj: &s, t: -4_f32 },
        ];
        let hit = hit(&intersections);
        assert!(hit.is_none());
    }
    #[test]
    fn translate_a_ray() {
        let ray = Ray {
            origin: Tuple::point(1_f32, 2_f32, 3_f32),
            direction: Tuple::vector(0_f32, 1_f32, 0_f32),
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
            direction: Tuple::vector(0_f32, 1_f32, 0_f32),
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
            direction: Tuple::vector(0_f32, 0_f32, 1_f32),
        };
        let mut sphere = Sphere::new(1);
        sphere.set_transformation(scaling(2_f32, 2_f32, 2_f32));
        let xs = sphere.intersections_by(&ray);
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
        let i = Intersection { t, obj: &sphere };

        let comps = prepare_computations(&i, &ray, &vec![]);
        assert_eq!(comps.t, t);
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
            obj: &sphere,
        };
        let comps = prepare_computations(&i, &ray, &vec![]);
        assert_eq!(comps.is_inside, false)
    }
    #[test]
    fn the_hit_when_an_interection_occurs_on_the_inside() {
        let ray = Ray::new(
            Tuple::point(0_f32, 0_f32, 0_f32),
            Tuple::vector(0_f32, 0_f32, 1_f32),
        );
        let sphere = Sphere::new(1);
        let i = Intersection {
            t: 1_f32,
            obj: &sphere,
        };
        let comps = prepare_computations(&i, &ray, &vec![]);
        assert_eq!(comps.point, Tuple::point(0_f32, 0_f32, 1_f32));
        assert_eq!(comps.eye_direction, Tuple::vector(0_f32, 0_f32, -1_f32));
        assert_eq!(comps.is_inside, true);
        assert_eq!(comps.surface_normalv, Tuple::vector(0_f32, 0_f32, -1_f32));
    }
    #[test]
    fn the_hit_should_offset_the_point_to_avoid_acne() {
        let ray = Ray::new(
            Tuple::point(0_f32, 0_f32, -5_f32),
            Tuple::vector(0_f32, 0_f32, 1_f32),
        );
        let mut sphere = Sphere::new(1);
        sphere.set_transformation(Matrix4::identity().translate(0_f32, 0_f32, 1_f32));
        let i = Intersection {
            t: 5_f32,
            obj: &sphere,
        };

        let comps = prepare_computations(&i, &ray, &vec![]);
        assert!(comps.over_point.z < -SHADOW_EPSILON / 2_f32);
        assert!(comps.point.z > comps.over_point.z);
    }
    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = Plane::new(1);
        let ray = Ray::new(
            Tuple::point(0_f32, 1_f32, -1_f32),
            Tuple::vector(0_f32, -(2_f32.sqrt() / 2_f32), 2_f32.sqrt() / 2_f32),
        );
        let intersection = Intersection {
            obj: &shape,
            t: 2_f32.sqrt(),
        };
        let comps = prepare_computations(&intersection, &ray, &vec![]);
        assert_eq!(
            comps.reflectv,
            Tuple::vector(0_f32, 2_f32.sqrt() / 2_f32, 2_f32.sqrt() / 2_f32)
        );
    }
    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let scenarios: [(f32, f32); 6] = [
            (1.0_f32, 1.5_f32),
            (1.5_f32, 2.0_f32),
            (2.0_f32, 2.5_f32),
            (2.5_f32, 2.5_f32),
            (2.5_f32, 1.5_f32),
            (1.5_f32, 1.0_f32),
        ];
        let mut a = Sphere::new_glass(1);
        a.set_transformation(Matrix4::identity().scale(2_f32, 2_f32, 2_f32));
        let mut b = Sphere::new_glass(2);
        b.set_transformation(Matrix4::identity().translate(0_f32, 0_f32, -0.25_f32));
        b.material.refractive_index = 2_f32;
        let mut c = Sphere::new_glass(3);
        c.set_transformation(Matrix4::identity().translate(0_f32, 0_f32, 0.25_f32));
        c.material.refractive_index = 2.5_f32;

        let ray = Ray::new(
            Tuple::point(0_f32, 0_f32, -4_f32),
            Tuple::vector(0_f32, 0_f32, 1_f32),
        );
        let xs: Vec<Intersection> = vec![
            Intersection { t: 2.0, obj: &a },
            Intersection { t: 2.75, obj: &b },
            Intersection { t: 3.25, obj: &c },
            Intersection { t: 4.75, obj: &b },
            Intersection { t: 5.25, obj: &c },
            Intersection { t: 6_f32, obj: &a },
        ];
        for i in 0..6 {
            let comps = prepare_computations(&xs[i], &ray, &xs);
            assert_eq!(comps.n1, scenarios[i].0);
            assert_eq!(comps.n2, scenarios[i].1);
        }
    }

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let shape = Sphere::new_glass(1);
        let ray = Ray::new(
            Tuple::point(0.0, 0.0, 2_f32.sqrt() / 2.0),
            Tuple::vector(0.0, 1.0, 0.0),
        );
        let xs: Vec<Intersection> = vec![
            Intersection {
                t: -(2_f32.sqrt() / 2.0),
                obj: &shape,
            },
            Intersection {
                t: (2_f32.sqrt() / 2.0),
                obj: &shape,
            },
        ];
        let comps = prepare_computations(&xs[1], &ray, &xs);
        let reflectance = schlick(&comps);
        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let shape = Sphere::new_glass(1);
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs: Vec<Intersection> = vec![
            Intersection {
                t: -1.0,
                obj: &shape,
            },
            Intersection {
                t: 1.0,
                obj: &shape,
            },
        ];
        let comps = prepare_computations(&xs[1], &ray, &xs);
        let reflectance = schlick(&comps);
        assert!((reflectance - 0.04_f32).abs() < EPSILON);
    }

    #[test]
    fn the_schlick_approximation_with_a_small_angle_and_n2_gt_n1() {
        let shape = Sphere::new_glass(1);
        let ray = Ray::new(Tuple::point(0.0, 0.99, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs: Vec<Intersection> = vec![Intersection {
            t: 1.8589,
            obj: &shape,
        }];
        let comps = prepare_computations(&xs[0], &ray, &xs);
        let reflectance = schlick(&comps);
        assert!((reflectance - 0.48873_f32).abs() < EPSILON);
    }
}
