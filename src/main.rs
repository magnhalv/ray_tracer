mod geometry;
mod color;
mod canvas;
mod matrix;

fn main() {
    let mut position = geometry::Point::new(0.0_f64, 1.0_f64, 0.0_f64);
    let mut velocity = geometry::Vector::new(1.0_f64, 1.8_f64, 0.0_f64).normalize() * 11.25;

    let gravity = geometry::Vector::new(0.0_f64, -0.1_f64, 0.0_f64);
    let wind = geometry::Vector::new(-0.01_f64, 0.0_f64, 0.0_f64);

    let mut canvas = canvas::Canvas::new(1200, 550);
    let color = color::Color::new(1.0_f64, 0_f64, 0_f64);

    while position.y > 0_f64 {
        let height = canvas.height;
        canvas::set_pixel(&mut canvas, position.x as usize, height - position.y as usize, color);

        let (new_position, new_velocity) = tick(position, velocity, gravity, wind);
        position = new_position;
        velocity = new_velocity;        
    }

    canvas::canvas_to_file(&canvas, "test.ppm".to_string());
}

fn tick(position: geometry::Point, velocity: geometry::Vector, gravity: geometry::Vector, wind: geometry::Vector) -> (geometry::Point, geometry::Vector) {
    let resulting_position = position + velocity;
    let resulting_velocity = velocity + gravity + wind;
    (resulting_position, resulting_velocity)
}


