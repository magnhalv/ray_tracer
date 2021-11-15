extern crate minifb;

use crate::camera::render;
use crate::transformation::view_transform;
use crate::camera::Camera;
use crate::world::World;
use core::f32::consts::PI;
use minifb::{Key, Window, WindowOptions};
mod camera;
mod canvas;
mod color;
mod lighting;
mod math;
mod matrix;
mod ray;
mod sphere;
mod transformation;
mod tuple;
mod world;

use color::Color;
use lighting::{lighting, PointLight};
use matrix::Matrix4;
use ray::{intersects, Ray};
use sphere::Sphere;
use tuple::Tuple;

const DIM_X : usize = 1600;
const DIM_Y : usize = 800;

fn main() {    

    let mut floor = Sphere::new(1);
    floor.set_transformation(Matrix4::identity().scale(10_f32, 0.01_f32, 10_f32));
    floor.material.color = Color::new(1_f32, 0.9_f32, 0.9_f32);
    floor.material.specular = 0_f32;
    let mut left_wall = Sphere::new(2);
    left_wall.set_transformation(
        Matrix4::identity()
            .translate(0_f32, 0_f32, 5_f32)
            .rotate_y(-PI / 4_f32)
            .rotate_x(PI/2_f32)
            .scale(10_f32, 0.01_f32, 10_f32),            
    );
    left_wall.material.color = Color::new(1_f32, 0.9_f32, 0.9_f32);
    left_wall.material.specular = 0_f32;

    let mut right_wall = Sphere::new(3);
    right_wall.set_transformation(
        Matrix4::identity()
            .translate(0_f32, 0_f32, 5_f32)
            .rotate_y(PI / 4_f32)
            .rotate_x(PI/2_f32)
            .scale(10_f32, 0.01_f32, 10_f32),            
    );
    right_wall.material.color = Color::new(1_f32, 0.9_f32, 0.9_f32);
    right_wall.material.specular = 0_f32;

    let mut middle = Sphere::new(4);
    middle.set_transformation(Matrix4::identity().translate(-0.5_f32, 1_f32, 0.5_f32));
    middle.material.color = Color::new(0.1_f32, 1_f32, 0.5_f32);
    middle.material.diffuse = 0.7_f32;
    middle.material.specular = 0.3_f32;
    
    
    let mut right = Sphere::new(5);
    right.set_transformation(Matrix4::identity().translate(1.5_f32, 0.5_f32, -0.5_f32).scale(0.5_f32, 0.5_f32, 0.5_f32));
    right.material.color = Color::new(0.5_f32, 0.5_f32, 1_f32);
    right.material.diffuse = 0.7_f32;
    right.material.specular = 0.3_f32;
    
    let mut left = Sphere::new(6);
    left.set_transformation(Matrix4::identity().translate(-1.5_f32, 0.33_f32, -0.75_f32).scale(0.33_f32, 0.33_f32, 0.33_f32));
    left.material.color = Color::new(1_f32, 1_f32, 0.0_f32);
    left.material.diffuse = 0.7_f32;
    left.material.specular = 0.3_f32;

    let light = PointLight::new(Tuple::point(-10_f32, 10_f32, -10_f32), Color::new(1_f32, 1_f32, 1_f32));
    let mut world = World::new(light);
    world.objects.push(floor);
    world.objects.push(left_wall);
    world.objects.push(right_wall);
    world.objects.push(middle);    
    world.objects.push(left);
    world.objects.push(right);

    let mut camera = Camera::new(DIM_X, DIM_Y, PI/3_f32);
    let from = Tuple::point(0_f32, 1.5_f32, -5_f32);
    let to = Tuple::point(0_f32, 1_f32, 0_f32);
    let up = Tuple::vector(0_f32, 1_f32, 0_f32);
    camera.set_transform(&view_transform(&from, &to, &up));
    let canvas = render(&camera, &world);    

    canvas::canvas_to_file(&canvas, "test.ppm".to_string());

    let mut buffer: Vec<u32> = vec![0; DIM_X * DIM_Y];

    let mut window = Window::new(
        "Test - ESC to exit",
        DIM_X,
        DIM_Y,
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
            .update_with_buffer(&buffer, DIM_X, DIM_Y)
            .unwrap();
    }
}

fn limit(value: i32, min: u8, max: u8) -> u8 {
    if value < min as i32 {
        return min;
    }
    if value > max as i32 {
        return max;
    }
    value as u8
}
