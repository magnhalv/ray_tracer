use crate::matrix::inverse4;
use crate::Matrix4;
use crate::Sphere;
use crate::color::BLACK;
use crate::color::WHITE;
use crate::Tuple;
use crate::Color;
use crate::Shape;

pub trait Pattern {
    fn color_at(&self, point: &Tuple) -> Color;
    fn color_at_obj(&self, object: &Shape, world_point: &Tuple) -> Color;
    fn set_transform(&mut self, transform: &Matrix4);
}

pub struct StripePattern {
    pub first: Color,
    pub second: Color,
    pub inverse_transformation: Matrix4
}

impl StripePattern {
    pub fn new_box(first: Color, second: Color) -> Option<Box<dyn Pattern>> {
        Some(Box::new(StripePattern { first, second, inverse_transformation: Matrix4::identity() }))
    }

    fn test_default() -> StripePattern {
        StripePattern { first: WHITE, second: BLACK, inverse_transformation: Matrix4::identity() }
    }
}

impl Pattern for StripePattern {    
    fn color_at(&self, point: &Tuple) -> Color { 
        if (point.x.abs() as i32) % 2 == 0 {
            return self.first
        }
        self.second
    }
    
    fn color_at_obj(&self, object: &Shape, world_point: &Tuple) -> Color {
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
    pub inverse_transformation: Matrix4
}

impl Pattern for GradientPattern {    
    fn color_at(&self, point: &Tuple) -> Color { 
        let distance = self.second - self.first;
        let x = point.x.abs();
        let fraction = x - x.floor();
        self.first + (distance * fraction)
    }
    
    fn color_at_obj(&self, object: &Shape, world_point: &Tuple) -> Color {
        let obj_point = object.get_inverse_transformation() * world_point;
        let pattern_point = &self.inverse_transformation * &obj_point;
        self.color_at(&pattern_point)
    }

    fn set_transform(&mut self, transform: &Matrix4) {
        self.inverse_transformation = inverse4(transform);
    }
}

pub struct RingPattern {
    pub first: Color,
    pub second: Color,
    pub inverse_transformation: Matrix4
}

impl Pattern for RingPattern {    
    fn color_at(&self, point: &Tuple) -> Color { 
        let magnitude = (point.x.powf(2_f32) + point.z.powf(2_f32)).sqrt();
        if (magnitude as i32) % 2 == 0 {
            return self.first
        }
        self.second
    }
    
    fn color_at_obj(&self, object: &Shape, world_point: &Tuple) -> Color {
        let obj_point = object.get_inverse_transformation() * world_point;
        let pattern_point = &self.inverse_transformation * &obj_point;
        self.color_at(&pattern_point)
    }

    fn set_transform(&mut self, transform: &Matrix4) {
        self.inverse_transformation = inverse4(transform);
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
        inverse_transformation: Matrix4::identity()
    };

    assert_eq!(pattern.color_at(&Tuple::point(0_f32, 0_f32, 0_f32)), WHITE);
    assert_eq!(pattern.color_at(&Tuple::point(0.25_f32, 0_f32, 0_f32)), Color::new(0.75_f32, 0.75_f32, 0.75_f32));
    assert_eq!(pattern.color_at(&Tuple::point(0.50_f32, 0_f32, 0_f32)), Color::new(0.5_f32, 0.5_f32, 0.5_f32));
    assert_eq!(pattern.color_at(&Tuple::point(0.75_f32, 0_f32, 0_f32)), Color::new(0.25_f32, 0.25_f32, 0.25_f32));
}