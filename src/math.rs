pub const EPSILON : f32 = 0.001_f32;
pub const SHADOW_EPSILON : f32 = 0.001_f32;
pub const INFINITY: f32 = std::f32::INFINITY;
pub const PI: f32 = core::f32::consts::PI;

pub fn float_equal(a: f32, b: f32) -> bool {    
    a == b || (a - b).abs() <= EPSILON
}