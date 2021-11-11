pub fn float_equal(a: f32, b: f32) -> bool {
    let max_diff = 0.00001_f32;
    a == b || (a - b).abs() <= max_diff
}