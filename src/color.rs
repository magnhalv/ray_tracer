use std::fmt;
use std::ops::{Add, Mul, Sub};
use std::cmp::PartialEq;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
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

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul<Color> for f64 {
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
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color {
            red: red,
            green: green,
            blue: blue,
        }
    }
}

#[test]
fn add_colors() {
    let c1 = Color::new(10.0_f64, 9.0_f64, 8.0_f64);
    let c2 = Color::new(1.0_f64, 2.0_f64, 3.0_f64);
    let result = c1 - c2;
    let expected = Color::new(9.0_f64, 7.0_f64, 5.0_f64);
    assert_eq!(result, expected);
}

#[test]
fn sub_colors() {
    let c1 = Color::new(10.0_f64, 9.0_f64, 8.0_f64);
    let c2 = Color::new(1.0_f64, 2.0_f64, 3.0_f64);
    let result = c1 - c2;
    let expected = Color::new(9.0_f64, 7.0_f64, 5.0_f64);
    assert_eq!(result, expected);
}

#[test]
fn multiply_color_by_scalar() {
    let color = Color::new(10.0_f64, -9.0_f64, 8.0_f64);
    let result1 = color * 2.0_f64;
    let result2 = 2.0_f64 * color;
    let expected = Color::new(20.0_f64, -18.0_f64, 16.0_f64);
    assert_eq!(result1, expected);
    assert_eq!(result2, expected);
}


#[test]
fn multiply_color_by_color() {
    let c1 = Color::new(10.0_f64, -9.0_f64, 8.0_f64);
    let c2 = Color::new(2.0_f64, 3.0_f64, 1.0_f64);
    let result = c1 * c2;
    let expected = Color::new(20.0_f64, -27.0_f64, 8.0_f64);
    assert_eq!(result, expected);
}
