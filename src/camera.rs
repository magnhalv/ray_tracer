use crate::Sphere;
use crate::canvas::set_pixel;
use crate::world::{color_at, World};
use crate::Color;
use crate::canvas::Canvas;
use crate::transformation::view_transform;
use crate::matrix::inverse4;
use crate::Matrix4;
use crate::Ray;
use crate::Tuple;
use core::f32::consts::PI;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f32,
    inverse_transform: Matrix4,
    half_height: f32,
    half_width: f32,
    pixel_size: f32,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Camera {
        // tan a = opp/adj, but adj is 1 since the canvas is always 1 unit from the camera.
        let half_view = (field_of_view / 2_f32).tan();
        let aspect_ratio = hsize as f32 / vsize as f32;

        let (half_width, half_height): (f32, f32);
        if aspect_ratio >= 1_f32 {
            half_width = half_view;
            half_height = half_view / aspect_ratio;
        } else {
            half_width = half_view * aspect_ratio;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2_f32) / hsize as f32;

        Camera {
            hsize,
            vsize,
            field_of_view,
            inverse_transform: Matrix4::identity(),
            half_height,
            half_width,
            pixel_size,
        }
    }

    pub fn set_transform(&mut self, transform: &Matrix4) {
        self.inverse_transform = inverse4(transform);
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let x_offset = (x as f32 + 0.5_f32) * self.pixel_size;
        let y_offset = (y as f32 + 0.5_f32) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let pixel = &self.inverse_transform * &Tuple::point(world_x, world_y, -1_f32);
        let origin = &self.inverse_transform * &Tuple::point(0_f32, 0_f32, 0_f32);
        let direction = (pixel - origin).normalize();
        Ray { origin, direction }
    }
}

pub fn render(camera: &Camera, world: &World) -> Canvas {
    let mut canvas = Canvas::new(camera.hsize, camera.vsize);
    print!("{}[2J", 27 as char);
    print!("Progress: ");
    let total = camera.vsize*camera.hsize;
    for y in 0..camera.vsize {
        for x in 0..camera.hsize {
            let ray = camera.ray_for_pixel(x, y);
            let color = color_at(world, &ray, 5);
            canvas.set_pixel(x, y, color);            
        }
        print!("{esc}[1;11H", esc = 27 as char);
        print!("{}%", ((y*camera.hsize)*100/total) as u32);
    }
    print!("{esc}[1;11H", esc = 27 as char);
    print!("100%");
    canvas
}

#[test]
fn the_pixel_size_for_a_horizontal_canvas() {
    let camera = Camera::new(200, 125, PI / 2_f32);
    assert_eq!(camera.pixel_size, 0.01_f32);
}

#[test]
fn the_pixel_size_for_a_vertical_canvas() {
    let camera = Camera::new(125, 200, PI / 2_f32);
    assert_eq!(camera.pixel_size, 0.01_f32);
}

#[test]
fn constructing_a_ray_through_the_center_of_the_canvas() {
    let camera = Camera::new(201, 101, PI / 2_f32);
    let ray = camera.ray_for_pixel(100, 50);
    assert_eq!(ray.origin, Tuple::point(0_f32, 0_f32, 0_f32));
    assert_eq!(ray.direction, Tuple::vector(0_f32, 0_f32, -1_f32));
}

#[test]
fn constructing_a_ray_through_a_corner_of_the_canvas() {
    let camera = Camera::new(201, 101, PI / 2_f32);
    let ray = camera.ray_for_pixel(0, 0);
    assert_eq!(ray.origin, Tuple::point(0_f32, 0_f32, 0_f32));
    assert_eq!(
        ray.direction,
        Tuple::vector(0.66519_f32, 0.33259_f32, -0.66851_f32)
    );
}

#[test]
fn constructing_a_ray_when_the_camera_is_transformed() {
    let mut camera = Camera::new(201, 101, PI / 2_f32);
    let transformation = Matrix4::identity()
        .rotate_y(PI / 4_f32)
        .translate(0_f32, -2_f32, 5_f32);
    camera.set_transform(&transformation);
    let ray = camera.ray_for_pixel(100, 50);
    assert_eq!(ray.origin, Tuple::point(0_f32, 2_f32, -5_f32));
    assert_eq!(
        ray.direction,
        Tuple::vector(2_f32.sqrt() / 2_f32, 0_f32, -2_f32.sqrt() / 2_f32)
    );
}

#[test]
fn rendering_a_world_with_a_camera() {
    let default : (Sphere, Sphere) = World::default_spheres();
    let world: World = World::default(&default.0, &default.1);
    let mut camera = Camera::new(11, 11, PI/2_f32);    
    let from = Tuple::point(0_f32, 0_f32, -5_f32);
    let to = Tuple::point(0_f32, 0_f32, 0_f32);
    let up = Tuple::vector(0_f32, 1_f32, 0_f32);
    camera.set_transform(&view_transform(&from, &to, &up));

    let canvas = render(&camera, &world);
    assert_eq!(canvas.pixel_at(5, 5), Color::new(0.38066_f32, 0.47583_f32, 0.2855_f32));

}