use crate::material::Material;
use crate::matrix::{inverse4, Matrix4};
use crate::ray::transform;
use crate::ray::Intersection;
use crate::shape::Shape;
use crate::Ray;
use crate::Tuple;

pub struct Sphere {
    pub id: u32,
    pub transformation: Matrix4,
    pub inverse_transformation: Matrix4,
    pub material: Material,
}

impl Sphere {
    pub fn new(id: u32) -> Sphere {
        Sphere {
            id,
            transformation: Matrix4::identity(),
            inverse_transformation: Matrix4::identity(),
            material: Material::default(),
        }
    }
}

impl Shape for Sphere {
    fn set_transformation(&mut self, t: Matrix4) {
        self.transformation = t;
        self.inverse_transformation = inverse4(&self.transformation)
    }

    fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let object_point = &self.inverse_transformation * world_point;
        let object_normal = (object_point - Tuple::point(0_f32, 0_f32, 0_f32)).normalize();
        let mut world_normal = &self.inverse_transformation.transpose() * &object_normal;
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
        // A point on a spehere exists so that
        // (x - x0)^2 + (y - y0)^2 + (z - z0)^2 = r^2
        // ||P - C||^2 = r^2
        // dot((P-C), (P-C)) = r^2
        // P = A + tB, where P is the ray
        // dot((tB + (A-C), tB + (A - C))) = r^2
        // <=>
        // t^2 dot(B, B) + 2t dot(B, A-C) + dot(A-C, A-C) - r^2 = 0
        let ray = transform(&ray, &self.inverse_transformation);
        let mut result = Vec::new();
        let shape_to_ray = ray.origin - Tuple::point(0_f32, 0_f32, 0_f32);
        let a = ray.direction.dot(&ray.direction);
        let b = 2_f32 * ray.direction.dot(&shape_to_ray);
        let c = shape_to_ray.dot(&shape_to_ray) - 1_f32;
        let discriminant = b.powf(2_f32) - 4_f32 * a * c;
        if discriminant < 0_f32 {
            return result;
        }
        let i1 = Intersection {
            obj: self,
            t: (-b - discriminant.sqrt()) / (2_f32 * a),
        };
        let i2 = Intersection {
            obj: self,
            t: (-b + discriminant.sqrt()) / (2_f32 * a),
        };
        result.push(i1);
        result.push(i2);
        result
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

    use crate::sphere::Sphere;
    use crate::Shape;
    use crate::Tuple;
    use crate::Matrix4;

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let sphere = Sphere::new(1);
        let normal = sphere.normal_at(&Tuple::point(1_f32, 0_f32, 0_f32));
        assert_eq!(normal, Tuple::vector(1_f32, 0_f32, 0_f32));
    }
    
    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let sphere = Sphere::new(1);
        let normal = sphere.normal_at(&Tuple::point(0_f32, 1_f32, 0_f32));
        assert_eq!(normal, Tuple::vector(0_f32, 1_f32, 0_f32));
    }
    
    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let sphere = Sphere::new(1);
        let normal = sphere.normal_at(&Tuple::point(0_f32, 0_f32, 1_f32));
        assert_eq!(normal, Tuple::vector(0_f32, 0_f32, 1_f32));
    }
    
    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let sphere = Sphere::new(1);
        let normal = sphere.normal_at(&Tuple::point(
            3_f32.sqrt() / 3_f32,
            3_f32.sqrt() / 3_f32,
            3_f32.sqrt() / 3_f32,
        ));
        assert_eq!(
            normal,
            Tuple::vector(
                3_f32.sqrt() / 3_f32,
                3_f32.sqrt() / 3_f32,
                3_f32.sqrt() / 3_f32
            )
        );
    }
    
    #[test]
    fn the_normal_on_a_sphere_is_normalized() {
        let sphere = Sphere::new(1);
        let normal = sphere.normal_at(&Tuple::point(
            3_f32.sqrt() / 3_f32,
            3_f32.sqrt() / 3_f32,
            3_f32.sqrt() / 3_f32,
        ));
        assert_eq!(normal, normal.normalize());
    }
    
    #[test]
    fn the_normal_on_a_sphere_translated_sphere() {
        let mut sphere = Sphere::new(1);
        sphere.set_transformation(Matrix4::identity().translate(0_f32, 1_f32, 0_f32));
        let normal = sphere.normal_at(&Tuple::point(0_f32, 1.70711_f32, -0.70711_f32));
        assert_eq!(normal, Tuple::vector(0_f32, 0.70711_f32, -0.70711_f32));
    }
    
    #[test]
    fn the_normal_on_a_sphere_transformed_sphere() {
        let mut sphere = Sphere::new(1);
        sphere.set_transformation(
            Matrix4::identity()
                .scale(1_f32, 0.5_f32, 1_f32)
                .rotate_z(core::f32::consts::PI / 5_f32),
        );
        let normal = sphere.normal_at(&Tuple::point(
            0_f32,
            2_f32.sqrt() / 2_f32,
            -2_f32.sqrt() / 2_f32,
        ));
        assert_eq!(normal, Tuple::vector(0_f32, 0.97014_f32, -0.24254_f32));
    }
    
}
