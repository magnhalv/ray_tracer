extern crate minifb;

use minifb::{Key, Window, WindowOptions};
mod tuple;
mod color;
mod canvas;
mod matrix;
mod transformation;
mod ray;
mod sphere;
mod lighting;
mod math;
mod world;
mod camera;

use matrix::{Matrix4};
use tuple::Tuple;
use ray::{Ray, intersects};
use sphere::Sphere;
use color::Color;
use lighting::{PointLight, lighting};

const DIM: usize = 1000;

fn main() {    
    let wall_size = 7_f32;
    let wall_z = 10_f32;
    let pixel_size = wall_size/(DIM as f32);
    let half = wall_size / 2_f32    ;
    let mut canvas = canvas::Canvas::new(DIM, DIM);        

    let mut sphere = Sphere::new(1);
    //sphere.set_transformation(Matrix4::identity().translate(600_f32, 275_f32, -1_f32).scale(240_f32, 240_f32, 1_f32));    
    sphere.material.color = Color::new(1_f32, 0.2_f32, 1_f32);
        
    let light = PointLight {
        position: Tuple::point(-10_f32, 10_f32, -10_f32),
        intensity: Color::new(1_f32, 1_f32, 1_f32)
    };    

    let mut nof_misses = 0;
    let mut nof_hits = 0;

    let mut ray = Ray {
        origin: Tuple::point(0_f32, 0_f32, -5_f32),
        direction: Tuple::vector(0_f32, 0_f32, 1_f32)
    };
    for y in 0..DIM {
        let world_y = half - (pixel_size * (y as f32));
        for x in 0..DIM {
            let world_x = -half + ((pixel_size) * (x as f32));

            ray.direction = (Tuple::vector(world_x, world_y, wall_z) - ray.origin).normalize();

            let intersections = intersects(&sphere, &ray);

            if intersections.len() > 0 {
                let hit = &intersections[0];
                let hit_point = ray.position(hit.t);
                let normal = sphere.normal_at(&hit_point);
                let eye = -ray.direction;

                let color = lighting(&sphere.material, &light, &ray.origin, &eye, &normal);
                canvas::set_pixel(&mut canvas, x, y, color);
                nof_hits = nof_hits + 1;
            }
            else {
                nof_misses = nof_misses + 1;                
            }
        }   
        
    }

    println!("Nof hits: {}. Nof misses: {}.", nof_hits, nof_misses);

    canvas::canvas_to_file(&canvas, "test.ppm".to_string());

    let mut buffer: Vec<u32> = vec![0; DIM * DIM];

    let mut window = Window::new(
        "Test - ESC to exit",
        DIM,
        DIM,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut index = 0;
        for i in buffer.iter_mut() {
            let color = canvas.pixels[index];
            let r = limit((color.red * 255.0_f32) as i32, 0, 255);
            let g = limit((color.green * 255.0_f32) as i32, 0, 255);
            let b = limit((color.blue * 255.0_f32) as i32, 0, 255);

            let value = [0, r, g, b];
            *i = u32::from_be_bytes(value); // write something more funny here!
            index = index + 1;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, DIM, DIM)
            .unwrap();
    }
}

fn limit(value: i32, min: u8, max: u8) -> u8 {
    if value < min as i32 {
        return min
    }
    if value > max as i32 {
        return max
    }
    value as u8
}


