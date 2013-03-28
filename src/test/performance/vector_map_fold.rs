/**
 * Component-wise map and fold function speed tests. For best results, compile
 * with the optimise flag (-O). These functions would allow for even more generic
 * operations on dimensional data structures. Map seems to be faster than hand
 * unrolling for add_t, but map2 for add_v is slower. A combination of map2 and
 * foldl is faster for dot product.
 */

extern mod std;
use std::time::precise_time_ns;
use cast::transmute;
use vec::raw::buf_as_slice;
use ptr::to_unsafe_ptr;
use cmp::Eq;
use num::from_int;

pub struct Vec4<T> { x: T, y: T, z: T, w: T }

pub mod Vec4 {
    #[inline(always)]
    pub fn new<T>(x: T, y: T, z: T, w: T) -> Vec4<T> {
        Vec4 { x: move x, y: move y, z: move z, w: move w }
    }
}

pub impl<T:Copy Num> Vec4<T> {
    #[inline(always)]
    fn index(i: uint) -> T {
        unsafe { do buf_as_slice(
            transmute::<*Vec4<T>, *T>(
                to_unsafe_ptr(&self)), 4) |slice| { slice[i] }
        }
    }
    
    ////////////////////////////////////////////////////////////////////////////
    
    #[inline(always)]
    fn map(f: fn&(a: &T) -> T) -> Vec4<T> {
        Vec4::new(f(&self[0]),
                  f(&self[1]),
                  f(&self[2]),
                  f(&self[3]))
    }
    
    #[inline(always)]
    fn map2(other: &Vec4<T>, f: fn&(a: &T, b: &T) -> T) -> Vec4<T> {
        Vec4::new(f(&self[0], &other[0]),
                  f(&self[1], &other[1]),
                  f(&self[2], &other[2]),
                  f(&self[3], &other[3]))
    }
    
    fn foldl<U: Copy>(z: U, p: &fn(t: T, u: &U) -> U) -> U {
        p(self[3], &p(self[2], &p(self[1], &p(self[0], &z))))
    }
    fn foldr<U: Copy>(z: U, p: &fn(t: &T, u: U) -> U) -> U {
        p(&self[0], p(&self[1], p(&self[2], p(&self[3], z))))
    }
    
    ////////////////////////////////////////////////////////////////////////////
    
    #[inline(always)]
    fn mul_t(value: T) -> Vec4<T> {
        Vec4::new(self[0] * value,
                  self[1] * value,
                  self[2] * value,
                  self[3] * value)
    }
    
    #[inline(always)]
    fn mul_t_map(value: T) -> Vec4<T> {
        do self.map |a| { a * value }
    }
    
    #[inline(always)]
    fn add_v(other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0] + other[0],
                  self[1] + other[1],
                  self[2] + other[2],
                  self[3] + other[3])
    }
    
    #[inline(always)]
    fn add_v_map2(other: &Vec4<T>) -> Vec4<T> {
        do self.map2(other) |a, b| { a + *b }
    }
    
    #[inline(always)]
    fn dot(other: &Vec4<T>) -> T {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2] +
        self[3] * other[3]
    }
    
    #[inline(always)]
    fn dot_foldl(other: &Vec4<T>) -> T {
        self.map2(other, |a, b| { a * *b })
            .foldl(from_int(0), |t, u| { t + *u })
    }
}

pub impl<T:Copy Num Eq> Vec4<T>: Eq {
    #[inline(always)]
    fn eq(other: &Vec4<T>) -> bool {
        self[0] == other[0] &&
        self[1] == other[1] &&
        self[2] == other[2] &&
        self[3] == other[3]
    }
    
    #[inline(always)]
    fn ne(other: &Vec4<T>) -> bool {
        !(self == *other)
    }
}

fn main() {
    let n_tests = 10000;
    
    // Map
    
    let a = Vec4::new(1f, 2f, 3f, 4f);
    let b = Vec4::new(5f, 6f, 7f, 8f);
    
    let mul_t_avg = do test_avg_time_ns(n_tests) {
        assert a.mul_t(8f) == Vec4::new(8f, 16f, 24f, 32f);
    };
    
    let mul_t_map_avg = do test_avg_time_ns(n_tests) {
        assert a.mul_t_map(8f) == Vec4::new(8f, 16f, 24f, 32f);
    };
    
    let min = [mul_t_avg, mul_t_map_avg].min();
    
    io::println(fmt!("mul_t:        %d = %d", mul_t_avg as int, (mul_t_avg - min) as int));
    io::println(fmt!("mul_t_map:    %d = %d", mul_t_map_avg as int, (mul_t_map_avg - min) as int));
    
    // Zip
    
    let add_v_avg = do test_avg_time_ns(n_tests) {
        assert a.add_v(&b) == Vec4::new( 6f,  8f, 10f, 12f);
    };
    
    let add_v_map2_avg = do test_avg_time_ns(n_tests) {
        assert a.add_v_map2(&b) == Vec4::new( 6f,  8f, 10f, 12f);
    };
    
    let min = [add_v_avg, add_v_map2_avg].min();
    
    io::println(fmt!("add_v:        %d = %d", add_v_avg as int, (add_v_avg - min) as int));
    io::println(fmt!("add_v_map2:    %d = %d", add_v_map2_avg as int, (add_v_map2_avg - min) as int));
    
    // Dot
    
    let dot_avg = do test_avg_time_ns(n_tests) {
        assert a.dot(&b) == 70f;
    };
    
    let dot_foldl_avg = do test_avg_time_ns(n_tests) {
        assert a.dot_foldl(&b) == 70f;
    };
    
    let min = [dot_avg, dot_foldl_avg].min();
    
    io::println(fmt!("dot:        %d = %d", dot_avg as int, (dot_avg - min) as int));
    io::println(fmt!("dot_foldl:  %d = %d", dot_foldl_avg as int, (dot_foldl_avg - min) as int));
    
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