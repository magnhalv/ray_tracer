use crate::matrix::inverse4;
use crate::Color;
use crate::Matrix4;
use crate::Shape;
use crate::Tuple;

pub trait Pattern: Send + Sync {
    fn color_at(&self, point: &Tuple) -> Color;
    fn color_at_obj(&self, object: &dyn Shape, world_point: &Tuple) -> Color;
    fn set_transform(&mut self, transform: &Matrix4);
}

pub struct StripePattern {
    pub first: Color,
    pub second: Color,
    pub inverse_transformation: Matrix4,
}

impl Pattern for StripePattern {
    fn color_at(&self, point: &Tuple) -> Color {
        if (point.x.floor() as i32) % 2 == 0 {
            return self.first;
        }
        self.second
    }
    fn color_at_obj(&self, object: &dyn Shape, world_point: &Tuple) -> Color {
        let obj_point = object.get_inverse_transformation() * world_point;
        let pattern_point = &self.inverse_transformation * &obj_point;
        self.color_at(&pattern_point)
    }

    fn set_transform(&mut self, transform: &Matrix4) {
        self.inverse_transformation = inverse4(transform);
    }
}

pub struct GradientPattern {
    pub first: Color,
    pub second: Color,
    pub inverse_transformation: Matrix4,
}

impl Pattern for GradientPattern {
    fn color_at(&self, point: &Tuple) -> Color {
        //println!("X: {}", point.x);
        let distance = self.second - self.first;
        let x = point.x;
        let fraction = x - x.floor();
        self.first + (distance * fraction)
    }
    fn color_at_obj(&self, object: &dyn Shape, world_point: &Tuple) -> Color {
        let obj_point = object.get_inverse_transformation() * world_point;
        let pattern_point = &self.inverse_transformation * &obj_point;

        /* unsafe {
            static mut smallest_obj_x: f32 = 1000_f32;;
            static mut bigest_obj_x: f32 = -1000_f32;;
            smallest_obj_x = f32::min(smallest_obj_x, pattern_point.x);
            bigest_obj_x = f32::max(bigest_obj_x, pattern_point.x);
            println!("Smallest x: {}. Biggest x: {}", smallest_obj_x, bigest_obj_x);
        } */

        self.color_at(&pattern_point)
    }

    fn set_transform(&mut self, transform: &Matrix4) {
        self.inverse_transformation = inverse4(transform);
    }
}

pub struct RingPattern {
    pub first: Color,
    pub second: Color,
    pub inverse_transformation: Matrix4,
}

impl Pattern for RingPattern {
    fn color_at(&self, point: &Tuple) -> Color {
        let magnitude = ((point.x.powf(2_f32) + point.z.powf(2_f32)).sqrt()).floor();
        //println!("Magnitude: {}. x: {}, z: {}", magnitude, point.x, point.z);
        if (magnitude as u32) % 2 == 0 {
            return self.first;
        }
        self.second
    }

    fn color_at_obj(&self, object: &dyn Shape, world_point: &Tuple) -> Color {
        let obj_point = object.get_inverse_transformation() * world_point;
        let pattern_point = &self.inverse_transformation * &obj_point;

        /* unsafe {
            static mut smallest_obj_x: f32 = 1000_f32;;
            static mut bigest_obj_x: f32 = -1000_f32;;
            smallest_obj_x = f32::min(smallest_obj_x, pattern_point.x);
            bigest_obj_x = f32::max(bigest_obj_x, pattern_point.x);
            println!("Smallest x: {}. Biggest x: {}", smallest_obj_x, bigest_obj_x);

            static mut smallest_obj_z: f32 = 1000_f32;;
            static mut bigest_obj_z: f32 = -1000_f32;;
            smallest_obj_z = f32::min(smallest_obj_z, pattern_point.z);
            bigest_obj_z = f32::max(bigest_obj_z, pattern_point.z);
            println!("Smallest z: {}. Biggest z: {}", smallest_obj_z, bigest_obj_z);
        } */
        self.color_at(&pattern_point)
    }

    fn set_transform(&mut self, transform: &Matrix4) {
        self.inverse_transformation = inverse4(transform);
    }
}

pub struct CheckerPattern {
    pub first: Color,
    pub second: Color,
    pub inverse_transformation: Matrix4,
}

impl Pattern for CheckerPattern {
    fn color_at(&self, point: &Tuple) -> Color {
        let magnitude = point.x.floor() + point.z.floor() + point.y.abs().floor();
        if (magnitude as i32) % 2 == 0 {
            return self.first;
        }
        self.second
    }
    fn color_at_obj(&self, object: &dyn Shape, world_point: &Tuple) -> Color {
        let obj_point = object.get_inverse_transformation() * world_point;
        let pattern_point = &self.inverse_transformation * &obj_point;
        self.color_at(&pattern_point)
    }

    fn set_transform(&mut self, transform: &Matrix4) {
        self.inverse_transformation = inverse4(transform);
    }
}

pub struct TestPattern {
    pub first: Color,
    pub second: Color,
    pub inverse_transformation: Matrix4,
}

impl Pattern for TestPattern {
    fn color_at(&self, point: &Tuple) -> Color {
        Color::new(point.x, point.y, point.z)
    }
    fn color_at_obj(&self, object: &dyn Shape, world_point: &Tuple) -> Color {
        let obj_point = object.get_inverse_transformation() * world_point;
        let pattern_point = &self.inverse_transformation * &obj_point;
        self.color_at(&pattern_point)
    }

