use crate::matrix::{{Matrix4, inverse4}};

pub struct Sphere {
    pub id: u32,
    pub transformation: Matrix4,
    pub inverse_transformation: Matrix4
}

impl Sphere {
    pub fn new(id: u32) -> Sphere {
        Sphere { id, transformation: Matrix4::identity(), inverse_transformation: Matrix4::identity() }
    }

    pub fn set_transformation(&mut self, t: Matrix4) {
        self.transformation = t;
        self.inverse_transformation = inverse4(&self.transformation)
    }
}