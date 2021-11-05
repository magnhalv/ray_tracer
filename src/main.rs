mod geometry;
mod color;
mod canvas;
mod matrix;
mod transformation;

fn main() {
    let mut position = geometry::Tuple::new_point(0.0_f32, 1.0_f32, 0.0_f32);
    let mut velocity = geometry::Tuple::new_vector(1.0_f32, 1.8_f32, 0.0_f32).normalize() * 11.25;

    let gravity = geometry::Tuple::new_vector(0.0_f32, -0.1_f32, 0.0_f32);
    let wind = geometry::Tuple::new_vector(-0.01_f32, 0.0_f32, 0.0_f32);

    let mut canvas = canvas::Canvas::new(1200, 550);
    let color = color::Color::new(1.0_f32, 0_f32, 0_f32);

    while position.y > 0_f32 {
        let height = canvas.height;
        canvas::set_pixel(&mut canvas, position.x as usize, height - position.y as usize, color);

        let (new_position, new_velocity) = tick(position, velocity, gravity, wind);
        position = new_position;
        velocity = new_velocity;        
    }

    canvas::canvas_to_file(&canvas, "test.ppm".to_string());
}

fn tick(position: geometry::Tuple, velocity: geometry::Tuple, gravity: geometry::Tuple, wind: geometry::Tuple) -> (geometry::Tuple, geometry::Tuple) {
    let resulting_position = position + velocity;
    let resulting_velocity = velocity + gravity + wind;
    (resulting_position, resulting_velocity)
}


