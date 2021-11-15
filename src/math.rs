pub const EPSILON : f32 = 0.0001_f32;
pub const SHADOW_EPSILON : f32 = 0.01_f32;

pub fn float_equal(a: f32, b: f32) -> bool {    
    a == b || (a - b).abs() <= EPSILON
}