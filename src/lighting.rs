use crate::color::Color;
use crate::tuple::{Tuple, reflect};

pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color
}

pub struct Material {
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub color: Color
}

impl Material {
    pub fn default() -> Material {
        Material { ambient: 0.1_f32, diffuse: 0.9_f32, specular: 0.9_f32, shininess: 200_f32, color: Color::new(1_f32, 1_f32, 1_f32) }
    }
}

pub fn lighting(material: &Material, light: &PointLight, eye_pos: &Tuple, eye_dir: &Tuple, &surface_normal: &Tuple) -> Color {
    let effective_color = material.color * light.intensity;
    let light_vector = (light.position - *eye_pos).normalize();

    let ambient = effective_color * material.ambient;

    let light_dot_normal = light_vector.dot(&surface_normal);
    
    let black = Color::new(0_f32, 0_f32, 0_f32);
    let diffuse : Color;
    let specular: Color;
    if light_dot_normal < 0_f32 {
        diffuse = black;
        specular = black;
    }
    else {
        diffuse = effective_color * material.diffuse * light_dot_normal;
        let reflect_v = reflect(&(-light_vector), &surface_normal);
        let reflect_dot_eye = reflect_v.dot(&eye_dir);

        if reflect_dot_eye <= 0_f32 {
            specular = black;
        }
        else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }

    ambient + diffuse + specular    
}

#[test]
fn lighting_with_the_eye_between_the_light_and_the_surface() {
    let material = Material::default();
    let position = Tuple::new_point(0_f32, 0_f32, 0_f32);


    let eye_vector = Tuple::new_vector(0_f32, 0_f32, -1_f32);
    let normal = Tuple::new_vector(0_f32, 0_f32, -1_f32);
    let light = PointLight {
        position: Tuple::new_point(0_f32, 0_f32, -10_f32),
        intensity: Color::new(1_f32, 1_f32, 1_f32)
    };

    let result = lighting(&material, &light, &position, &eye_vector, &normal);    
    assert_eq!(result, Color::new(1.9_f32, 1.9_f32, 1.9_f32));
}

#[test]
fn lighting_with_the_eye_between_light_and_the_surface_and_45deg_on_surface() {
    let material = Material::default();
    let position = Tuple::new_point(0_f32, 0_f32, 0_f32);


    let eye_vector = Tuple::new_vector(0_f32, 2_f32.sqrt()/2_f32, -2_f32.sqrt()/2_f32);
    let normal = Tuple::new_vector(0_f32, 0_f32, -1_f32);
    let light = PointLight {
        position: Tuple::new_point(0_f32, 0_f32, -10_f32),
        intensity: Color::new(1.0_f32, 1.0_f32, 1.0_f32)
    };

    let result = lighting(&material, &light, &position, &eye_vector, &normal);
    assert_eq!(result, Color::new(1_f32, 1_f32, 1_f32));
}

#[test]
fn lighting_with_the_eye_oppsite_surface_light_offset_45deg() {
    let material = Material::default();
    let position = Tuple::new_point(0_f32, 0_f32, 0_f32);


    let eye_vector = Tuple::new_vector(0_f32, 0_f32, -1_f32);
    let normal = Tuple::new_vector(0_f32, 0_f32, -1_f32);
    let light = PointLight {
        position: Tuple::new_point(0_f32, 10_f32, -10_f32),
        intensity: Color::new(1.0_f32, 1.0_f32, 1.0_f32)
    };

    let result = lighting(&material, &light, &position, &eye_vector, &normal);
    assert_eq!(result, Color::new(0.7364_f32, 0.7364_f32, 0.7364_f32));
}

#[test]
fn lighting_with_the_eye_in_the_path_of_the_reflection_vector() {
    let material = Material::default();
    let position = Tuple::new_point(0_f32, 0_f32, 0_f32);


    let eye_vector = Tuple::new_vector(0_f32, -(2_f32.sqrt()/2_f32), -2_f32.sqrt()/2_f32);
    let normal = Tuple::new_vector(0_f32, 0_f32, -1_f32);
    let light = PointLight {
        position: Tuple::new_point(0_f32, 10_f32, -10_f32),
        intensity: Color::new(1.0_f32, 1.0_f32, 1.0_f32)
    };

    let result = lighting(&material, &light, &position, &eye_vector, &normal);
    assert_eq!(result, Color::new(1.63638_f32, 1.63638_f32, 1.63638_f32));
}

#[test]
fn lighting_with_the_light_behind_the_surface() {
    let material = Material::default();
    let position = Tuple::new_point(0_f32, 0_f32, 0_f32);


    let eye_vector = Tuple::new_vector(0_f32, 0_f32, -1_f32);
    let normal = Tuple::new_vector(0_f32, 0_f32, -1_f32);
    let light = PointLight {
        position: Tuple::new_point(0_f32, 0_f32, 10_f32),
        intensity: Color::new(1_f32, 1_f32, 1_f32)
    };

    let result = lighting(&material, &light, &position, &eye_vector, &normal);    
    assert_eq!(result, Color::new(0.1_f32, 0.1_f32, 0.1_f32));
}