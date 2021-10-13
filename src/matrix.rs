use std::ops::{Add, Mul, Neg, Sub, Index, IndexMut};

#[derive(Debug)]
struct Matrix {
    x: usize,
    y: usize,
    values: Box<[f64]>
}

impl Matrix {
    pub fn new(x: usize, y: usize) -> Matrix {                
        Matrix {
            x, y,
            values: vec![unsafe { std::mem::zeroed() }; x*y].into_boxed_slice()
        }
    }    

    pub fn new2x2(
        m11: f64, m12: f64, 
        m21: f64, m22: f64
    ) -> Matrix {
        let size = 2;
        Matrix {
            x: size, 
            y: size,
            values: vec![m11, m12, m21, m22].into_boxed_slice()
        }
    }

    pub fn new3x3(
        m11: f64, m12: f64, m13: f64,
        m21: f64, m22: f64, m23: f64,
        m31: f64, m32: f64, m33: f64
    ) -> Matrix {
        let size = 3;
        Matrix {
            x: size, 
            y: size,
            values: vec![m11, m12, m13, m21, m22, m23, m31, m32, m33].into_boxed_slice()
        }
    }

    pub fn new4x4(
        m11: f64, m12: f64, m13: f64, m14: f64,
        m21: f64, m22: f64, m23: f64, m24: f64,
        m31: f64, m32: f64, m33: f64, m34: f64,
        m41: f64, m42: f64, m43: f64, m44: f64,
    ) -> Matrix {
        let size = 4;
        Matrix {
            x: size, 
            y: size,
            values: vec![m11, m12, m13, m14, m21, m22, m23, m24, m31, m32, m33, m34, m41, m42, m43, m44].into_boxed_slice()
        }
    }
    
}

impl Index<[usize; 2]> for Matrix {
    type Output = f64;
    fn index<'a>(&'a self, index: [usize; 2]) -> &'a f64 {            
        debug_assert!(
            (index[0] >= 1 && index[1] >= 1) && (index[0] <= self.y && index[1] <= self.x), 
            "Out of range matrix {0}x{1} access attempted! [{2}, {3}]", self.y, self.x, index[0], index[1]);
        &self.values[(index[0]-1)*self.y + index[1]-1]
    }
}

impl IndexMut<[usize; 2]> for Matrix {
    
    fn index_mut<'a>(&'a mut self, index: [usize; 2]) -> &'a mut f64 {            
        debug_assert!(
            (index[0] >= 1 && index[1] >= 1) && (index[0] <= self.y && index[1] <= self.x), 
            "Out of range matrix {0}x{1} access attempted! [{2}, {3}]", self.y, self.x, index[0], index[1]);
        self.values.index_mut((index[0]-1)*self.y + index[1]-1)
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        let size = self.x * self.y;
        let other_size = other.x * other.y;
        size == other_size && self.values.iter().zip(other.values.iter()).all(|(a, b)| a == b)
    }

    fn ne(&self, other: &Matrix) -> bool {
        let size = self.x * self.y;
        let other_size = other.x * other.y;
        size != other_size || self.values.iter().zip(other.values.iter()).any(|(a, b)| a != b)
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix {
        let mut result = Matrix::new(self.y, other.x);
        for i in 1..self.y+1 {
            for j in 1..other.x+1 {
                let mut dot_product = 0_f64;
                for x in 1..self.x+1 {
                    dot_product += self[[i, x]] * other[[x, j]]
                }
                result[[i, j]] = dot_product;
            }
        }
        result
    }
}

#[test]
fn init_matrix() {
    let matrix = Matrix::new(4, 4);    
    assert_eq!(matrix[[1, 1]], 0_f64);
}

#[test]
fn init_2x2_matrix() {
    let matrix = Matrix::new2x2(
        1_f64, 2_f64, 
        3_f64, 4_f64
    );
    assert_eq!(matrix[[1, 1]], 1_f64);
    assert_eq!(matrix[[1, 2]], 2_f64);
    assert_eq!(matrix[[2, 1]], 3_f64);
    assert_eq!(matrix[[2, 2]], 4_f64);    
}

#[test]
fn matrix_equals() {
    let matrix1 = Matrix::new2x2(
        1_f64, 2_f64, 
        3_f64, 4_f64
    );

    let matrix2 = Matrix::new2x2(
        1_f64, 2_f64, 
        3_f64, 4_f64
    );

    let matrix3 = Matrix::new3x3(
        1_f64, 2_f64, 3_f64,
        4_f64, 5_f64, 6_f64,
        7_f64, 8_f64, 9_f64
    );

    let matrix4 = Matrix::new3x3(
        1_f64, 2_f64, 3_f64,
        4_f64, 5_f64, 6_f64,
        7_f64, 8_f64, 9_f64
    );
    

    assert_eq!(matrix1, matrix2);
    assert_eq!(matrix3, matrix4);
}

#[test]
fn matrix_multiplication() {
    let matrix1 = Matrix::new4x4(
        1_f64, 2_f64, 3_f64, 4_f64,
        5_f64, 6_f64, 7_f64, 8_f64,
        9_f64, 8_f64, 7_f64, 6_f64,
        5_f64, 4_f64, 3_f64, 2_f64,
    );

    let matrix2 = Matrix::new4x4(
        -2_f64, 1_f64, 2_f64, 3_f64,
        3_f64, 2_f64, 1_f64, -1_f64,
        4_f64, 3_f64, 6_f64, 5_f64,
        1_f64, 2_f64, 7_f64, 8_f64,
    );
    

    let expected = Matrix::new4x4(
        20_f64, 22_f64, 50_f64, 48_f64,
        44_f64, 54_f64, 114_f64, 108_f64,
        40_f64, 58_f64, 110_f64, 102_f64,
        16_f64, 26_f64, 46_f64, 42_f64,
    );

    let result = matrix1 * matrix2;
    assert_eq!(expected, result);    
}