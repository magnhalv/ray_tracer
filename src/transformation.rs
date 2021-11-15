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


// Maybe move to camera?
pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix4 {
    let forward = (*to - *from).normalize();    
    let left = forward.cross(&up.normalize());
    let true_up = left.cross(&forward);
    let orientation = Matrix4::new(
        left.x,       left.y,       left.z,      0_f32, 
        true_up.x,    true_up.y,    true_up.z,   0_f32, 
        -forward.x,   -forward.y,   -forward.z,  0_f32,
        0_f32,        0_f32,        0_f32,       1_f32
    );

    &orientation * &translation(-from.x, -from.y, -from.z)
}

#[test]
fn translate_point() {
    let transform = translation(5_f32, -3_f32, 2_f32);
    let p = Tuple::point(-3_f32, 4_f32, 5_f32);    
    let translated_point = &transform * &p;
    assert_eq!(translated_point, Tuple::point(2_f32, 1_f32, 7_f32));

    let inverse_transform = inverse4(&transform);
    let inversed_translated_point = &inverse_transform * &translated_point;
    assert_eq!(p, inversed_translated_point);
}
 
#[test]
fn scale_point() {
    let transform = scaling(2_f32, 3_f32, 4_f32);
    let p = Tuple::point(-4_f32, 6_f32, 8_f32);
    assert_eq!(&transform * &p, Tuple::point(-8_f32, 18_f32, 32_f32))
} 

#[test]
fn scale_vector() {
    let transform = scaling(2_f32, 3_f32, 4_f32);
    let v = Tuple::vector(-4_f32, 6_f32, 8_f32);
    assert_eq!(&transform * &v, Tuple::vector(-8_f32, 18_f32, 32_f32))
} 

#[test]
fn rotation_x_axis() {
    let point = Tuple::point(0_f32, 1_f32, 0_f32);
    let half_quarter = rotation_x(PI/4_f32);
    let full_quarter = rotation_x(PI/2_f32);

    assert_eq!(&half_quarter * &point, Tuple::point(0_f32, 2_f32.sqrt()/2_f32, 2_f32.sqrt()/2_f32));
    assert_eq!(&full_quarter * &point, Tuple::point(0_f32, 0_f32, 1_f32));    

    let inverse_half_quarter = inverse4(&half_quarter);
    assert_eq!(&inverse_half_quarter * &point, Tuple::point(0_f32, 2_f32.sqrt()/2_f32, -2_f32.sqrt()/2_f32));
}

#[test]
fn rotation_y_axis() {
    let point = Tuple::point(0_f32, 0_f32, 1_f32);
    let half_quarter = rotation_y(PI/4_f32);
    let full_quarter = rotation_y(PI/2_f32);

    assert_eq!(&half_quarter * &point, Tuple::point(2_f32.sqrt()/2_f32, 0_f32, 2_f32.sqrt()/2_f32));
    assert_eq!(&full_quarter * &point, Tuple::point(1_f32, 0_f32, 0_f32));    

    let inverse_half_quarter = inverse4(&half_quarter);
    assert_eq!(&inverse_half_quarter * &point, Tuple::point(-2_f32.sqrt()/2_f32, 0_f32, 2_f32.sqrt()/2_f32));
}

#[test]
fn rotation_z_axis() {
    let point = Tuple::point(0_f32, 1_f32, 0_f32);
    let half_quarter = rotation_z(PI/4_f32);
    let full_quarter = rotation_z(PI/2_f32);

    assert_eq!(&half_quarter * &point, Tuple::point(-2_f32.sqrt()/2_f32, 2_f32.sqrt()/2_f32, 0_f32));
    assert_eq!(&full_quarter * &point, Tuple::point(-1_f32, 0_f32, 0_f32));    
}

#[test]
fn shearing_x_in_proportion_to_y() {
    let transform = shearing(1_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32);
    let point = Tuple::point(2_f32, 3_f32, 4_f32);
    assert_eq!(&transform * &point, Tuple::point(5_f32, 3_f32, 4_f32));
}

#[test]
fn shearing_x_in_proportion_to_z() {
    let transform = shearing(0_f32, 1_f32, 0_f32, 0_f32, 0_f32, 0_f32);
    let point = Tuple::point(2_f32, 3_f32, 4_f32);
    assert_eq!(&transform * &point, Tuple::point(6_f32, 3_f32, 4_f32));
}

#[test]
fn shearing_y_in_proportion_to_x() {
    let transform = shearing(0_f32, 0_f32, 1_f32, 0_f32, 0_f32, 0_f32);
    let point = Tuple::point(2_f32, 3_f32, 4_f32);
    assert_eq!(&transform * &point, Tuple::point(2_f32, 5_f32, 4_f32));
}

