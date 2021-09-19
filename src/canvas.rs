use crate::color::{Color};

struct Canvas {
    width: usize,
    height: usize,
    pixels: Box<[Color]>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {                
        Canvas {
            width, height,
            pixels: vec![unsafe { std::mem::zeroed() }; width*height].into_boxed_slice()
        }
    }
}

#[test]
fn init_canvas() {
    let canvas = Canvas::new(5, 10);
    assert_eq!(canvas.width, 5);
    assert_eq!(canvas.height, 10);

    let black = Color::new(0_f64, 0_f64, 0_f64);
    for c in canvas.pixels.to_vec() {
        assert_eq!(c, black);
    }
}
