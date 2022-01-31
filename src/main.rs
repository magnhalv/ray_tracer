extern crate minifb;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

use crate::camera::render_at;
use crate::camera::Camera;
use crate::color::BLACK;
use crate::shape::Shape;
use crate::transformation::view_transform;
use crate::world_generator::generate_world;
use crate::world_generator::generate_test_world;
use core::f32::consts::PI;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
mod camera;
mod color;
mod cube;
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
mod world_generator;

use color::Color;
use lighting::PointLight;
use matrix::Matrix4;
use ray::Ray;
use tuple::Tuple;
const DIM_X: usize = 2560;
const DIM_Y: usize = 1440;
//const DIM_X: usize = 800;
//const DIM_Y: usize = 540;

fn main() {
    let world = generate_world();

    let mut camera = Camera::new(DIM_X, DIM_Y, PI / 3_f32);
    let from = Tuple::point(25_f32, 20_f32, -60_f32);
    let to = Tuple::point(0_f32, 1_f32, 50_f32);
    let up = Tuple::vector(0_f32, 1_f32, 0_f32);
    camera.set_transform(&view_transform(&from, &to, &up));

    let mut color_buffer: Vec<u32> = vec![0; DIM_X * DIM_Y];

    let mut window = Window::new("Test - ESC to exit", DIM_X, DIM_Y, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    /* let mut y = 0;
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
    } */

    let hsize = camera.hsize;
    let vsize = camera.vsize;

    let color = render_at(240, 148, &camera, &world);        
    let color2 = render_at(32, 212, &camera, &world);

    let new_world = Arc::new(world);
    let new_camera = Arc::new(camera);

    let num_threads = 36;
    let mut thread_buffers: Vec<Arc<Mutex<Vec<u32>>>> = vec![];
    for _ in 0..num_threads {
        thread_buffers.push(Arc::new(Mutex::new(vec![0; hsize])));
    }
    let mut y = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
        .update_with_buffer(&color_buffer, DIM_X, DIM_Y)
        .unwrap();
        let mut handles = vec![];

        if y < vsize {
            for thread in 0..num_threads {
                let buffer = Arc::clone(&thread_buffers[thread]);
                let local_world = Arc::clone(&new_world);
                let local_camera = Arc::clone(&new_camera);
                let local_y = y + thread;
                let handle = thread::spawn(move || {
                    let mut buffer = buffer.lock().unwrap();
                    for x in 0..hsize {
                        let color = render_at(x, local_y, &local_camera, &local_world);
                        let r = limit((color.red * 255.0_f32) as i32, 0, 255);
                        let g = limit((color.green * 255.0_f32) as i32, 0, 255);
                        let b = limit((color.blue * 255.0_f32) as i32, 0, 255);
                        let value = u32::from_be_bytes([0, r, g, b]);
                        buffer[x] = value;
                    }
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }

            for thread in 0..num_threads {
                let buffer = &*thread_buffers[thread].lock().unwrap();
                let local_y = y + thread;
                let mut x = 0;
                for value in buffer {
                    color_buffer[(local_y * hsize) + x] = *value;
                    x = x + 1;
                }
            }

            window
                .update_with_buffer(&color_buffer, DIM_X, DIM_Y)
                .unwrap();
            y = y + num_threads;
        } else {
            //thread::sleep(Duration::from_millis(1000));
            let pos = window.get_mouse_pos(MouseMode::Clamp);
            match pos {
                Some(p) => {
                    let local_world = Arc::clone(&new_world);
                    let local_camera = Arc::clone(&new_camera);
                    let color = render_at(p.0 as usize, p.1 as usize, &local_camera, &local_world);

                    println!(
                        "Pos: {}, {}, Color: {}, {}, {} ",
                        p.0, p.1, color.red, color.green, color.blue
                    );                
                    // 48, 203 - wrong
                    // 42, 213 - correct

                }
                None => println!("No pos"),
            }

            // Need update to keep checing for key down
            window
                .update_with_buffer(&color_buffer, DIM_X, DIM_Y)
                .unwrap();
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
