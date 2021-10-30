use std::ops::{Add, Mul, Neg, Sub, Index, IndexMut};

#[derive(Debug)]
pub struct Matrix2 {
    values: [f32; 4]
}

impl Matrix2 {
    pub fn new(
        m11: f32, m12: f32, 
        m21: f32, m22: f32
    ) -> Matrix2 {        
        Matrix2 {
            values: [m11, m12, m21, m22]
        }
    }
}

impl Index<[usize; 2]> for Matrix2 {
    type Output = f32;
    fn index<'a>(&'a self, index: [usize; 2]) -> &'a f32 {            
        &self.values[(index[0]-1)*2 + index[1]-1]
    }
}

impl IndexMut<[usize; 2]> for Matrix2 {    
    fn index_mut<'a>(&'a mut self, index: [usize; 2]) -> &'a mut f32 {            
        self.values.index_mut((index[0]-1)*2 + index[1]-1)
    }
}

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Matrix2) -> bool {
        self.values.iter().zip(other.values.iter()).all(|(a, b)| a == b)
    }

    fn ne(&self, other: &Matrix2) -> bool {
        self.values.iter().zip(other.values.iter()).any(|(a, b)| a != b)
    }
}


#[derive(Debug)]
pub struct Matrix3 {
    values: [f32; 9]
}

impl Matrix3 {
    pub fn new(
        m11: f32, m12: f32, m13: f32,
        m21: f32, m22: f32, m23: f32,
        m31: f32, m32: f32, m33: f32
    ) -> Matrix3 {        
        Matrix3 {
            values: [m11, m12, m13, m21, m22, m23, m31, m32, m33]
        }
    }
}

impl Index<[usize; 2]> for Matrix3 {
    type Output = f32;
    fn index<'a>(&'a self, index: [usize; 2]) -> &'a f32 {            
        &self.values[(index[0]-1)*3 + index[1]-1]
    }
}

impl IndexMut<[usize; 2]> for Matrix3 {    
    fn index_mut<'a>(&'a mut self, index: [usize; 2]) -> &'a mut f32 {            
        self.values.index_mut((index[0]-1)*3 + index[1]-1)
    }
}

impl PartialEq for Matrix3 {
    fn eq(&self, other: &Matrix3) -> bool {
        self.values.iter().zip(other.values.iter()).all(|(a, b)| a == b)
    }

    fn ne(&self, other: &Matrix3) -> bool {
        self.values.iter().zip(other.values.iter()).any(|(a, b)| a != b)
    }
}


#[derive(Debug)]
pub struct Matrix4 {
    values: [f32; 16]
}

impl Matrix4 {
    pub fn new_empty() -> Matrix4 {
        Matrix4 {
            values: [0_f32; 16]
        }
    }

    pub fn new(
        m11: f32, m12: f32, m13: f32, m14: f32,
        m21: f32, m22: f32, m23: f32, m24: f32,
        m31: f32, m32: f32, m33: f32, m34: f32,
        m41: f32, m42: f32, m43: f32, m44: f32,
    ) -> Matrix4 {        
        Matrix4 {
            values: [m11, m12, m13, m14, m21, m22, m23, m24, m31, m32, m33, m34, m41, m42, m43, m44]
        }
    }
}

impl Index<[usize; 2]> for Matrix4 {
    type Output = f32;
    fn index<'a>(&'a self, index: [usize; 2]) -> &'a f32 {            
        &self.values[(index[0]-1)*4 + index[1]-1]
    }
}

impl IndexMut<[usize; 2]> for Matrix4 {    
    fn index_mut<'a>(&'a mut self, index: [usize; 2]) -> &'a mut f32 {            
        self.values.index_mut((index[0]-1)*4 + index[1]-1)
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Matrix4) -> bool {
        self.values.iter().zip(other.values.iter()).all(|(a, b)| a == b)
    }

    fn ne(&self, other: &Matrix4) -> bool {
        self.values.iter().zip(other.values.iter()).any(|(a, b)| a != b)
    }
}


impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Matrix4 {
        let mut result = Matrix4::new_empty();
        for i in 1..5 {
            for j in 1..5 {
                let mut dot_product = 0_f32;
                for x in 1..5 {
                    dot_product += self[[i, x]] * other[[x, j]]
                }
                result[[i, j]] = dot_product;
            }
        }
        result
    }
}

fn transpose(m: &Matrix4) -> Matrix4 {
    let mut result = Matrix4::new_empty();
    for i in 1..5 {
        for j in 1..5 {
            result[[j, i]] = m[[i, j]];
        }
    }
    result
}

#[test]
fn init_matrix() {
    let matrix = Matrix4::new_empty();
    assert_eq!(matrix[[1, 1]], 0_f32);
}

#[test]
fn init_2x2_matrix() {
    let matrix = Matrix2::new(
        1_f32, 2_f32, 
        3_f32, 4_f32
    );
    assert_eq!(matrix[[1, 1]], 1_f32);
    assert_eq!(matrix[[1, 2]], 2_f32);
    assert_eq!(matrix[[2, 1]], 3_f32);
    assert_eq!(matrix[[2, 2]], 4_f32);    
}

#[test]
fn matrix_equals() {
    let matrix1 = Matrix2::new(
        1_f32, 2_f32, 
        3_f32, 4_f32
    );

    let matrix2 = Matrix2::new(
        1_f32, 2_f32, 
        3_f32, 4_f32
    );

    let matrix3 = Matrix3::new(
        1_f32, 2_f32, 3_f32,
        4_f32, 5_f32, 6_f32,
        7_f32, 8_f32, 9_f32
    );

    let matrix4 = Matrix3::new(
        1_f32, 2_f32, 3_f32,
        4_f32, 5_f32, 6_f32,
        7_f32, 8_f32, 9_f32
    );
    

    assert_eq!(matrix1, matrix2);
    assert_eq!(matrix3, matrix4);
}

#[test]
fn matrix_multiplication() {
    let matrix1 = Matrix4::new(
        1_f32, 2_f32, 3_f32, 4_f32,
        5_f32, 6_f32, 7_f32, 8_f32,
        9_f32, 8_f32, 7_f32, 6_f32,
        5_f32, 4_f32, 3_f32, 2_f32,
    );

    let matrix2 = Matrix4::new(
        -2_f32, 1_f32, 2_f32, 3_f32,
        3_f32, 2_f32, 1_f32, -1_f32,
        4_f32, 3_f32, 6_f32, 5_f32,
        1_f32, 2_f32, 7_f32, 8_f32,
    );
    

    let expected = Matrix4::new(
        20_f32, 22_f32, 50_f32, 48_f32,
        44_f32, 54_f32, 114_f32, 108_f32,
        40_f32, 58_f32, 110_f32, 102_f32,
        16_f32, 26_f32, 46_f32, 42_f32,
    );

    let result = matrix1 * matrix2;
    assert_eq!(expected, result);    
}

#[test]
fn matrix_transpose() {
    let matrix = Matrix4::new(
        0_f32, 9_f32, 1_f32, 0_f32,
        9_f32, 8_f32, 0_f32, 8_f32,
        1_f32, 8_f32, 5_f32, 3_f32,
        0_f32, 0_f32, 5_f32, 8_f32,
    );
    

    let expected = Matrix4::new(
        0_f32, 9_f32, 1_f32, 0_f32,
        9_f32, 8_f32, 8_f32, 0_f32,
        1_f32, 0_f32, 5_f32, 5_f32,
        0_f32, 8_f32, 3_f32, 8_f32,
    ); 
    
    let result = transpose(&matrix);
    assert_eq!(expected, result)
}