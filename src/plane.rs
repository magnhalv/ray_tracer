use crate::math::EPSILON;
use crate::Shape;
use crate::material::Material;
use crate::matrix::inverse4;
use crate::ray::transform;
use crate::ray::Intersection;
use crate::Matrix4;
use crate::Ray;
use crate::Tuple;

pub struct Plane {
    pub id: u32,
    pub material: Material,
    pub inverse_transformation: Matrix4,
}

impl Plane {
    pub fn new(id: u32) -> Plane {
        Plane {
            id,
            material: Material::default(),
            inverse_transformation: Matrix4::identity()
        }
    }
}

impl Shape for Plane {    
    fn set_transformation(&mut self, t: Matrix4) {        
        self.inverse_transformation = inverse4(&t)
    }

    fn normal_at(&self, _: &Tuple) -> Tuple {
        //let point = &self.inverse_transformation * world_point;
        let normal = Tuple::vector(0_f32, 1_f32, 0_f32); // It's constant
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
        if ray.direction.y.abs() < EPSILON {
            return vec![];
        }
        
        let i1 = Intersection {
            obj: self,
            t: -ray.origin.y / ray.direction.y
        };
        
        vec![i1]
    }

    fn get_inverse_transformation(&self) -> &Matrix4 {
        &self.inverse_transformation
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}

#[cfg(test)]
mod tests {    
    use crate::plane::Plane;
    use crate::Shape;
    use crate::Tuple;
    use crate::Ray;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let plane = Plane::new(1);
        let n1 = plane.normal_at(&Tuple::point(0_f32, 0_f32, 0_f32));
        let n2 = plane.normal_at(&Tuple::point(10_f32, 0_f32, -10_f32));
        let n3 = plane.normal_at(&Tuple::point(-5_f32, 0_f32, 150_f32));
    
        assert_eq!(n1, Tuple::vector(0_f32, 1_f32, 0_f32));
        assert_eq!(n2, Tuple::vector(0_f32, 1_f32, 0_f32));
        assert_eq!(n3, Tuple::vector(0_f32, 1_f32, 0_f32));
    }
    
    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let plane = Plane::new(1);
        let ray = Ray::new(Tuple::point(0_f32, 10_f32, 0_f32), Tuple::vector(0_f32, 0_f32, 1_f32));
        let xs = plane.intersections_by(&ray);
        assert_eq!(xs.len(), 0);
    }
    
    #[test]
    fn intersect_with_a_coplanar_ray() {
        let plane = Plane::new(1);
        let ray = Ray::new(Tuple::point(0_f32, 0_f32, 0_f32), Tuple::vector(0_f32, 0_f32, 1_f32));
        let xs = plane.intersections_by(&ray);
        assert_eq!(xs.len(), 0);
    }
    
    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let plane = Plane::new(1);
        let ray = Ray::new(Tuple::point(0_f32, 1_f32, 0_f32), Tuple::vector(0_f32, -1_f32, 0_f32));
        let xs = plane.intersections_by(&ray);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1_f32);
    }
    
    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let plane = Plane::new(1);
        let ray = Ray::new(Tuple::point(0_f32, -1_f32, 0_f32), Tuple::vector(0_f32, 1_f32, 0_f32));
        let xs = plane.intersections_by(&ray);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1_f32);
    }    
}

