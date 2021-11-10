use crate::tuple::{Tuple};
use crate::matrix::{Matrix4, inverse4};
use std::f32::consts::{PI};

impl Matrix4 {
    pub fn identity() -> Matrix4 {
        Matrix4::new(
            1_f32, 0_f32, 0_f32, 0_f32,
            0_f32, 1_f32, 0_f32, 0_f32,
            0_f32, 0_f32, 1_f32, 0_f32,
            0_f32, 0_f32, 0_f32, 1_f32,
        )   
    }

    pub fn translate(self, x: f32, y: f32, z: f32) -> Matrix4 {
        &self * &translation(x, y, z)
    }

    pub fn scale(self, x: f32, y: f32, z: f32) -> Matrix4 {
        &self * &scaling(x, y, z)
    }

    pub fn rotate_x(self, r: f32) -> Matrix4 {
        &self * &rotation_x(r)
    }

    pub fn rotate_y(self, r: f32) -> Matrix4 {
        &self * &rotation_y(r)
    }

    pub fn rotate_z(self, r: f32) -> Matrix4 {
        &self * &rotation_z(r)
    }
}

pub fn translation(x: f32, y: f32, z: f32) -> Matrix4 {
    Matrix4::new(
        1_f32, 0_f32, 0_f32, x,
        0_f32, 1_f32, 0_f32, y,
        0_f32, 0_f32, 1_f32, z,
        0_f32, 0_f32, 0_f32, 1_f32,
    )
}

pub fn scaling(x: f32, y: f32, z: f32) -> Matrix4 {
    Matrix4::new(
        x, 0_f32, 0_f32, 0_f32,
        0_f32, y, 0_f32, 0_f32,
        0_f32, 0_f32, z, 0_f32,
        0_f32, 0_f32, 0_f32, 1_f32,
    )
}

pub fn rotation_x(r: f32) -> Matrix4 {
    Matrix4::new(
        1_f32,   0_f32,    0_f32,     0_f32,
        0_f32,   r.cos(),  -r.sin(),  0_f32,
        0_f32,   r.sin(),  r.cos(),   0_f32,
        0_f32,   0_f32,    0_f32,     1_f32,
    )
}

pub fn rotation_y(r: f32) -> Matrix4 {
    Matrix4::new(
        r.cos(),  0_f32,  r.sin(), 0_f32,
        0_f32,    1_f32,  0_f32,   0_f32,
        -r.sin(), 0_f32,  r.cos(), 0_f32,
        0_f32,    0_f32,  0_f32,   1_f32,
    )
}

pub fn rotation_z(r: f32) -> Matrix4 {
    Matrix4::new(
        r.cos(),  -r.sin(), 0_f32,  0_f32,
        r.sin(),  r.cos(),  0_f32,  0_f32,
        0_f32,    0_f32,    1_f32,  0_f32,
        0_f32,    0_f32,    0_f32,  1_f32,
    )
}

pub fn shearing(x_to_y: f32, x_to_z: f32, y_to_x: f32, y_to_z: f32, z_to_x: f32, z_to_y: f32) -> Matrix4 {
    Matrix4::new(
        1_f32,    x_to_y,  x_to_z,  0_f32,
        y_to_x,   1_f32,   y_to_z,  0_f32,
        z_to_x,   z_to_y,  1_f32,   0_f32,
        0_f32,    0_f32,   0_f32,   1_f32,
    )
}

#[test]
fn translate_point() {
    let transform = translation(5_f32, -3_f32, 2_f32);
    let p = Tuple::new_point(-3_f32, 4_f32, 5_f32);    
    let translated_point = &transform * &p;
    assert_eq!(translated_point, Tuple::new_point(2_f32, 1_f32, 7_f32));

    let inverse_transform = inverse4(&transform);
    let inversed_translated_point = &inverse_transform * &translated_point;
    assert_eq!(p, inversed_translated_point);
}
 
#[test]
fn scale_point() {
    let transform = scaling(2_f32, 3_f32, 4_f32);
    let p = Tuple::new_point(-4_f32, 6_f32, 8_f32);
    assert_eq!(&transform * &p, Tuple::new_point(-8_f32, 18_f32, 32_f32))
} 

#[test]
fn scale_vector() {
    let transform = scaling(2_f32, 3_f32, 4_f32);
    let v = Tuple::new_vector(-4_f32, 6_f32, 8_f32);
    assert_eq!(&transform * &v, Tuple::new_vector(-8_f32, 18_f32, 32_f32))
} 

#[test]
fn rotation_x_axis() {
    let point = Tuple::new_point(0_f32, 1_f32, 0_f32);
    let half_quarter = rotation_x(PI/4_f32);
    let full_quarter = rotation_x(PI/2_f32);

    assert_eq!(&half_quarter * &point, Tuple::new_point(0_f32, 2_f32.sqrt()/2_f32, 2_f32.sqrt()/2_f32));
    assert_eq!(&full_quarter * &point, Tuple::new_point(0_f32, 0_f32, 1_f32));    

    let inverse_half_quarter = inverse4(&half_quarter);
    assert_eq!(&inverse_half_quarter * &point, Tuple::new_point(0_f32, 2_f32.sqrt()/2_f32, -2_f32.sqrt()/2_f32));
}

