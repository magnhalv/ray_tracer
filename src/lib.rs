#[cfg(test)]
mod tests {

    use std::cmp::PartialEq;
    use std::ops::{Neg, Sub};

    fn is_approx_equal(lhs: f64, rhs: f64) -> bool {
        (lhs - rhs).abs() < 0.00001_f64
    }

    #[derive(Debug)]
    struct Vector {
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    }

    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    }
    fn gen_vector(x: f64, y: f64, z: f64) -> Vector {
        Vector {
            x,
            y,
            z,
            w: 0.0_f64,
        }
    }

    fn gen_point(x: f64, y: f64, z: f64, w: f64) -> Point {
        Point { x, y, z, w }
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

    impl PartialEq for Point {
        fn eq(&self, other: &Point) -> bool {
            is_approx_equal(self.x, other.x)
                && is_approx_equal(self.y, other.y)
                && is_approx_equal(self.z, other.z)
                && is_approx_equal(self.w, other.w)
        }

        fn ne(&self, other: &Point) -> bool {
            !is_approx_equal(self.x, other.x)
                || !is_approx_equal(self.y, other.y)
                || !is_approx_equal(self.z, other.z)
                || !is_approx_equal(self.w, other.w)
        }
    }

    impl PartialEq for Vector {
        fn eq(&self, other: &Vector) -> bool {
            is_approx_equal(self.x, other.x)
                && is_approx_equal(self.y, other.y)
                && is_approx_equal(self.z, other.z)
                && is_approx_equal(self.w, other.w)
        }

        fn ne(&self, other: &Vector) -> bool {
            !is_approx_equal(self.x, other.x)
                || !is_approx_equal(self.y, other.y)
                || !is_approx_equal(self.z, other.z)
                || !is_approx_equal(self.w, other.w)
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

    #[test]
    fn sub_point_from_point_gives_vector() {
        let p1 = gen_point(10.0_f64, 9.0_f64, 8.0_f64, 7.0_f64);
        let p2 = gen_point(1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64);
        let result = p1 - p2;
        let expected = gen_vector(9.0_f64, 7.0_f64, 5.0_f64);
        assert_eq!(result, expected);
    }

    #[test]
    fn sub_vector_from_point_gives_point() {
        let point = gen_point(10.0_f64, 9.0_f64, 8.0_f64, 7.0_f64);
        let vector = gen_vector(1.0_f64, 2.0_f64, 3.0_f64);
        let result = point - vector;
        let expected = gen_point(9.0_f64, 7.0_f64, 5.0_f64, 7.0_f64);
        assert_eq!(result, expected);
    }

    #[test]
    fn sub_vector_from_vector_gives_vector() {
        let v1 = gen_vector(10.0_f64, 9.0_f64, 8.0_f64);
        let v2 = gen_vector(1.0_f64, 2.0_f64, 3.0_f64);
        let result = v1 - v2;
        let expected = gen_vector(9.0_f64, 7.0_f64, 5.0_f64);
        assert_eq!(result, expected);
    }

    #[test]
    fn negating_vector() {
        let v1 = gen_vector(-10.0_f64, 9.0_f64, 8.0_f64);
        let result = -v1;
        let expected = gen_vector(10.0_f64, -9.0_f64, -8.0_f64);
        assert_eq!(result, expected);
    }

    #[test]
    fn negating_point() {
        let point = gen_point(10.0_f64, -9.0_f64, 8.0_f64, 1.0_f64);
        let result = -point;
        let expected = gen_point(-10.0_f64, 9.0_f64, -8.0_f64, -1.0_f64);
        assert_eq!(result, expected);
    }
}
