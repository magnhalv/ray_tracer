use std::ops::{Mul, Index, IndexMut};
use crate::geometry::{Tuple};

// TODO: At later time, look into how matrices are constructed. E.g. would it be worthwile to send in a pointer to put resulting matrices.

#[derive(Debug)]
pub struct Matrix2 {
    values: [f32; 4]
}

impl Matrix2 {
    pub fn new_empty() -> Matrix2 {
        Matrix2 {
            values: [0_f32; 4]
        }
    }

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
        &self.values[(index[0]*2) + index[1]]
    }
}

impl IndexMut<[usize; 2]> for Matrix2 {    
    fn index_mut<'a>(&'a mut self, index: [usize; 2]) -> &'a mut f32 {            
        self.values.index_mut((index[0]*2) + index[1])
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

fn determinant2(m: &Matrix2) -> f32 {
    m[[0,0]]*m[[1, 1]] - m[[0,1]]*m[[1,0]]
}


#[derive(Debug)]
pub struct Matrix3 {
    values: [f32; 9]
}

impl Matrix3 {
    pub fn new_empty() -> Matrix3 {
        Matrix3 {
            values: [0_f32; 9]
        }
    }

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
        &self.values[(index[0]*3) + index[1]]
    }
}

impl IndexMut<[usize; 2]> for Matrix3 {    
    fn index_mut<'a>(&'a mut self, index: [usize; 2]) -> &'a mut f32 {            
        self.values.index_mut((index[0]*3) + index[1])
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

fn submatrix3(m: &Matrix3, skip_i: usize, skip_j: usize) -> Matrix2 {
    let mut result = Matrix2::new_empty();
    let mut new_i = 0;
    let mut new_j = 0;
    
    for i in 0..3 {
        if i == skip_i {
            continue;
        }
        for j in 0..3 {
            if j == skip_j {
                continue;
            }
            result[[new_i, new_j]] = m[[i, j]];
            new_j += 1;
        }
        new_i += 1;
        new_j = 0;
    }
    result
}

fn minor3(m: &Matrix3, i: usize, j: usize) -> f32 {
    let submatrix = submatrix3(m, i, j);
    determinant2(&submatrix)
}

fn cofactor3(m: &Matrix3, i: usize, j: usize) -> f32 {
    let minor = minor3(m, i, j);
    if ((i + j) % 2) == 0 {
        return minor
    }
    -minor
}

fn determinant3(m: &Matrix3) -> f32 {
    let mut result = 0_f32;
    for j in 0..3 {
        let cofactor = cofactor3(m, 0, j);
        result += m[[0, j]] * cofactor;
    }
    result
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
        &self.values[(index[0])*4 + index[1]]
    }
}

impl IndexMut<[usize; 2]> for Matrix4 {    
    fn index_mut<'a>(&'a mut self, index: [usize; 2]) -> &'a mut f32 {            
        self.values.index_mut((index[0])*4 + index[1])
    }
}

impl PartialEq for Matrix4 {
    

    fn eq(&self, other: &Matrix4) -> bool {
        let epsilon = 0.0001_f32;
        self.values.iter().zip(other.values.iter()).all(|(a, b)| (a - b).abs() < epsilon)
    }

    fn ne(&self, other: &Matrix4) -> bool {
        let epsilon = 0.0001_f32;
        self.values.iter().zip(other.values.iter()).any(|(a, b)| (a - b).abs() > epsilon)
    }
}

impl <'a, 'b> Mul<&'b Matrix4> for &'a Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: &'b Matrix4) -> Matrix4 {
        // TODO: SIMD this?
        let mut result = Matrix4::new_empty();
        for i in 0..4 {
            for j in 0..4 {
                let mut dot_product = 0_f32;
                for x in 0..4 {
                    dot_product += self[[i, x]] * other[[x, j]]
                }
                result[[i, j]] = dot_product;
            }
        }
        result
    }
}

impl <'a, 'b> Mul<&'b Tuple> for &'a Matrix4 {
    type Output = Tuple;

