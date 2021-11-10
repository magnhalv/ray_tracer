use core::f32::consts::PI;
use crate::Tuple;
use crate::matrix::{{Matrix4, inverse4}};

pub struct Sphere {
    pub id: u32,
    pub transformation: Matrix4,
    pub inverse_transformation: Matrix4
}

impl Sphere {
    pub fn new(id: u32) -> Sphere {
        Sphere { id, transformation: Matrix4::identity(), inverse_transformation: Matrix4::identity() }
    }

    pub fn set_transformation(&mut self, t: Matrix4) {
        self.transformation = t;
        self.inverse_transformation = inverse4(&self.transformation)
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = &self.inverse_transformation * &world_point;
        let object_normal = (object_point - Tuple::new_point(0_f32, 0_f32, 0_f32)).normalize();        
        let mut world_normal = &self.inverse_transformation.transpose() * &object_normal;
        world_normal.w = 0_f32; // Hack. Should actually find submatrix 3x3, and multiply with the inverse of that, to avoid messing with w. But this is fine and faster.
        world_normal.normalize()
    }
}

#[test]
fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
    let sphere = Sphere::new(1);
    let normal = sphere.normal_at(Tuple::new_point(1_f32, 0_f32, 0_f32));
    assert_eq!(normal, Tuple::new_vector(1_f32, 0_f32, 0_f32));
}

#[test]
fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
    let sphere = Sphere::new(1);
    let normal = sphere.normal_at(Tuple::new_point(0_f32, 1_f32, 0_f32));
    assert_eq!(normal, Tuple::new_vector(0_f32, 1_f32, 0_f32));
}

#[test]
fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
    let sphere = Sphere::new(1);
    let normal = sphere.normal_at(Tuple::new_point(0_f32, 0_f32, 1_f32));
    assert_eq!(normal, Tuple::new_vector(0_f32, 0_f32, 1_f32));
}

#[test]
fn the_normal_on_a_sphere_at_a_nonaxial_point() {
    let sphere = Sphere::new(1);
    let normal = sphere.normal_at(Tuple::new_point(3_f32.sqrt()/3_f32, 3_f32.sqrt()/3_f32, 3_f32.sqrt()/3_f32));
    assert_eq!(normal, Tuple::new_vector(3_f32.sqrt()/3_f32, 3_f32.sqrt()/3_f32, 3_f32.sqrt()/3_f32));
}

#[test]
fn the_normal_on_a_sphere_is_normalized() {
    let sphere = Sphere::new(1);
    let normal = sphere.normal_at(Tuple::new_point(3_f32.sqrt()/3_f32, 3_f32.sqrt()/3_f32, 3_f32.sqrt()/3_f32));
    assert_eq!(normal, normal.normalize());
}

#[test]
fn the_normal_on_a_sphere_translated_sphere() {
    let mut sphere = Sphere::new(1);
    sphere.set_transformation(Matrix4::identity().translate(0_f32, 1_f32, 0_f32));
    let normal = sphere.normal_at(Tuple::new_point(0_f32, 1.70711_f32, -0.70711_f32));
    assert_eq!(normal, Tuple::new_vector(0_f32, 0.70711_f32, -0.70711_f32));
}

#[test]
fn the_normal_on_a_sphere_transformed_sphere() {
    let mut sphere = Sphere::new(1);
    sphere.set_transformation(Matrix4::identity().scale(1_f32, 0.5_f32, 1_f32).rotate_z(PI/5_f32));
    let normal = sphere.normal_at(Tuple::new_point(0_f32, 2_f32.sqrt()/2_f32, -2_f32.sqrt()/2_f32));
    assert_eq!(normal, Tuple::new_vector(0_f32, 0.97014_f32, -0.24254_f32));
}