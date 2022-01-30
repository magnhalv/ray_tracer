use crate::color::{BLACK, WHITE, Color};
use crate::matrix::Matrix4;
use crate::sphere::Sphere;
use crate::tuple::Tuple;
use crate::lighting::PointLight;
use crate::pattern::CheckerPattern;
use crate::pattern::GradientPattern;
use crate::pattern::Pattern;
use crate::plane::Plane;
use crate::shape::Shape;
use crate::world::World;
use core::f32::consts::PI;

pub fn generate_world() -> World {
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
    let mut floor_pattern = CheckerPattern {
        first: WHITE,
        second: BLACK,
        inverse_transformation: Matrix4::identity(),
    };

    floor_pattern.set_transform(&Matrix4::identity().scale(1_f32, 1_f32, 1_f32));
    floor.material.pattern = Some(Box::new(floor_pattern));

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
    world.objects.push(Box::new(wall));
    world.objects.push(Box::new(floor));
    world.objects.push(Box::new(middle));
    world.objects.push(Box::new(left));
    world.objects.push(Box::new(right));
    return world;
}