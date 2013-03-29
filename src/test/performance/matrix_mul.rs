/**
 * Matrix multiplication performance test. For best results, compile with the optimise
 * flag (-O). Surprisingly it seems mul_matrix_dot_product is faster than 
 * mul_matrix_expanded.
 *
 * Output:
 *
 * ~~~
 * % rustc matrix_mul.rs -O
 *
 * % ./matrix_mul          
 * mul_matrix_expanded:       810 = 41
 * mul_matrix_dot_product:    769 = 0
 *
 * % ./matrix_mul
 * mul_matrix_expanded:       801 = 0
 * mul_matrix_dot_product:    817 = 16
 *
 * % ./matrix_mul
 * mul_matrix_expanded:       850 = 68
 * mul_matrix_dot_product:    782 = 0
 *
 * % ./matrix_mul
 * mul_matrix_expanded:       808 = 0
 * mul_matrix_dot_product:    817 = 9
 *
 * % ./matrix_mul
 * mul_matrix_expanded:       815 = 42
 * mul_matrix_dot_product:    773 = 0
 *
 * % ./matrix_mul
 * mul_matrix_expanded:       831 = 42
 * mul_matrix_dot_product:    789 = 0
 * ~~~
 */

extern mod std;
use std::time::precise_time_ns;
use cast::transmute;
use cmp::Eq;
use vec::raw::buf_as_slice;
use ptr::to_unsafe_ptr;

fn main() {
    let n_tests = 100000;
    
    let a = Mat4::new(1f, 5f,  9f, 13f,
                      2f, 6f, 10f, 14f,
                      3f, 7f, 11f, 15f,
                      4f, 8f, 12f, 16f);
    let b = Mat4::new(2f, 6f, 10f, 14f,
                      3f, 7f, 11f, 15f,
                      4f, 8f, 12f, 16f,
                      5f, 9f, 13f, 17f);
    
    let expected = Mat4::new(100f, 228f, 356f, 484f,
                             110f, 254f, 398f, 542f,
                             120f, 280f, 440f, 600f,
                             130f, 306f, 482f, 658f);
    
    let mul_matrix_expanded_avg = do test_avg_time_ns(n_tests) {
        assert a.mul_matrix_expanded(&b) == expected;
    };
    
    let mul_matrix_dot_product_avg = do test_avg_time_ns(n_tests) {
        assert a.mul_matrix_dot_product(&b) == expected;
    };
    
    let min = [mul_matrix_expanded_avg, mul_matrix_dot_product_avg].min();
    
    io::println(fmt!("mul_matrix_expanded:       %d = %d", mul_matrix_expanded_avg as int, (mul_matrix_expanded_avg - min) as int));
    io::println(fmt!("mul_matrix_dot_product:    %d = %d", mul_matrix_dot_product_avg as int, (mul_matrix_dot_product_avg - min) as int));
    
}

// Vector

pub struct Vec4 { x: float, y: float, z: float, w: float }

mod Vec4 {
    #[inline(always)]
    pub fn new(x: float, y: float, z: float, w: float) -> Vec4 {
        Vec4 { x: move x, y: move y, z: move z, w: move w }
    }
}

impl Vec4 {
    #[inline(always)]
    fn dot(other: &Vec4) -> float {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2] +
        self[3] * other[3]
    }
}

pub impl Vec4: Index<uint, float> {
    #[inline(always)]
    fn index(i: uint) -> float {
        unsafe { do buf_as_slice(
            transmute::<*Vec4, *float>(
                to_unsafe_ptr(&self)), 4) |slice| { slice[*i] }
        }
    }
}

pub impl Vec4: Eq {
    #[inline(always)]
    fn eq(other: &Vec4) -> bool {
        self[0] == other [0] &&
        self[1] == other [1] &&
        self[2] == other [2] &&
        self[3] == other [3]
    }
    
    #[inline(always)]
    fn ne(other: &Vec4) -> bool {
        !(self == *other)
    }
}

// Matrix

pub struct Mat4 { x: Vec4, y: Vec4, z: Vec4, w: Vec4 }

mod Mat4 {
    #[inline(always)]
    pub fn new(c0r0: float, c0r1: float, c0r2: float, c0r3: float,
                    c1r0: float, c1r1: float, c1r2: float, c1r3: float,
                    c2r0: float, c2r1: float, c2r2: float, c2r3: float,
                    c3r0: float, c3r1: float, c3r2: float, c3r3: float) -> Mat4  {
        Mat4::from_cols(Vec4::new(move c0r0, move c0r1, move c0r2, move c0r3),
                        Vec4::new(move c1r0, move c1r1, move c1r2, move c1r3),
                        Vec4::new(move c2r0, move c2r1, move c2r2, move c2r3),
                        Vec4::new(move c3r0, move c3r1, move c3r2, move c3r3))
    }
    
