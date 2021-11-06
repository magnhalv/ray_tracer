mod geometry;
mod color;
mod canvas;
mod matrix;
mod transformation;

use matrix::Matrix4;
use geometry::Tuple;
use std::f32::consts::{PI};

fn main() {
    let mut canvas = canvas::Canvas::new(1200, 550);    

    let translate_origin = Matrix4::identity().translate(600_f32, 275_f32, 0_f32).scale(200_f32, 200_f32, 0_f32);

    let point = Tuple::new_point(0_f32, 1_f32, 0_f32);
    let color = color::Color::new(1_f32, 1_f32, 1_f32);
    for hour in 0..12 {
        let hour = hour as f32;
        let rotate = Matrix4::identity().rotate_z((12_f32 - hour) * (2_f32*PI)/12_f32);  
        let transformation = &translate_origin * &rotate;      
        let point = &transformation * &point;
        println!("Hour {0}: {1}", hour, point);

        let x = point.x as usize;
        let y = point.y as usize;
        canvas::set_pixel(&mut canvas, x, y, color);
    }

    canvas::canvas_to_file(&canvas, "test.ppm".to_string());
}


