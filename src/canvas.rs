use crate::color::{Color};
use std::fs;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Box<[Color]>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {                
        Canvas {
            width, height,
            pixels: vec![unsafe { std::mem::zeroed() }; width*height].into_boxed_slice()
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y*self.width + x]
    }

    pub fn set_pixel (&mut self, x: usize, y: usize, color: Color) {        
        self.pixels[y*self.width + x] = color;
    }
}

pub fn get_pixel(canvas: &Canvas, x: usize, y: usize) -> Color {
    canvas.pixels[y*canvas.width + x]
}

pub fn set_pixel (canvas: &mut Canvas, x: usize, y: usize, color: Color) {        
    canvas.pixels[y*canvas.width + x] = color;
}

pub fn canvas_to_ppm(canvas: &Canvas) -> String {
    let mut result = String::from(format!("P3\n{0} {1}\n255\n", canvas.width, canvas.height));    
    for y in 0..canvas.height {        
        let mut line = String::with_capacity(canvas.width*3 + (canvas.width*3-1));
        for x in 0..canvas.width {
            let color = get_pixel(canvas, x, y);
            let r = limit((color.red * 255.0_f32) as i32, 0, 255);
            let g = limit((color.green * 255.0_f32) as i32, 0, 255);
            let b = limit((color.blue * 255.0_f32) as i32, 0, 255);

            if x == 0 {
                line.push_str(&format!("{0} {1} {2}", r, g, b))
            }
            else {
                line.push_str(&format!(" {0} {1} {2}", r, g, b))
            }
            
        }
        result.push_str(&format!("{}\n", line));
    }
    result
}

pub fn canvas_to_file(canvas: &Canvas, file_name: String) {
    let canvas = canvas_to_ppm(canvas);
    fs::write(file_name, canvas).expect("Unable to write cavas to ppm file.");
}

fn limit(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        return min
    }
    if value > max {
        return max
    }
    value
}

#[cfg(test)]
mod canvas_tests {

    use crate::canvas::{Canvas, set_pixel, get_pixel, canvas_to_ppm};
    use crate::Color;

    #[test]
    fn init_canvas() {
        let canvas = Canvas::new(5, 10);
        assert_eq!(canvas.width, 5);
        assert_eq!(canvas.height, 10);
    
        let black = Color::new(0_f32, 0_f32, 0_f32);
        for c in canvas.pixels.to_vec() {
            assert_eq!(c, black);
        }
    }
    
    
    #[test]
    fn set_pixel_test() {
        let mut canvas = Canvas::new(5, 10);    
    
        let red = Color::new(1_f32, 0_f32, 0_f32);
        let x = 2;
        let y = 3;    
        set_pixel(&mut canvas, x, y, red);
        let pixel = get_pixel(&canvas, x, y);
        assert_eq!(pixel, red);
    }
    
    #[test]
    fn to_ppm() {
        let mut canvas = Canvas::new(5, 3);    
        set_pixel(&mut canvas, 0, 0, Color::new(1.5_f32, 0_f32, 0_f32));
        set_pixel(&mut canvas, 2, 1, Color::new(0_f32, 0.5_f32, 0_f32));
        set_pixel(&mut canvas, 4, 2, Color::new(-0.5_f32, 0_f32, 1_f32));
        let result = canvas_to_ppm(&canvas);
        let expected = 
    "P3
    5 3
    255
    255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 127 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";
    
        assert_eq!(result, expected);
    }
    
    
}