#[test]
fn rotation_y_axis() {
    let point = Tuple::new_point(0_f32, 0_f32, 1_f32);
    let half_quarter = rotation_y(PI/4_f32);
    let full_quarter = rotation_y(PI/2_f32);

    assert_eq!(&half_quarter * &point, Tuple::new_point(2_f32.sqrt()/2_f32, 0_f32, 2_f32.sqrt()/2_f32));
    assert_eq!(&full_quarter * &point, Tuple::new_point(1_f32, 0_f32, 0_f32));    

    let inverse_half_quarter = inverse4(&half_quarter);
    assert_eq!(&inverse_half_quarter * &point, Tuple::new_point(-2_f32.sqrt()/2_f32, 0_f32, 2_f32.sqrt()/2_f32));
}

#[test]
fn rotation_z_axis() {
    let point = Tuple::new_point(0_f32, 1_f32, 0_f32);
    let half_quarter = rotation_z(PI/4_f32);
    let full_quarter = rotation_z(PI/2_f32);

    assert_eq!(&half_quarter * &point, Tuple::new_point(-2_f32.sqrt()/2_f32, 2_f32.sqrt()/2_f32, 0_f32));
    assert_eq!(&full_quarter * &point, Tuple::new_point(-1_f32, 0_f32, 0_f32));    
}

#[test]
fn shearing_x_in_proportion_to_y() {
    let transform = shearing(1_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32);
    let point = Tuple::new_point(2_f32, 3_f32, 4_f32);
    assert_eq!(&transform * &point, Tuple::new_point(5_f32, 3_f32, 4_f32));
}

#[test]
fn shearing_x_in_proportion_to_z() {
    let transform = shearing(0_f32, 1_f32, 0_f32, 0_f32, 0_f32, 0_f32);
    let point = Tuple::new_point(2_f32, 3_f32, 4_f32);
    assert_eq!(&transform * &point, Tuple::new_point(6_f32, 3_f32, 4_f32));
}

#[test]
fn shearing_y_in_proportion_to_x() {
    let transform = shearing(0_f32, 0_f32, 1_f32, 0_f32, 0_f32, 0_f32);
    let point = Tuple::new_point(2_f32, 3_f32, 4_f32);
    assert_eq!(&transform * &point, Tuple::new_point(2_f32, 5_f32, 4_f32));
}

#[test]
fn shearing_y_in_proportion_to_z() {
    let transform = shearing(0_f32, 0_f32, 0_f32, 1_f32, 0_f32, 0_f32);
    let point = Tuple::new_point(2_f32, 3_f32, 4_f32);
    assert_eq!(&transform * &point, Tuple::new_point(2_f32, 7_f32, 4_f32));
}

#[test]
fn shearing_z_in_proportion_to_x() {
    let transform = shearing(0_f32, 0_f32, 0_f32, 0_f32, 1_f32, 0_f32);
    let point = Tuple::new_point(2_f32, 3_f32, 4_f32);
    assert_eq!(&transform * &point, Tuple::new_point(2_f32, 3_f32, 6_f32));
}


#[test]
fn shearing_z_in_proportion_to_y() {
    let transform = shearing(0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 1_f32);
    let point = Tuple::new_point(2_f32, 3_f32, 4_f32);
    assert_eq!(&transform * &point, Tuple::new_point(2_f32, 3_f32, 7_f32));
}

#[test]
fn chaining_transformations() {
    let point = Tuple::new_point(1_f32, 0_f32, 1_f32);
    let a = rotation_x(PI/2_f32);
    let b = scaling(5_f32, 5_f32, 5_f32);
    let c = translation(10_f32, 5_f32, 7_f32);

    let point2 = &a * &point;
    assert_eq!(point2, Tuple::new_point(1_f32, -1_f32, 0_f32));

    let point3 = &b * &point2;
    assert_eq!(point3, Tuple::new_point(5_f32, -5_f32, 0_f32));

    let point4 = &c * &point3;
    assert_eq!(point4, Tuple::new_point(15_f32, 0_f32, 7_f32));

    let full_transform = &(&c * &b) * &a;
    let point5 = &full_transform * &point;
    assert_eq!(point5, Tuple::new_point(15_f32, 0_f32, 7_f32));
} 

#[test]
fn chaining_fluent_transformations() {
    let point = Tuple::new_point(1_f32, 0_f32, 1_f32);
    
    let full_transform = Matrix4::identity()
        .translate(10_f32, 5_f32, 7_f32)
        .scale(5_f32, 5_f32, 5_f32)
        .rotate_x(PI/2_f32);
    let point = &full_transform * &point;
    assert_eq!(point, Tuple::new_point(15_f32, 0_f32, 7_f32));
} 