    fn mul(self, other: &'b Tuple) -> Tuple {
        // TODO: SIMD this?        
       let x = self[[0, 0]] * other.x + self[[0, 1]] * other.y + self[[0, 2]] * other.z + self[[0, 3]] * other.w;
       let y = self[[1, 0]] * other.x + self[[1, 1]] * other.y + self[[1, 2]] * other.z + self[[1, 3]] * other.w;
       let z = self[[2, 0]] * other.x + self[[2, 1]] * other.y + self[[2, 2]] * other.z + self[[2, 3]] * other.w;
       let w = self[[3, 0]] * other.x + self[[3, 1]] * other.y + self[[3, 2]] * other.z + self[[3, 3]] * other.w;
       let result = Tuple::new(x, y, z, w);
       result
    }
}

fn transpose4(m: &Matrix4) -> Matrix4 {
    let mut result = Matrix4::new_empty();
    for i in 0..4 {
        for j in 0..4 {
            result[[j, i]] = m[[i, j]];
        }
    }
    result
}

fn submatrix4(m: &Matrix4, skip_i: usize, skip_j: usize) -> Matrix3 {
    let mut result = Matrix3::new_empty();
    let mut new_i = 0;
    let mut new_j = 0;
    
    for i in 0..4 {
        if i == skip_i {
            continue;
        }
        for j in 0..4 {
            if j == skip_j {
                continue;
            }
            result[[new_i, new_j]] = m[[i, j]];
            new_j += 1;
        }
        new_i += 1;
        new_j = 0;
    }
    result
}

fn minor4(m: &Matrix4, i: usize, j: usize) -> f32 {
    let submatrix = submatrix4(m, i, j);
    determinant3(&submatrix)
}

fn cofactor4(m: &Matrix4, i: usize, j: usize) -> f32 {
    let minor = minor4(m, i, j);
    if ((i + j) % 2) == 0 {
        return minor
    }
    -minor
}
// + - + - - + - + + - + - - + - +
// + - + -
// - + - +
// + - + -
// - + - +

fn determinant4(m: &Matrix4) -> f32 {
    let mut result = 0_f32;
    for j in 0..4 {
        let cofactor = cofactor4(m, 0, j);
        result += m[[0, j]] * cofactor;
    }
    result
}

