use crate::material::Material;
use crate::ray::Intersection;
use crate::Matrix4;
use crate::Ray;
use crate::Tuple;

pub trait Shape: Sync + Send {
    fn get_id(&self) -> u32;
    fn set_transformation(&mut self, t: Matrix4);
    fn get_inverse_transformation(&self) -> &Matrix4;
    fn normal_at(&self, world_point: &Tuple) -> Tuple;
    fn get_material(&self) -> &Material;
    fn get_mut_material(&mut self) -> &mut Material;
    fn intersections_by<'a>(&'a self, ray: &Ray) -> Vec<Intersection<'a>>;
}

impl PartialEq for dyn Shape {
    fn eq(&self, rhs: &dyn Shape) -> bool {
        self.get_id() == rhs.get_id()
    }
}
