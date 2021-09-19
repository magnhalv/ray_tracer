mod geometry;
mod color;

fn main() {
    let mut position = geometry::Point::new(0.0_f64, 1.0_f64, 0.0_f64);
    let mut velocity = geometry::Vector::new(1.0_f64, 1.0_f64, 1.0_f64).normalize();

    let gravity = geometry::Vector::new(0.0_f64, -0.1_f64, 0.0_f64);
    let wind = geometry::Vector::new(-0.01_f64, 0.0_f64, 0.0_f64);

    while position.y > 0_f64 {
        let (new_position, new_velocity) = tick(position, velocity, gravity, wind);
        position = new_position;
        velocity = new_velocity;

        println!("{0}", position)
    }
}

fn tick(position: geometry::Point, velocity: geometry::Vector, gravity: geometry::Vector, wind: geometry::Vector) -> (geometry::Point, geometry::Vector) {
    let resulting_position = position + velocity;
    let resulting_velocity = velocity + gravity + wind;
    (resulting_position, resulting_velocity)
}


