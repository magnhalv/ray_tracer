use crate::Sphere;
use crate::pattern::StripePattern;
use crate::pattern::Pattern;
use crate::color::BLACK;
use crate::color::WHITE;
use crate::lighting;
use crate::PointLight;
use crate::Tuple;
use crate::Color;

pub struct Material {
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflective: f32,
    pub color: Color,    
    pub pattern: Option<Box<dyn Pattern>>
}

impl Material {
    pub fn default() -> Material {
        Material { ambient: 0.1_f32, diffuse: 0.9_f32, specular: 0.9_f32, shininess: 200_f32, reflective: 0_f32, color: Color::new(1_f32, 1_f32, 1_f32), pattern: None }
    }
}

#[test]
fn lighting_with_a_pattern_applied() {
    let mut material = Material::default();
    material.ambient = 1_f32;
    material.diffuse = 0_f32;
    material.specular = 0_f32;
    material.pattern = StripePattern::new_box(WHITE, BLACK);

    let eyev = Tuple::vector(0_f32, 0_f32, -1_f32);
    let normalv = Tuple::vector(0_f32, 0_f32, -1_f32);
    let light = PointLight::new(Tuple::point(0_f32, 0_f32, -10_f32), Color::new(1_f32, 1_f32, 1_f32));
    
    let c1 = lighting(&material, &Sphere::new(0), &light, &Tuple::point(0.9_f32, 0_f32, 0_f32), &eyev, &normalv, false);
    let c2 = lighting(&material, &Sphere::new(0), &light, &Tuple::point(1.1_f32, 0_f32, 0_f32), &eyev, &normalv, false);

    let color = match &material.pattern {
        Some(p) => p.color_at(&Tuple::point(0.9_f32, 0_f32, 0_f32)),
        None => material.color
    };    
    
    assert_eq!(c1, WHITE);
    assert_eq!(c2, BLACK);
}