    fn set_transform(&mut self, transform: &Matrix4) {
        self.inverse_transformation = inverse4(transform);
    }
}

#[cfg(test)]
pub mod pattern_tests {
    use crate::color::{Color, BLACK, WHITE};
    use crate::pattern::{GradientPattern, Pattern, RingPattern, StripePattern, TestPattern};
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;
    use crate::Matrix4;
    use crate::Shape;

    impl StripePattern {
        pub fn new_box(first: Color, second: Color) -> Option<Box<dyn Pattern>> {
            Some(Box::new(StripePattern {
                first,
                second,
                inverse_transformation: Matrix4::identity(),
            }))
        }
        pub fn test_default() -> StripePattern {
            StripePattern {
                first: WHITE,
                second: BLACK,
                inverse_transformation: Matrix4::identity(),
            }
        }
    }

    impl TestPattern {
        pub fn new() -> TestPattern {
            TestPattern {
                first: Color::new(0.0, 0.0, 0.0),
                second: Color::new(0.0, 0.0, 0.0),
                inverse_transformation: Matrix4::identity()
            }
        }
    }

    #[test]
    fn stripe_pattern_alternates_only_in_x() {
        let pattern = StripePattern::test_default();
        assert_eq!(pattern.color_at(&Tuple::point(0_f32, 0_f32, 0_f32)), WHITE);
        assert_eq!(pattern.color_at(&Tuple::point(1_f32, 0_f32, 0_f32)), BLACK);
        assert_eq!(pattern.color_at(&Tuple::point(2_f32, 0_f32, 0_f32)), WHITE);
        assert_eq!(pattern.color_at(&Tuple::point(0_f32, 0_f32, 0_f32)), WHITE);
        assert_eq!(pattern.color_at(&Tuple::point(0_f32, 1_f32, 0_f32)), WHITE);
        assert_eq!(pattern.color_at(&Tuple::point(0_f32, 2_f32, 0_f32)), WHITE);
        assert_eq!(pattern.color_at(&Tuple::point(0_f32, 0_f32, 0_f32)), WHITE);
        assert_eq!(pattern.color_at(&Tuple::point(0_f32, 0_f32, 1_f32)), WHITE);
        assert_eq!(pattern.color_at(&Tuple::point(0_f32, 0_f32, 1_f32)), WHITE);
    }
    #[test]
    fn stripes_with_an_object_transformation() {
        let mut object = Sphere::new(1);
        object.set_transformation(Matrix4::identity().scale(2_f32, 2_f32, 2_f32));
        let pattern = StripePattern::test_default();
        let color = pattern.color_at_obj(&object, &Tuple::point(1.5_f32, 0_f32, 0_f32));
        assert_eq!(color, WHITE);
    }
    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Sphere::new(1);
        let mut pattern = StripePattern::test_default();
        pattern.set_transform(&Matrix4::identity().scale(2_f32, 2_f32, 2_f32));
        let color = pattern.color_at_obj(&object, &Tuple::point(1.5_f32, 0_f32, 0_f32));
        assert_eq!(color, WHITE);
    }
    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let mut object = Sphere::new(1);
        object.set_transformation(Matrix4::identity().scale(2_f32, 2_f32, 2_f32));
        let mut pattern = StripePattern::test_default();
        pattern.set_transform(&Matrix4::identity().translate(0.5_f32, 0_f32, 0_f32));
        let color = pattern.color_at_obj(&object, &Tuple::point(3.5_f32, 0_f32, 0_f32));
        assert_eq!(color, BLACK);
    }
    #[test]
    fn a_gradient_linerarly_interpolates_between_colors() {
        let pattern = GradientPattern {
            first: WHITE,
            second: BLACK,
            inverse_transformation: Matrix4::identity(),
        };
        assert_eq!(pattern.color_at(&Tuple::point(0_f32, 0_f32, 0_f32)), WHITE);
        assert_eq!(
            pattern.color_at(&Tuple::point(0.25_f32, 0_f32, 0_f32)),
            Color::new(0.75_f32, 0.75_f32, 0.75_f32)
        );
        assert_eq!(
            pattern.color_at(&Tuple::point(0.50_f32, 0_f32, 0_f32)),
            Color::new(0.5_f32, 0.5_f32, 0.5_f32)
        );
        assert_eq!(
            pattern.color_at(&Tuple::point(0.75_f32, 0_f32, 0_f32)),
            Color::new(0.25_f32, 0.25_f32, 0.25_f32)
        );
    }
    #[test]
    fn a_ring_shoud_extend_in_both_x_and_y() {
        let pattern = RingPattern {
            first: WHITE,
            second: BLACK,
            inverse_transformation: Matrix4::identity(),
        };
        assert_eq!(pattern.color_at(&Tuple::point(0_f32, 0_f32, 0_f32)), WHITE);
        assert_eq!(pattern.color_at(&Tuple::point(1_f32, 0_f32, 0_f32)), BLACK);
        assert_eq!(pattern.color_at(&Tuple::point(0_f32, 0_f32, 1_f32)), BLACK);
        assert_eq!(
            pattern.color_at(&Tuple::point(0.708_f32, 0_f32, 0.708_f32)),
            BLACK
        );
    }
}
