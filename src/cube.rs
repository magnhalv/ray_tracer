use crate::math::{EPSILON, INFINITY};
use crate::Shape;
use crate::material::Material;
use crate::matrix::inverse4;
use crate::ray::Intersection;
use crate::Matrix4;
use crate::Ray;
use crate::Tuple;
use crate::ray::transform;

pub struct Cube {
    pub id: u32,
    pub material: Material,
    pub inverse_transformation: Matrix4,
}

impl Cube {
    pub fn new(id: u32) -> Cube {
        Cube {
            id,
            material: Material::default(),
            inverse_transformation: Matrix4::identity()
        }
    }
}

impl Shape for Cube {    
    fn set_transformation(&mut self, t: Matrix4) {        
        self.inverse_transformation = inverse4(&t)
    }

    fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let point = &self.inverse_transformation * world_point;
        let max = f32::max(f32::max(point.x.abs(), point.y.abs()), point.z.abs());
        
        let normal: Tuple;
        if max == point.x.abs() {
            normal = Tuple::vector(point.x, 0.0, 0.0);
        }
        else if max == point.y.abs() {
            normal = Tuple::vector(0.0, point.y, 0.0);
        }
        else {
            normal = Tuple::vector(0.0, 0.0, point.z);
        }
        
        let mut world_normal = &self.inverse_transformation.transpose() * &normal;
        world_normal.w = 0_f32; // Hack. Should actually find submatrix 3x3, and multiply with the inverse of that, to avoid messing with w. But this is fine and faster.
        world_normal.normalize()
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_mut_material(&mut self) -> &mut Material {
        return &mut self.material;
    }

    fn intersections_by<'a>(&'a self, ray: &Ray) -> Vec<Intersection<'a>> {
        let ray = transform(&ray, &self.inverse_transformation);
        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z);

        let tmin = f32::max(f32::max(xtmin, ytmin), ztmin);
        let tmax = f32::min(f32::min(xtmax, ytmax), ztmax);

        if tmin > tmax {
            return vec![]
        }

        let i_min = Intersection {
            obj: self,
            t: tmin
        };
        let i_max = Intersection {
            obj: self,
            t: tmax
        };        

        vec![i_min, i_max]
    }

    fn get_inverse_transformation(&self) -> &Matrix4 {
        &self.inverse_transformation
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}

fn check_axis(origin: f32, direction: f32) -> (f32, f32) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let mut tmin;
    let mut tmax;
    if direction.abs() >= EPSILON {
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    }
    else {
        tmin = tmin_numerator * INFINITY;
        tmax = tmax_numerator * INFINITY;
    }

    if tmin > tmax {
        let temp = tmin;
        tmin = tmax;
        tmax = temp;
    }

    (tmin, tmax)
}

#[cfg(test)]
mod tests {

    use crate::cube::Cube;
    use crate::Shape;
    use crate::tuple::Tuple;
    use crate::Ray;    

    #[test]
    fn a_ray_intersects_a_cube() {
        let test_cases = vec![
            (Tuple::point(5.0, 0.5, 0.0), Tuple::vector(-1.0, 0.0, 0.0), 4.0, 6.0),
            (Tuple::point(-5.0, 0.5, 0.0), Tuple::vector(1.0, 0.0, 0.0), 4.0, 6.0),
            (Tuple::point(0.5, 5.0, 0.0), Tuple::vector(0.0, -1.0, 0.0), 4.0, 6.0),
            (Tuple::point(0.5, -5.0, 0.0), Tuple::vector(0.0, 1.0, 0.0), 4.0, 6.0),
            (Tuple::point(0.5, 0.0, 5.0), Tuple::vector(0.0, 0.0, -1.0), 4.0, 6.0),
            (Tuple::point(0.5, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0), 4.0, 6.0),
            (Tuple::point(0.0, 0.5, 0.0), Tuple::vector(0.0, 0.0, 1.0), -1.0, 1.0),
        ];
        let cube = Cube::new(1);

        for test_case in test_cases {
            let ray = Ray::new(test_case.0, test_case.1);
            let xs = cube.intersections_by(&ray);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, test_case.2);
            assert_eq!(xs[1].t, test_case.3);
        }
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let test_cases = vec![
            (Tuple::point(-2.0, 0.0, 0.0), Tuple::vector(0.2673, 0.5345, 0.8018)),
            (Tuple::point(0.0, -2.0, 0.0), Tuple::vector(0.8018, 0.2673, 0.5345)),
            (Tuple::point(0.0, 0.0, -2.0), Tuple::vector(0.5345, 0.8018, 0.2673)),
            (Tuple::point(2.0, 0.0, 2.0), Tuple::vector(0.0, 0.0, -1.0)),
            (Tuple::point(0.0, 2.0, 2.0), Tuple::vector(0.0, -1.0, 0.0)),
            (Tuple::point(2.0, 2.0, 0.0), Tuple::vector(-1.0, 0.0, 0.0))
        ];
        let cube = Cube::new(1);

        for test_case in test_cases {
            let ray = Ray::new(test_case.0, test_case.1);
            let xs = cube.intersections_by(&ray);
            assert_eq!(xs.len(), 0);
        }
    }


    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let test_cases = vec![
            (Tuple::point(1.0, 0.5, -0.8), Tuple::vector(1.0, 0.0, 0.0)),
            (Tuple::point(-1.0, -0.2, 0.9), Tuple::vector(-1.0, 0.0, 0.0)),
            (Tuple::point(-0.4, 1.0, -0.1), Tuple::vector(0.0, 1.0, 0.0)),
            (Tuple::point(0.3, -1.0, -0.7), Tuple::vector(0.0, -1.0, 0.0)),
            (Tuple::point(-0.6, 0.3, 1.0), Tuple::vector(0.0, 0.0, 1.0)),
            (Tuple::point(0.4, 0.4, -1.0), Tuple::vector(0.0, 0.0, -1.0)),
            (Tuple::point(1.0, 1.0, 1.0), Tuple::vector(1.0, 0.0, 0.0)),
            (Tuple::point(-1.0, -1.0, -1.0), Tuple::vector(-1.0, 0.0, 0.0))
        ];
        let cube = Cube::new(1);

        for test_case in test_cases {
            let normal = cube.normal_at(&test_case.0);            
            assert_eq!(normal, test_case.1, "Point: {}", test_case.0);
        }
    }
}