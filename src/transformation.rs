use crate::geometry::{Tuple};
use crate::matrix::{Matrix4, inverse4};
use std::f32::consts::{PI};

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