use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

// TODO: Simplify operations by creating a "Tuple" trait?
#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: 0.0_f64,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, other: Point) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: 0.0_f64,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, other: Vector) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, other: Vector) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w,
        }
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: 0_f64,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: 0_f64,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }

    fn ne(&self, other: &Point) -> bool {
        self.x != other.x || self.y != other.y || self.z != other.z || self.w != other.w
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }

    fn ne(&self, other: &Vector) -> bool {
        self.x != other.x || self.y != other.y || self.z != other.z || self.w != other.w
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: 0_f64,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z, w: 0_f64 }
    }
    pub fn dot(self, rhs: Vector) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn mag(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Vector {
        let mag = self.mag();
        Vector {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: 0.0_f64,
        }
    }

    pub fn cross(self, lhs: Vector) -> Vector {
        Vector {
            x: self.y * lhs.z - self.z * lhs.y,
            y: self.z * lhs.x - self.x * lhs.z,
            z: self.x * lhs.y - self.y * lhs.x,
            w: 0.0_f64,
        }
    }
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z, w: 1.0_f64 }
    }
}

#[test]
fn sub_point_from_point_gives_vector() {
    let p1 = Point::new(10.0_f64, 9.0_f64, 8.0_f64);
    let p2 = Point::new(1.0_f64, 2.0_f64, 3.0_f64);
    let result = p1 - p2;
    let expected = Vector::new(9.0_f64, 7.0_f64, 5.0_f64);
    assert_eq!(result, expected);
}

#[test]
fn sub_vector_from_point_gives_point() {
    let point = Point::new(10.0_f64, 9.0_f64, 8.0_f64);
    let vector = Vector::new(1.0_f64, 2.0_f64, 3.0_f64);
    let result = point - vector;
    let expected = Point::new(9.0_f64, 7.0_f64, 5.0_f64);
    assert_eq!(result, expected);
}

#[test]
fn add_vector_to_point_gives_point() {
    let point = Point::new(10.0_f64, 9.0_f64, 8.0_f64);
    let vector = Vector::new(1.0_f64, 2.0_f64, 3.0_f64);
    let result = point + vector;
    let expected = Point::new(11.0_f64, 11.0_f64, 11.0_f64);
    assert_eq!(result, expected);
}

#[test]
fn sub_vector_from_vector_gives_vector() {
    let v1 = Vector::new(10.0_f64, 9.0_f64, 8.0_f64);
    let v2 = Vector::new(1.0_f64, 2.0_f64, 3.0_f64);
    let result = v1 - v2;
    let expected = Vector::new(9.0_f64, 7.0_f64, 5.0_f64);
    assert_eq!(result, expected);
}

#[test]
fn add_vector_to_vector_gives_vector() {
    let v1 = Vector::new(10.0_f64, 9.0_f64, 8.0_f64);
    let v2 = Vector::new(1.0_f64, 2.0_f64, 4.0_f64);
    let result = v1 + v2;
    let expected = Vector::new(11.0_f64, 11.0_f64, 12.0_f64);
    assert_eq!(result, expected);
}

#[test]
fn negating_vector() {
    let v1 = Vector::new(-10.0_f64, 9.0_f64, 8.0_f64);
    let result = -v1;
    let expected = Vector::new(10.0_f64, -9.0_f64, -8.0_f64);
    assert_eq!(result, expected);
}

#[test]
fn negating_point() {
    let point = Point::new(10.0_f64, -9.0_f64, 8.0_f64);
    let result = -point;
    let mut expected = Point::new(-10.0_f64, 9.0_f64, -8.0_f64);
    expected.w = -expected.w;
    assert_eq!(result, expected);
}

#[test]
fn multiply_point_by_scalar() {
    let point = Point::new(10.0_f64, -9.0_f64, 8.0_f64);
    let result1 = point * 2.0_f64;
    let result2 = 2.0_f64 * point;
    let mut expected = Point::new(20.0_f64, -18.0_f64, 16.0_f64);
    expected.w = 2.0_f64;
    assert_eq!(result1, expected);
    assert_eq!(result2, expected);
}

#[test]
fn dot_product_vector() {
    let v1 = Vector::new(2.0_f64, 3.0_f64, 4.0_f64);
    let v2 = Vector::new(2.0_f64, 1.0_f64, 3.0_f64);
    assert_eq!(v1.dot(v2), 19.0_f64);
}

#[test]
fn magnitude_vector() {
    let v1 = Vector::new(1.0_f64, 0.0_f64, 0.0_f64);
    assert_eq!(v1.mag(), 1.0_f64);

    let v2 = Vector::new(0.0_f64, 1.0_f64, 0.0_f64);
    assert_eq!(v2.mag(), 1.0_f64);

    let v2 = Vector::new(1.0_f64, 2.0_f64, 3.0_f64);
    assert_eq!(v2.mag(), 14.0_f64.sqrt());
}

#[test]
fn normalize_vector() {
    let v1 = Vector::new(1.0_f64, 0.0_f64, 0.0_f64);
    assert_eq!(v1.normalize(), Vector::new(1.0_f64, 0.0_f64, 0.0_f64));

    let v2 = Vector::new(1.0_f64, 2.0_f64, 3.0_f64);
    assert_eq!(
        v2.normalize(),
        Vector::new(
            1.0_f64 / 14.0_f64.sqrt(),
            2.0_f64 / 14.0_f64.sqrt(),
            3.0_f64 / 14.0_f64.sqrt()
        )
    );
    assert_eq!(v2.normalize().mag(), 1.0_f64);
}

#[test]
fn cross_product_vector() {
    let v1 = Vector::new(1.0_f64, 0.0_f64, 0.0_f64);
    let v2 = Vector::new(0.0_f64, 0.0_f64, 1.0_f64);
    assert_eq!(v2.cross(v1), Vector::new(0.0_f64, 1.0_f64, 0.0_f64));
    assert_eq!(v1.cross(v2), Vector::new(0.0_f64, -1.0_f64, 0.0_f64));
}