use core::ops::Index;

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
        let size = 3;
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
    assert_eq!(matrix[[3, 3]], 4_f64);
}