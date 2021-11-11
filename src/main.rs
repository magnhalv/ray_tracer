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

use matrix::{Matrix4};
use tuple::Tuple;
use ray::{Ray, intersects};
use sphere::Sphere;
use color::Color;
use lighting::{PointLight, lighting};

fn main() {
    let dim = 2000;    
    let wall_size = 7_f32;
    let wall_z = 10_f32;
    let pixel_size = wall_size/(dim as f32);
    let half = wall_size / 2_f32    ;
    let mut canvas = canvas::Canvas::new(dim, dim);        

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
    for y in 0..dim {
        let world_y = half - (pixel_size * (y as f32));
        for x in 0..dim {
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
}


