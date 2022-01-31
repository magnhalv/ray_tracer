use crate::color::{Color, BLACK, WHITE};
use crate::cube::Cube;
use crate::lighting::PointLight;
use crate::matrix::Matrix4;
use crate::pattern::CheckerPattern;
use crate::pattern::GradientPattern;
use crate::pattern::Pattern;
use crate::plane::Plane;
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::tuple::Tuple;
use crate::world::World;
use core::f32::consts::PI;

const red: Color = Color {
    red: 1.0,
    green: 0.0,
    blue: 0.0,
};
const pink: Color = Color {
    red: 1.0,
    green: 0.0,
    blue: 1.0,
};

pub fn generate_world() -> World {
    let bone_color = Color::new(205.0 / 255.0, 197.0 / 255.0, 180.0 / 255.0);
    let liliac_color = Color::new(181.0 / 255.0, 157.0 / 255.0, 164.0 / 255.0);
    let metalic_color = Color::new(133.0 / 255.0, 117.0 / 255.0, 110.0 / 255.0);
    let sepia_color = Color::new(109.0 / 255.0, 61.0 / 255.0, 20.0 / 255.0);
    let dark_red_color = Color::new(85.0 / 255.0, 27.0 / 255.0, 20.0 / 255.0);
    let platinum = Color::new(227.0 / 255.0, 227.0 / 255.0, 227.0 / 255.0);
    let glaucous = Color::new(104.0 / 255.0, 131.0 / 255.0, 186.0 / 255.0);

    let mut wall = Plane::new(1);
    wall.set_transformation(
        Matrix4::identity()
            .scale(10.0, 10.0, 1.0)
            .translate(0.0, 0.0, 100.0)
            .rotate_x(-PI / 2_f32)

    );
    wall.material.color = metalic_color;
    wall.material.specular = 0.3_f32;
    wall.material.diffuse = 0.7_f32;
    wall.material.reflective = 1.0;

    /* Floor  */
    let mut floor = Plane::new(2);
    //floor.set_transformation(Matrix4::identity().(10_f32, 0.01_f32, 10_f32));
    floor.material.color = metalic_color;
    let mut floor_pattern = CheckerPattern {
        first: platinum,
        second: glaucous,
        inverse_transformation: Matrix4::identity(),
    };

    floor_pattern.set_transform(&Matrix4::identity().scale(10_f32, 10_f32, 10_f32));
    floor.material.pattern = Some(Box::new(floor_pattern));
    floor.material.specular = 0.3_f32;
    floor.material.diffuse = 0.7_f32;
    floor.material.reflective = 0.2;

    /* Middle  */
    let mut middle = Sphere::new(3);
    middle.set_transformation(
        Matrix4::identity()
            .translate(0_f32, 15_f32, 30_f32)
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
            .translate(50_f32, 10_f32, 50_f32)
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

    let mut left = Cube::new(5);
    left.set_transformation(
        Matrix4::identity()
            .translate(0_f32, 5_f32, -5.5_f32)
            .scale(5_f32, 5_f32, 5_f32),
    );
    left.material.color = sepia_color;
    //left.material.ambient = 1.0_f32;
    //left.material.diffuse = 0.7_f32;
    //left.material.specular = 0.3_f32;
    left.material.shininess = 300.0;
    //left.material.reflective = 0.2_f32;
    //left.material.transparency = 0.0_f32;
    /* let mut left_pattern = RingPattern {
        first: Color::new(0_f32, 0.4_f32, 0.0_f32),
        second: Color::new(0.4_f32, 0.8_f32, 0.4_f32),
        inverse_transformation: Matrix4::identity()
    };
    left_pattern.set_transform(&Matrix4::identity().translate(-1_f32, -1_f32, -1_f32).scale(0.2_f32, 0.2_f32, 0.2_f32));
    left.material.pattern = Some(Box::new(left_pattern)); */

    // REST
    let light = PointLight::new(
        Tuple::point(0_f32, 200_f32, -100_f32),
        Color::new(1.0, 1.0, 1.0),
    );
    let mut world = World::new(light);
    world.objects.push(Box::new(wall));
    world.objects.push(Box::new(floor));
    world.objects.push(Box::new(middle));
    world.objects.push(Box::new(left));
    world.objects.push(Box::new(right));
    return world;
}

pub fn generate_test_world() -> World {
    let mut wall = Plane::new(1);
    wall.set_transformation(
        Matrix4::identity()
            .scale(10.0, 10.0, 1.0)
            .translate(0.0, -20.0, 100.0)
            .rotate_x(PI / 2_f32),
    );
    wall.material.color = Color::new(1.0, 0.0, 0.0);
    wall.material.diffuse = 0.1;
    wall.material.specular = 1.0;
    wall.material.ambient = 0.1;
    wall.material.reflective = 0.9;
    wall.material.shininess = 300.0;
    let mut wall_pattern = CheckerPattern {
        first: red,
        second: pink,
        inverse_transformation: Matrix4::identity(),
    };

    wall_pattern.set_transform(&Matrix4::identity().scale(1_f32, 1_f32, 1_f32));
    wall.material.pattern = Some(Box::new(wall_pattern));

    /* Floor  */
    let mut floor = Plane::new(2);
    floor.material.color = Color::new(0.0, 1.0, 0.0);
    floor.material.specular = 0.3_f32;
    floor.material.diffuse = 0.7_f32;
    floor.material.reflective = 0.2;
    floor.set_transformation(Matrix4::identity().scale(5_f32, 5_f32, 5_f32));

    let mut left = Cube::new(5);
    left.set_transformation(
        Matrix4::identity()
            .translate(0_f32, 5_f32, 0_f32)
            .scale(10_f32, 10_f32, 5_f32),
    );
    left.material.color = Color::new(0.0, 0.0, 1.0);
    left.material.diffuse = 0.7_f32;
    left.material.specular = 0.3_f32;
    left.material.shininess = 300.0;
    left.material.reflective = 0.2_f32;

    let light = PointLight::new(
        Tuple::point(-100_f32, 200_f32, -100_f32),
        Color::new(1.0, 1.0, 1.0),
    );
    let mut world = World::new(light);
    world.objects.push(Box::new(wall));
    world.objects.push(Box::new(floor));
    world.objects.push(Box::new(left));

    return world;
}

#[cfg(test)]
mod tests {
    use crate::camera::{render_at, Camera};
    use crate::color::Color;
    use crate::math::PI;
    use crate::transformation::view_transform;
    use crate::tuple::Tuple;
    use crate::world_generator::generate_world;

    const DIM_X: usize = 800;
    const DIM_Y: usize = 540;
    #[test]
    fn test_basic_world() {
        let world = generate_world();
        let mut camera = Camera::new(DIM_X, DIM_Y, PI / 3_f32);
        let from = Tuple::point(25_f32, 10_f32, -30_f32);
        let to = Tuple::point(0_f32, 1_f32, 50_f32);
        let up = Tuple::vector(0_f32, 1_f32, 0_f32);
        camera.set_transform(&view_transform(&from, &to, &up));
        let color = render_at(38, 233, &camera, &world);
        assert_eq!(color, Color::new(0.124647096, 0.02392157, 0.007843138));
        let color2 = render_at(32, 212, &camera, &world);
        assert_eq!(color2, Color::new(0.0627451, 0.02392157,0.007843138));
    }
}
