use crate::math::float_equal;
use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Index<usize> for Tuple {
    type Output = f32;
    fn index<'a>(&'a self, index: usize) -> &'a f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("unknown index on tuple: {}", index),
        }
    }
}

impl IndexMut<usize> for Tuple {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("unknown index on tuple: {}", index),
        }
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, other: f32) -> Tuple {
        Tuple {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Mul<Tuple> for f32 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        Tuple {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        float_equal(self.x, other.x) && float_equal(self.y, other.y) && float_equal(self.z, other.z) && float_equal(self.w, other.w)
    }

    fn ne(&self, other: &Tuple) -> bool {
        !float_equal(self.x, other.x) || !float_equal(self.y, other.y) || !float_equal(self.z, other.z) || !float_equal(self.w, other.w)
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub fn new_point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple { x, y, z, w: 1_f32 }
    }

    pub fn new_vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple { x, y, z, w: 0_f32 }
    }

    pub fn dot(self, rhs: &Tuple) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn mag(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }    

    pub fn normalize(self) -> Tuple {
        let mag = self.mag();
        Tuple {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: 0.0_f32,
        }
    }

    pub fn cross(self, lhs: Tuple) -> Tuple {
        Tuple {
            x: self.y * lhs.z - self.z * lhs.y,
            y: self.z * lhs.x - self.x * lhs.z,
            z: self.x * lhs.y - self.y * lhs.x,
            w: 0.0_f32,
        }
    }
}

pub fn reflect(incomming: &Tuple, normal: &Tuple) -> Tuple {    
    *incomming - *normal * 2_f32 * incomming.dot(normal)
}

#[test]
fn sub_point_from_point_gives_vector() {
    let p1 = Tuple::new_point(10.0_f32, 9.0_f32, 8.0_f32);
    let p2 = Tuple::new_point(1.0_f32, 2.0_f32, 3.0_f32);
    let result = p1 - p2;
    let expected = Tuple::new_vector(9.0_f32, 7.0_f32, 5.0_f32);
    assert_eq!(result, expected);
}

#[test]
fn sub_vector_from_point_gives_point() {
    let point = Tuple::new_point(10.0_f32, 9.0_f32, 8.0_f32);
    let vector = Tuple::new_vector(1.0_f32, 2.0_f32, 3.0_f32);
    let result = point - vector;
    let expected = Tuple::new_point(9.0_f32, 7.0_f32, 5.0_f32);
    assert_eq!(result, expected);
}

#[test]
fn add_vector_to_point_gives_point() {
    let point = Tuple::new_point(10.0_f32, 9.0_f32, 8.0_f32);
    let vector = Tuple::new_vector(1.0_f32, 2.0_f32, 3.0_f32);
    let result = point + vector;
    let expected = Tuple::new_point(11.0_f32, 11.0_f32, 11.0_f32);
    assert_eq!(result, expected);
}

#[test]
fn sub_vector_from_vector_gives_vector() {
    let v1 = Tuple::new_vector(10.0_f32, 9.0_f32, 8.0_f32);
    let v2 = Tuple::new_vector(1.0_f32, 2.0_f32, 3.0_f32);
    let result = v1 - v2;
    let expected = Tuple::new_vector(9.0_f32, 7.0_f32, 5.0_f32);
    assert_eq!(result, expected);
}

#[test]
fn add_vector_to_vector_gives_vector() {
    let v1 = Tuple::new_vector(10.0_f32, 9.0_f32, 8.0_f32);
    let v2 = Tuple::new_vector(1.0_f32, 2.0_f32, 4.0_f32);
    let result = v1 + v2;
    let expected = Tuple::new_vector(11.0_f32, 11.0_f32, 12.0_f32);
    assert_eq!(result, expected);
}

#[test]
fn negating_vector() {
    let v1 = Tuple::new_vector(-10.0_f32, 9.0_f32, 8.0_f32);
    let result = -v1;
    let expected = Tuple::new_vector(10.0_f32, -9.0_f32, -8.0_f32);
    assert_eq!(result, expected);
}

#[test]
fn negating_point() {
    let point = Tuple::new_point(10.0_f32, -9.0_f32, 8.0_f32);
    let result = -point;
    let mut expected = Tuple::new_point(-10.0_f32, 9.0_f32, -8.0_f32);
    expected.w = -expected.w;
    assert_eq!(result, expected);
}

#[test]
fn multiply_point_by_scalar() {
    let point = Tuple::new_point(10.0_f32, -9.0_f32, 8.0_f32);
    let result1 = point * 2.0_f32;
    let result2 = 2.0_f32 * point;
    let mut expected = Tuple::new_point(20.0_f32, -18.0_f32, 16.0_f32);
    expected.w = 2.0_f32;
    assert_eq!(result1, expected);
    assert_eq!(result2, expected);
}

#[test]
fn dot_product_vector() {
    let v1 = Tuple::new_vector(2.0_f32, 3.0_f32, 4.0_f32);
    let v2 = Tuple::new_vector(2.0_f32, 1.0_f32, 3.0_f32);
    assert_eq!(v1.dot(&v2), 19.0_f32);
}

#[test]
fn magnitude_vector() {
    let v1 = Tuple::new_vector(1.0_f32, 0.0_f32, 0.0_f32);
    assert_eq!(v1.mag(), 1.0_f32);

    let v2 = Tuple::new_vector(0.0_f32, 1.0_f32, 0.0_f32);
    assert_eq!(v2.mag(), 1.0_f32);

    let v2 = Tuple::new_vector(1.0_f32, 2.0_f32, 3.0_f32);
    assert_eq!(v2.mag(), 14.0_f32.sqrt());
}

#[test]
fn normalize_vector() {
    let v1 = Tuple::new_vector(1.0_f32, 0.0_f32, 0.0_f32);
    assert_eq!(v1.normalize(), Tuple::new_vector(1.0_f32, 0.0_f32, 0.0_f32));

    let v2 = Tuple::new_vector(1.0_f32, 2.0_f32, 3.0_f32);
    assert_eq!(
        v2.normalize(),
        Tuple::new_vector(
            1.0_f32 / 14.0_f32.sqrt(),
            2.0_f32 / 14.0_f32.sqrt(),
            3.0_f32 / 14.0_f32.sqrt()
        )
    );
    assert_eq!(v2.normalize().mag(), 0.99999994_f32);
}

#[test]
fn cross_product_vector() {
    let v1 = Tuple::new_vector(1.0_f32, 0.0_f32, 0.0_f32);
    let v2 = Tuple::new_vector(0.0_f32, 0.0_f32, 1.0_f32);
    assert_eq!(v2.cross(v1), Tuple::new_vector(0.0_f32, 1.0_f32, 0.0_f32));
    assert_eq!(v1.cross(v2), Tuple::new_vector(0.0_f32, -1.0_f32, 0.0_f32));
}

#[test]
fn reflect_vector_at_45deg() {
    let v = Tuple::new_vector(1_f32, -1_f32, 0_f32);
    let normal = Tuple::new_vector(0_f32, 1_f32, 0_f32);
    assert_eq!(reflect(&v, &normal), Tuple::new_vector(1_f32, 1_f32, 0_f32));
}

#[test]
fn reflect_vector_at_slanted_surface() {
    let v = Tuple::new_vector(0_f32, -1_f32, 0_f32);
    let normal = Tuple::new_vector(2_f32.sqrt()/2_f32, 2_f32.sqrt()/2_f32, 0_f32);
    assert_eq!(reflect(&v, &normal), Tuple::new_vector(1_f32, 0_f32, 0_f32));
}