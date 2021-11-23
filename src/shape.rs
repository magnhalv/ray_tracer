use crate::ray::Intersection;
use crate::Ray;
use crate::material::Material;
use crate::Tuple;
use crate::Matrix4;

pub trait Shape {
    fn get_id(&self) -> u32;
    fn set_transformation(&mut self, t: Matrix4);  
    fn get_inverse_transformation(&self) -> &Matrix4;
    fn normal_at(&self, world_point: &Tuple) -> Tuple;
    fn get_material(&self) -> &Material;
    fn intersections_by<'a>(&'a self, ray: &Ray) -> Vec<Intersection<'a>>;
}