use crate::color::black;
use crate::color::white;
use crate::Tuple;
use crate::Color;

pub trait Pattern {
    fn color_at(&self, point: &Tuple) -> Color;
}

pub struct StripePattern {
    pub first: Color,
    pub second: Color
}

impl Pattern for StripePattern {    
    fn color_at(&self, point: &Tuple) -> Color { 
        if (point.x as i32) % 2 == 0 {
            return self.first
        }
        self.second
     }
}

#[test]
fn stripe_pattern_alternates_only_in_x() {
    let pattern = StripePattern {
        first: white,
        second: black
    };

    assert_eq!(pattern.color_at(&Tuple::point(0_f32, 0_f32, 0_f32)), white);
    assert_eq!(pattern.color_at(&Tuple::point(1_f32, 0_f32, 0_f32)), black);
    assert_eq!(pattern.color_at(&Tuple::point(2_f32, 0_f32, 0_f32)), white);

    assert_eq!(pattern.color_at(&Tuple::point(0_f32, 0_f32, 0_f32)), white);
    assert_eq!(pattern.color_at(&Tuple::point(0_f32, 1_f32, 0_f32)), white);
    assert_eq!(pattern.color_at(&Tuple::point(0_f32, 2_f32, 0_f32)), white);

    assert_eq!(pattern.color_at(&Tuple::point(0_f32, 0_f32, 0_f32)), white);
    assert_eq!(pattern.color_at(&Tuple::point(0_f32, 0_f32, 1_f32)), white);
    assert_eq!(pattern.color_at(&Tuple::point(0_f32, 0_f32, 1_f32)), white);
}