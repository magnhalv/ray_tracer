extern crate minifb;

use crate::matrix::inverse4;
use crate::pattern::Pattern;
use crate::pattern::CheckerPattern;
use crate::pattern::RingPattern;
use crate::pattern::GradientPattern;
use crate::camera::render;
use crate::camera::Camera;
use crate::color::BLACK;
use crate::color::WHITE;
use crate::pattern::StripePattern;
use crate::plane::Plane;
use crate::shape::Shape;
use crate::transformation::view_transform;
use crate::world::World;
use core::f32::consts::PI;
use minifb::{Key, Window, WindowOptions};
mod camera;
mod canvas;
mod color;
mod lighting;
mod material;
mod math;
mod matrix;
mod pattern;
mod plane;
mod ray;
mod shape;
mod sphere;
mod transformation;
mod tuple;
mod world;

use color::Color;
use lighting::{lighting, PointLight};
use matrix::Matrix4;
use ray::Ray;
use sphere::Sphere;
use tuple::Tuple;

const DIM_X: usize = 1600;
const DIM_Y: usize = 800;

fn main() {
    /* Floor  */
    let mut floor = Plane::new();
    //floor.set_transformation(Matrix4::identity().(10_f32, 0.01_f32, 10_f32));
    floor.material.color = Color::new(1_f32, 0.9_f32, 0.9_f32);
    floor.material.specular = 0.3_f32;
    floor.material.diffuse = 0.7_f32;
    floor.set_transformation(Matrix4::identity().scale(5_f32, 5_f32, 5_f32));
    let mut floorPattern = CheckerPattern {
        first: WHITE,
        second: BLACK,
        inverse_transformation: Matrix4::identity()
    };    

    floorPattern.set_transform(&Matrix4::identity().scale(1_f32, 1_f32, 1_f32));    
    floor.material.pattern = Some(Box::new(floorPattern));

    /* Middle  */
    let mut middle = Sphere::new(4);
    middle.set_transformation(
        Matrix4::identity()
            .translate(-5_f32, 10_f32, 5_f32)
            .scale(10_f32, 10_f32, 10_f32),
    );
    middle.material.color = Color::new(128_f32, 0_f32, 128_f32);
    middle.material.diffuse = 0.7_f32;
    middle.material.specular = 0.3_f32;
    let mut middlePattern = StripePattern {
        first: Color::new(0_f32, 0.4_f32, 0.0_f32),
        second: Color::new(0.4_f32, 0.8_f32, 0.4_f32),
        inverse_transformation: Matrix4::identity()
    };    
    middlePattern.set_transform(&Matrix4::identity().scale(0.2_f32, 0.2_f32, 0.2_f32));    
    middle.material.pattern = Some(Box::new(middlePattern));

    
    
    //middle.material.pattern = Some(Box::new(floorPattern));
    
    let mut right = Sphere::new(5);
    right.set_transformation(
        Matrix4::identity()
            .translate(15_f32, 5_f32, -5_f32)
            .scale(5_f32, 5_f32, 5_f32),
    );    
    right.material.diffuse = 0.7_f32;
    right.material.specular = 0.3_f32;
    let mut right_pattern = GradientPattern {
        first: Color::new(1_f32, 0.0_f32, 0.0_f32),
        second: Color::new(0.0_f32, 0.0_f32, 1.0_f32),
        inverse_transformation: Matrix4::identity()
    };    
    right_pattern.set_transform(&Matrix4::identity().translate(1_f32, 0_f32, 0_f32).scale(2_f32, 1_f32, 1_f32));    
    right.material.pattern = Some(Box::new(right_pattern));
        
    /* LEFT */

    let mut left = Sphere::new(6);
    left.set_transformation(
        Matrix4::identity()
            .translate(-15_f32, 5_f32, -5.5_f32)
            .scale(5_f32, 5_f32, 5_f32),
    );
    left.material.diffuse = 0.7_f32;
    left.material.specular = 0.3_f32;
    let mut left_pattern = RingPattern {
        first: Color::new(0_f32, 0.4_f32, 0.0_f32),
        second: Color::new(0.4_f32, 0.8_f32, 0.4_f32),
        inverse_transformation: Matrix4::identity()
    };    
    left_pattern.set_transform(&Matrix4::identity().translate(-1_f32, 0_f32, -1_f32));    
    left.material.pattern = Some(Box::new(left_pattern));

    // REST
    let light = PointLight::new(
        Tuple::point(-100_f32, 100_f32, -100_f32),
        Color::new(1_f32, 1_f32, 1_f32),
    );
    let mut world = World::new(light);
    world.objects.push(&floor);
    world.objects.push(&middle);
    world.objects.push(&left);
    world.objects.push(&right);

    let mut camera = Camera::new(DIM_X, DIM_Y, PI / 3_f32);
    let from = Tuple::point(0_f32, 15_f32, -50_f32);
    let to = Tuple::point(0_f32, 1_f32, 50_f32);
    let up = Tuple::vector(0_f32, 1_f32, 0_f32);
    camera.set_transform(&view_transform(&from, &to, &up));
    let canvas = render(&camera, &world);

    canvas::canvas_to_file(&canvas, "test.ppm".to_string());

    let mut buffer: Vec<u32> = vec![0; DIM_X * DIM_Y];

    let mut window = Window::new("Test - ESC to exit", DIM_X, DIM_Y, WindowOptions::default())
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
            *i = u32::from_be_bytes(value);
            index = index + 1;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, DIM_X, DIM_Y).unwrap();
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
