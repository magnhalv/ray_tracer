mod tuple;
mod color;
mod canvas;
mod matrix;
mod transformation;
mod ray;
mod sphere;

use matrix::{Matrix4};
use tuple::Tuple;
use ray::{Ray, intersects};
use sphere::Sphere;

fn main() {
    let x_dim = 1200;
    let y_dim = 550;    
    let mut canvas = canvas::Canvas::new(x_dim, y_dim);        

    let mut sphere = Sphere::new(1);
    sphere.set_transformation(Matrix4::identity().translate(600_f32, 275_f32, 1_f32).scale(275_f32, 275_f32, 1_f32));    
    
    let hit_color = color::Color::new(1_f32, 1_f32, 1_f32);

    let mut nof_misses = 0;
    let mut nof_hits = 0;

    let mut ray = Ray {
        origin: Tuple::new_point(0_f32, 0_f32, -1_f32),
        direction: Tuple::new_vector(0_f32, 0_f32, 1_f32)
    };
    for y in 0..y_dim {
        for x in 0..x_dim {
            ray.origin.x = x as f32;
            ray.origin.y = y as f32;

            let intersections = intersects(&sphere, &ray);

            if intersections.len() > 0 {
                canvas::set_pixel(&mut canvas, x, y, hit_color);
                nof_hits = nof_hits + 1;
            }
            else {
                nof_misses = nof_misses + 1;                
            }
        }   
        
    }

    println!("Nof hits: {}. Nof misses: {}.", nof_hits, nof_misses);

    canvas::canvas_to_file(&canvas, "test.ppm".to_string());
}