// TODO: Make this member function?
pub fn inverse4(m: &Matrix4) -> Matrix4 {
    let mut result = Matrix4::new_empty();
    let determinant = determinant4(&m);
    for i in 0..4 {
        for j in 0..4 {
            let cofactor = cofactor4(&m, i, j);
            result[[j, i]] = cofactor / determinant;
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
    assert_eq!(matrix[[0, 0]], 1_f32);
    assert_eq!(matrix[[0, 1]], 2_f32);
    assert_eq!(matrix[[1, 0]], 3_f32);
    assert_eq!(matrix[[1, 1]], 4_f32);    
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

    let result = &matrix1 * &matrix2;
    assert_eq!(expected, result);    
}

fn is_invertible(m: &Matrix4) -> bool {
    determinant4(&m) != 0_f32
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
    
    let result = transpose4(&matrix);
    assert_eq!(expected, result)
}

#[test]
fn matrix_determinant() {
    let matrix = Matrix2::new(
        1_f32, 5_f32,
        -3_f32, 2_f32
    );

    let result = determinant2(&matrix);
    assert_eq!(17_f32, result);
}
 
#[test]
fn matrix_submatrix() {
    let matrix1 = Matrix3::new(
        1_f32, 2_f32, 3_f32,
        4_f32, 5_f32, 6_f32,
        7_f32, 8_f32, 9_f32
    );

    let expected1 = Matrix2::new(
        4_f32, 6_f32,
        7_f32, 9_f32
    );

    assert_eq!(expected1, submatrix3(&matrix1, 0, 1)); 

    let matrix2 = Matrix4::new(
        1_f32, 2_f32, 3_f32, 4_f32, 
        5_f32, 6_f32, 7_f32, 8_f32, 
        9_f32, 10_f32, 11_f32, 12_f32,
        13_f32, 14_f32, 15_f32, 16_f32
    );

    let expected2 = Matrix3::new(        
        5_f32, 6_f32, 7_f32, 
        9_f32, 10_f32, 11_f32,
        13_f32, 14_f32, 15_f32,
    );

    assert_eq!(expected2, submatrix4(&matrix2, 0, 3));
} 

#[test]
fn matrix3_minor() {
    let m = Matrix3::new(        
        3_f32, 5_f32, 0_f32, 
        2_f32, -1_f32, -7_f32,
        6_f32, -1_f32, 5_f32,
    );

    assert_eq!(25_f32, minor3(&m, 1, 0));
}

#[test]
fn matrix3_cofactor3() {
    let m = Matrix3::new(        
        3_f32, 5_f32, 0_f32, 
        2_f32, -1_f32, -7_f32,
        6_f32, -1_f32, 5_f32,
    );

    assert_eq!(-12_f32, minor3(&m, 0, 0));
    assert_eq!(-12_f32, cofactor3(&m, 0, 0));
    assert_eq!(25_f32, minor3(&m, 1, 0));
    assert_eq!(-25_f32, cofactor3(&m, 1, 0));
}

#[test]
fn matrix3_determinant() {
    let m = Matrix3::new(        
        1_f32, 2_f32, 6_f32, 
        -5_f32, 8_f32, -4_f32,
        2_f32, 6_f32, 4_f32,
    );

    assert_eq!(56_f32, cofactor3(&m, 0, 0));
    assert_eq!(12_f32, cofactor3(&m, 0, 1));
    assert_eq!(-46_f32, cofactor3(&m, 0, 2));
    assert_eq!(-196_f32, determinant3(&m));
}

#[test]
fn matrix4_determinant() {
    let m = Matrix4::new(        
        -2_f32, -8_f32, 3_f32, 5_f32, 
        -3_f32, 1_f32, 7_f32, 3_f32,
        1_f32, 2_f32, -9_f32, 6_f32,
        -6_f32, 7_f32, 7_f32, -9_f32
    );

    assert_eq!(690_f32, cofactor4(&m, 0, 0));
    assert_eq!(447_f32, cofactor4(&m, 0, 1));
    assert_eq!(210_f32, cofactor4(&m, 0, 2));
    assert_eq!(-4071_f32, determinant4(&m));
}

#[test]
fn matrix4_is_invertibile() {
    let m = Matrix4::new(        
        6_f32, 4_f32, 4_f32, 4_f32, 
        5_f32, 5_f32, 7_f32, 6_f32,
        4_f32, -9_f32, 3_f32, -7_f32,
        9_f32, 1_f32, 7_f32, -6_f32
    );

    assert_eq!(true, is_invertible(&m));

    let m = Matrix4::new(        
        -4_f32, 2_f32, -2_f32, -3_f32, 
        9_f32, 6_f32, 2_f32, 6_f32,
        0_f32, -5_f32, 1_f32, -5_f32,
        0_f32, 0_f32, 0_f32, 0_f32
    );

    assert_eq!(false, is_invertible(&m));
}

#[test]
fn matrix4_invert() {
    let m = Matrix4::new(        
        -5_f32, 2_f32, 6_f32, -8_f32, 
        1_f32, -5_f32, 1_f32, 8_f32,
        7_f32, 7_f32, -6_f32, -7_f32,
        1_f32, -3_f32, 7_f32, 4_f32
    );

    let m_inversed = inverse4(&m);

    let expected_inverse = Matrix4::new(
        0.21805_f32, 0.45113_f32, 0.24060_f32, -0.04511_f32,
        -0.80827_f32, -1.45677_f32, -0.44361_f32, 0.52068_f32,
        -0.07895_f32, -0.22368_f32, -0.05263_f32, 0.19737_f32,
        -0.52256_f32, -0.81391_f32, -0.30075_f32, 0.30639_f32
    );

    assert_eq!(532_f32, determinant4(&m));
    assert_eq!(expected_inverse, m_inversed);
    assert_eq!(-160_f32, cofactor4(&m, 2, 3));
    assert_eq!(-160_f32/532_f32, m_inversed[[3, 2]]);
    assert_eq!(105_f32, cofactor4(&m, 3, 2));
    assert_eq!(105_f32/532_f32, m_inversed[[2, 3]]);

    let a = Matrix4::new(        
        3_f32, -9_f32, 7_f32, 3_f32, 
        3_f32, -8_f32, 2_f32, -9_f32,
        -4_f32, 4_f32, 4_f32, 1_f32,
        -6_f32, 5_f32, -1_f32, 1_f32
    );

    let b = Matrix4::new(        
        8_f32, 2_f32, 2_f32, 2_f32, 
        3_f32, -1_f32, 7_f32, 0_f32,
        7_f32, 0_f32, 5_f32, 4_f32,
        6_f32, -2_f32, 0_f32, 5_f32
    );

    let c = &a * &b;
    let inverse_b = inverse4(&b);
    let expect_a = &c * &inverse_b;

    assert_eq!(a, expect_a);
}