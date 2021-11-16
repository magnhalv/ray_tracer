use crate::Tuple;
use crate::lighting::Material;
use crate::Matrix4;

trait Shape {
    fn set_transformation(&mut self, transformation: Matrix4);
    fn get_material(&self) -> &Material;
}

struct Square {
    id: usize, 
    material: Material,
    transformation: Matrix4
}

impl Shape for Square {
    fn set_transformation(&mut self, transformation: Matrix4) {
        self.transformation = transformation;
    }
    
    fn get_material(&self) -> &Material {
        &self.material
    }
}

struct Sphere {
    id: usize, 
    material: Material,
    transformation: Matrix4
}


fn test() {
    let mut s = Square {
        id: 5,
        material: Material::default(),
        transformation: Matrix4::identity()
    };
    let mut vec : Vec<&mut Shape> = Vec::new();
    vec.push(&mut s);

    vec[0].set_transformation(Matrix4::identity());
}