    #[inline(always)]
    pub fn from_cols(c0: Vec4, c1: Vec4, c2: Vec4, c3: Vec4) -> Mat4 {
        Mat4 { x: move c0,
               y: move c1,
               z: move c2,
               w: move c3 }
    }
}

impl Mat4 {
    #[inline(always)]
    fn col(i: uint) -> Vec4 { self[i] }
    
    #[inline(always)]
    fn row(i: uint) -> Vec4 {
        Vec4::new(self[0][i],
                  self[1][i],
                  self[2][i],
                  self[3][i])
    }
    
    fn mul_matrix_expanded(other: &Mat4) -> Mat4 {
        Mat4::new(self[0][0] * other[0][0] + self[1][0] * other[0][1] + self[2][0] * other[0][2] + self[3][0] * other[0][3],
                  self[0][1] * other[0][0] + self[1][1] * other[0][1] + self[2][1] * other[0][2] + self[3][1] * other[0][3],
                  self[0][2] * other[0][0] + self[1][2] * other[0][1] + self[2][2] * other[0][2] + self[3][2] * other[0][3],
                  self[0][3] * other[0][0] + self[1][3] * other[0][1] + self[2][3] * other[0][2] + self[3][3] * other[0][3],
             
                  self[0][0] * other[1][0] + self[1][0] * other[1][1] + self[2][0] * other[1][2] + self[3][0] * other[1][3],
                  self[0][1] * other[1][0] + self[1][1] * other[1][1] + self[2][1] * other[1][2] + self[3][1] * other[1][3],
                  self[0][2] * other[1][0] + self[1][2] * other[1][1] + self[2][2] * other[1][2] + self[3][2] * other[1][3],
                  self[0][3] * other[1][0] + self[1][3] * other[1][1] + self[2][3] * other[1][2] + self[3][3] * other[1][3],
             
                  self[0][0] * other[2][0] + self[1][0] * other[2][1] + self[2][0] * other[2][2] + self[3][0] * other[2][3],
                  self[0][1] * other[2][0] + self[1][1] * other[2][1] + self[2][1] * other[2][2] + self[3][1] * other[2][3],
                  self[0][2] * other[2][0] + self[1][2] * other[2][1] + self[2][2] * other[2][2] + self[3][2] * other[2][3],
                  self[0][3] * other[2][0] + self[1][3] * other[2][1] + self[2][3] * other[2][2] + self[3][3] * other[2][3],
             
                  self[0][0] * other[3][0] + self[1][0] * other[3][1] + self[2][0] * other[3][2] + self[3][0] * other[3][3],
                  self[0][1] * other[3][0] + self[1][1] * other[3][1] + self[2][1] * other[3][2] + self[3][1] * other[3][3],
                  self[0][2] * other[3][0] + self[1][2] * other[3][1] + self[2][2] * other[3][2] + self[3][2] * other[3][3],
                  self[0][3] * other[3][0] + self[1][3] * other[3][1] + self[2][3] * other[3][2] + self[3][3] * other[3][3])
    }
    
    fn mul_matrix_dot_product(other: &Mat4) -> Mat4 {
        Mat4::new(self.row(0).dot(&other.col(0)), self.row(1).dot(&other.col(0)), self.row(2).dot(&other.col(0)), self.row(3).dot(&other.col(0)),
                  self.row(0).dot(&other.col(1)), self.row(1).dot(&other.col(1)), self.row(2).dot(&other.col(1)), self.row(3).dot(&other.col(1)),
                  self.row(0).dot(&other.col(2)), self.row(1).dot(&other.col(2)), self.row(2).dot(&other.col(2)), self.row(3).dot(&other.col(2)),
                  self.row(0).dot(&other.col(3)), self.row(1).dot(&other.col(3)), self.row(2).dot(&other.col(3)), self.row(3).dot(&other.col(3)))
    }
}

pub impl Mat4: Index<uint, Vec4> {
    #[inline(always)]
    fn index(i: uint) -> Vec4 {
        unsafe { do buf_as_slice(
            transmute::<*Mat4, *Vec4>(
                to_unsafe_ptr(&self)), 4) |slice| { slice[*i] }
        }
    }
}

pub impl Mat4: Eq {
    #[inline(always)]
    fn eq(other: &Mat4) -> bool {
        self[0] == other [0] &&
        self[1] == other [1] &&
        self[2] == other [2] &&
        self[3] == other [3]
    }
    
    #[inline(always)]
    fn ne(other: &Mat4) -> bool {
        !(self == *other)
    }
}

fn test_avg_time_ns(n: uint, f: fn&()) -> u64 {
    let mut total = 0;
    for n.times {
        let start_time = precise_time_ns();
        
        f();
        
        total += precise_time_ns() - start_time;
    }
    return total / (n as u64);
}