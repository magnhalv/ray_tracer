extern crate minifb;
use std::{thread, time::Duration};

use crate::camera::Camera;
use crate::camera::{render, render_at};
use crate::color::BLACK;
use crate::color::WHITE;
use crate::material::GLASS_REFRACTIVE_INDEX;
use crate::matrix::inverse4;
use crate::pattern::CheckerPattern;
use crate::pattern::GradientPattern;
use crate::pattern::Pattern;
use crate::pattern::RingPattern;
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
//const DIM_X: usize = 2560;
//const DIM_Y: usize = 1440;
const DIM_X: usize = 800;
const DIM_Y: usize = 400;

fn main() {
    let mut wall = Plane::new(1);
    wall.set_transformation(
        Matrix4::identity()
            .scale(10.0, 10.0, 1.0)
            .translate(0.0, -20.0, 100.0)
            .rotate_x(PI / 2_f32),
    );
    //floor.set_transformation(Matrix4::identity().(10_f32, 0.01_f32, 10_f32));
    wall.material.color = Color::new(0.0_f32, 0.0_f32, 0.0_f32);
    wall.material.diffuse = 0.1;
    wall.material.specular = 1.0;
    wall.material.ambient = 0.1;
    wall.material.reflective = 0.9;
    wall.material.shininess = 300.0;

    let mut wall_pattern = CheckerPattern {
        first: WHITE,
        second: Color::new(0.0, 1.0, 0.0),
        inverse_transformation: Matrix4::identity(),
    };

    wall_pattern.set_transform(&Matrix4::identity().scale(1_f32, 1_f32, 1_f32));
    wall.material.pattern = Some(Box::new(wall_pattern));

    /* Floor  */
    let mut floor = Plane::new(2);
    //floor.set_transformation(Matrix4::identity().(10_f32, 0.01_f32, 10_f32));
    //floor.material.color = Color::new(0.8_f32, 0.8_f32, 0.8_f32);
    floor.material.specular = 0.3_f32;
    floor.material.diffuse = 0.7_f32;
    floor.material.reflective = 0.0;
    floor.set_transformation(Matrix4::identity().scale(5_f32, 5_f32, 5_f32));
    let mut floorPattern = CheckerPattern {
        first: WHITE,
        second: BLACK,
        inverse_transformation: Matrix4::identity(),
    };

    floorPattern.set_transform(&Matrix4::identity().scale(1_f32, 1_f32, 1_f32));
    floor.material.pattern = Some(Box::new(floorPattern));

    /* Middle  */
    let mut middle = Sphere::new(3);
    middle.set_transformation(
        Matrix4::identity()
            .translate(-5_f32, 15_f32, 30_f32)
            .scale(15_f32, 15_f32, 15_f32),
    );
    middle.material.color = Color::new(0.2, 0.2, 0.2);

    middle.material.diffuse = 0.1;
    middle.material.specular = 1.0;
    middle.material.ambient = 0.1;
    middle.material.transparency = 1.0;
    middle.material.reflective = 0.4;
    middle.material.shininess = 300.0;
    middle.material.refractive_index = 1.52;
    /* let mut middlePattern = StripePattern {
        first: Color::new(0_f32, 0.4_f32, 0.0_f32),
        second: Color::new(0.4_f32, 0.8_f32, 0.4_f32),
        inverse_transformation: Matrix4::identity()
    };
    middlePattern.set_transform(&Matrix4::identity().scale(0.2_f32, 0.2_f32, 0.2_f32));
    middle.material.pattern = Some(Box::new(middlePattern)); */

    //middle.material.pattern = Some(Box::new(floorPattern));
    let mut right = Sphere::new(4);
    right.set_transformation(
        Matrix4::identity()
            .translate(5_f32, 10_f32, 80_f32)
            .scale(10_f32, 10_f32, 10_f32),
    );
    right.material.diffuse = 0.7_f32;
    right.material.specular = 0.3_f32;
    right.material.reflective = 0.0_f32;
    let mut right_pattern = GradientPattern {
        first: Color::new(1_f32, 0.0_f32, 0.0_f32),
        second: Color::new(0.0_f32, 0.0_f32, 1.0_f32),
        inverse_transformation: Matrix4::identity(),
    };
    right_pattern.set_transform(
        &Matrix4::identity()
            .translate(-1_f32, 0_f32, 0_f32)
            .scale(2_f32, 1_f32, 1_f32),
    );
    right.material.pattern = Some(Box::new(right_pattern));
    /* LEFT */

    let mut left = Sphere::new(5);
    left.set_transformation(
        Matrix4::identity()
            .translate(-15_f32, 5_f32, -5.5_f32)
            .scale(5_f32, 5_f32, 5_f32)
            .rotate_x(PI / 2_f32),
    );
    left.material.color = Color::new(0.4_f32, 0.4_f32, 0.4_f32);
    left.material.ambient = 0.1_f32;
    left.material.diffuse = 0.4_f32;
    left.material.specular = 0.9_f32;
    left.material.shininess = 300.0;
    left.material.reflective = 0.7_f32;
    left.material.transparency = 0.0_f32;
    /* let mut left_pattern = RingPattern {
        first: Color::new(0_f32, 0.4_f32, 0.0_f32),
        second: Color::new(0.4_f32, 0.8_f32, 0.4_f32),
        inverse_transformation: Matrix4::identity()
    };
    left_pattern.set_transform(&Matrix4::identity().translate(-1_f32, -1_f32, -1_f32).scale(0.2_f32, 0.2_f32, 0.2_f32));
    left.material.pattern = Some(Box::new(left_pattern)); */

    // REST
    let light = PointLight::new(
        Tuple::point(-100_f32, 200_f32, -100_f32),
        Color::new(1.0, 1.0, 1.0),
    );
    let mut world = World::new(light);
    world.objects.push(&wall);
    world.objects.push(&floor);
    world.objects.push(&middle);
    world.objects.push(&left);
    world.objects.push(&right);

    let mut camera = Camera::new(DIM_X, DIM_Y, PI / 3_f32);
    let from = Tuple::point(30_f32, 20_f32, -60_f32);
    let to = Tuple::point(0_f32, 1_f32, 50_f32);
    let up = Tuple::vector(0_f32, 1_f32, 0_f32);
    camera.set_transform(&view_transform(&from, &to, &up));
    //let canvas = render(&camera, &world);

    let mut buffer: Vec<u32> = vec![0; DIM_X * DIM_Y];

    let mut window = Window::new("Test - ESC to exit", DIM_X, DIM_Y, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut y = 0;    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if y < camera.vsize {
            for x in 0..camera.hsize {
                let color = render_at(x, y, &camera, &world);
                let r = limit((color.red * 255.0_f32) as i32, 0, 255);
                let g = limit((color.green * 255.0_f32) as i32, 0, 255);
                let b = limit((color.blue * 255.0_f32) as i32, 0, 255);
                let value = u32::from_be_bytes([0, r, g, b]);
                buffer[y * camera.hsize + x] = value;
            }
            window.update_with_buffer(&buffer, DIM_X, DIM_Y).unwrap();
            y = y + 1;
        }
        else {                        
            thread::sleep(Duration::from_millis(100));            
            // Need update to keep checing for key down
            window.update_with_buffer(&buffer, DIM_X, DIM_Y).unwrap();
        }
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
