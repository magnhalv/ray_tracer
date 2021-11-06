use std::fmt;
use std::ops::{Add, Mul, Sub};
use std::cmp::PartialEq;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f32, // 255
    pub green: f32, // 255
    pub blue: f32, // 255
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.red, self.green, self.blue)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }

    fn ne(&self, other: &Color) -> bool {
        self.red != other.red || self.green != other.green || self.blue != other.blue
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            red: self * rhs.red,
            green: self * rhs.green,
            blue: self * rhs.blue,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color {
            red: red,
            green: green,
            blue: blue,
        }
    }
}

#[test]
fn add_colors() {
    let c1 = Color::new(10.0_f32, 9.0_f32, 8.0_f32);
    let c2 = Color::new(1.0_f32, 2.0_f32, 3.0_f32);
    let result = c1 - c2;
    let expected = Color::new(9.0_f32, 7.0_f32, 5.0_f32);
    assert_eq!(result, expected);
}

#[test]
fn sub_colors() {
    let c1 = Color::new(10.0_f32, 9.0_f32, 8.0_f32);
    let c2 = Color::new(1.0_f32, 2.0_f32, 3.0_f32);
    let result = c1 - c2;
    let expected = Color::new(9.0_f32, 7.0_f32, 5.0_f32);
    assert_eq!(result, expected);
}

#[test]
fn multiply_color_by_scalar() {
    let color = Color::new(10.0_f32, -9.0_f32, 8.0_f32);
    let result1 = color * 2.0_f32;
    let result2 = 2.0_f32 * color;
    let expected = Color::new(20.0_f32, -18.0_f32, 16.0_f32);
    assert_eq!(result1, expected);
    assert_eq!(result2, expected);
}


#[test]
fn multiply_color_by_color() {
    let c1 = Color::new(10.0_f32, -9.0_f32, 8.0_f32);
    let c2 = Color::new(2.0_f32, 3.0_f32, 1.0_f32);
    let result = c1 * c2;
    let expected = Color::new(20.0_f32, -27.0_f32, 8.0_f32);
    assert_eq!(result, expected);
}