#[test]
fn shearing_y_in_proportion_to_z() {
    let transform = shearing(0_f32, 0_f32, 0_f32, 1_f32, 0_f32, 0_f32);
    let point = Tuple::point(2_f32, 3_f32, 4_f32);
    assert_eq!(&transform * &point, Tuple::point(2_f32, 7_f32, 4_f32));
}

#[test]
fn shearing_z_in_proportion_to_x() {
    let transform = shearing(0_f32, 0_f32, 0_f32, 0_f32, 1_f32, 0_f32);
    let point = Tuple::point(2_f32, 3_f32, 4_f32);
    assert_eq!(&transform * &point, Tuple::point(2_f32, 3_f32, 6_f32));
}


#[test]
fn shearing_z_in_proportion_to_y() {
    let transform = shearing(0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 1_f32);
    let point = Tuple::point(2_f32, 3_f32, 4_f32);
    assert_eq!(&transform * &point, Tuple::point(2_f32, 3_f32, 7_f32));
}

#[test]
fn chaining_transformations() {
    let point = Tuple::point(1_f32, 0_f32, 1_f32);
    let a = rotation_x(PI/2_f32);
    let b = scaling(5_f32, 5_f32, 5_f32);
    let c = translation(10_f32, 5_f32, 7_f32);

    let point2 = &a * &point;
    assert_eq!(point2, Tuple::point(1_f32, -1_f32, 0_f32));

    let point3 = &b * &point2;
    assert_eq!(point3, Tuple::point(5_f32, -5_f32, 0_f32));

    let point4 = &c * &point3;
    assert_eq!(point4, Tuple::point(15_f32, 0_f32, 7_f32));

    let full_transform = &(&c * &b) * &a;
    let point5 = &full_transform * &point;
    assert_eq!(point5, Tuple::point(15_f32, 0_f32, 7_f32));
} 

#[test]
fn chaining_fluent_transformations() {
    let point = Tuple::point(1_f32, 0_f32, 1_f32);
    
    let full_transform = Matrix4::identity()
        .translate(10_f32, 5_f32, 7_f32)
        .scale(5_f32, 5_f32, 5_f32)
        .rotate_x(PI/2_f32);
    let point = &full_transform * &point;
    assert_eq!(point, Tuple::point(15_f32, 0_f32, 7_f32));
} 

#[test]
fn the_view_transformation_matrix_for_the_default_orientation() {
    let from = Tuple::point(0_f32, 0_f32, 0_f32);
    let to = Tuple::point(0_f32, 0_f32, -1_f32);
    let up = Tuple::vector(0_f32, 1_f32, 0_f32);

    let t = view_transform(&from, &to, &up);
    assert_eq!(t, Matrix4::identity());
}

#[test]
fn a_view_transformation_matrix_looking_in_positive_z_direction() {
    let from = Tuple::point(0_f32, 0_f32, 0_f32);
    let to = Tuple::point(0_f32, 0_f32, 1_f32);
    let up = Tuple::vector(0_f32, 1_f32, 0_f32);

    let t = view_transform(&from, &to, &up);
    assert_eq!(t, Matrix4::identity().scale(-1_f32, 1_f32, -1_f32));
}

#[test]
fn the_view_transformation_matrix_moves_the_world() {
    let from = Tuple::point(0_f32, 0_f32, 8_f32);
    let to = Tuple::point(0_f32, 0_f32, 0_f32);
    let up = Tuple::vector(0_f32, 1_f32, 0_f32);

    let t = view_transform(&from, &to, &up);
    assert_eq!(t, Matrix4::identity().translate(0_f32, 0_f32, -8_f32));
}

#[test]
fn an_arbitrary_view_transformation() {
    let from = Tuple::point(1_f32, 3_f32, 2_f32);
    let to = Tuple::point(4_f32, -2_f32, 8_f32);
    let up = Tuple::vector(1_f32, 1_f32, 0_f32);

    let t = view_transform(&from, &to, &up);
    let expected = Matrix4::new(
        -0.50709_f32,  0.50709_f32,  0.67612_f32,   -2.36643_f32, 
        0.76772_f32,   0.60609_f32,  0.12122_f32,   -2.82843_f32, 
        -0.35857_f32,  0.59761_f32,  -0.71714_f32,  0_f32, 
        0_f32,         0_f32,        0_f32,         1_f32);
    assert_eq!(t, expected);
}