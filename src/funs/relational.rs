/**
 * Vector Relational Functions
 */

use core::cmp::{Eq, Ord};

use vec::{Vector, Vec2, Vec3, Vec4};

pub trait RelVector<BVec> {
    pure fn less_than(y: &self) -> BVec;
    pure fn less_than_equal(y: &self) -> BVec;
    pure fn greater_than(y: &self) -> BVec;
    pure fn greater_than_equal(y: &self) -> BVec;
    pure fn equal(y: &self) -> BVec;
    pure fn not_equal(y: &self) -> BVec;
}

#[inline(always)] pub pure fn less_than         <T:RelVector<BV>, BV>(x: &T, y: &T) -> BV { x.less_than(y)          }
#[inline(always)] pub pure fn less_than_equal   <T:RelVector<BV>, BV>(x: &T, y: &T) -> BV { x.less_than_equal(y)    }
#[inline(always)] pub pure fn greater_than      <T:RelVector<BV>, BV>(x: &T, y: &T) -> BV { x.greater_than(y)       }
#[inline(always)] pub pure fn greater_than_equal<T:RelVector<BV>, BV>(x: &T, y: &T) -> BV { x.greater_than_equal(y) }
#[inline(always)] pub pure fn equal             <T:RelVector<BV>, BV>(x: &T, y: &T) -> BV { x.equal(y)              }
#[inline(always)] pub pure fn not_equal         <T:RelVector<BV>, BV>(x: &T, y: &T) -> BV { x.not_equal(y)          }

pub impl<T:Copy Ord Eq> Vec2<T>: RelVector<Vec2<bool>> {
    #[inline(always)]
    pure fn less_than(y: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(self[0] < y[0],
                  self[1] < y[1])
    }
    
    #[inline(always)]
    pure fn less_than_equal(y: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(self[0] <= y[0],
                  self[1] <= y[1])
    }
    
    #[inline(always)]
    pure fn greater_than(y: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(self[0] > y[0],
                  self[1] > y[1])
    }
    
    #[inline(always)]
    pure fn greater_than_equal(y: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(self[0] >= y[0],
                  self[1] >= y[1])
    }
    
    #[inline(always)]
    pure fn equal(y: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(self[0] == y[0],
                  self[1] == y[1])
    }
    
    #[inline(always)]
    pure fn not_equal(y: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(self[0] != y[0],
                  self[1] != y[1])
    }
}

pub impl<T:Copy Ord Eq> Vec3<T>: RelVector<Vec3<bool>> {
    #[inline(always)]
    pure fn less_than(y: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(self[0] < y[0],
                  self[1] < y[1],
                  self[2] < y[2])
    }
    
    #[inline(always)]
    pure fn less_than_equal(y: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(self[0] <= y[0],
                  self[1] <= y[1],
                  self[2] <= y[2])
    }
    
    #[inline(always)]
    pure fn greater_than(y: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(self[0] > y[0],
                  self[1] > y[1],
                  self[2] > y[2])
    }
    
    #[inline(always)]
    pure fn greater_than_equal(y: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(self[0] >= y[0],
                  self[1] >= y[1],
                  self[2] >= y[2])
    }
    
    #[inline(always)]
    pure fn equal(y: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(self[0] == y[0],
                  self[1] == y[1],
                  self[2] == y[2])
    }
    
    #[inline(always)]
    pure fn not_equal(y: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(self[0] != y[0],
                  self[1] != y[1],
                  self[2] != y[2])
    }
}

pub impl<T:Copy Ord Eq> Vec4<T>: RelVector<Vec4<bool>> {
    #[inline(always)]
    pure fn less_than(y: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(self[0] < y[0],
                  self[1] < y[1],
                  self[2] < y[2],
                  self[3] < y[3])
    }
    
    #[inline(always)]
    pure fn less_than_equal(y: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(self[0] <= y[0],
                  self[1] <= y[1],
                  self[2] <= y[2],
                  self[3] <= y[3])
    }
    
    #[inline(always)]
    pure fn greater_than(y: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(self[0] > y[0],
                  self[1] > y[1],
                  self[2] > y[2],
                  self[3] > y[3])
    }
    
    #[inline(always)]
    pure fn greater_than_equal(y: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(self[0] >= y[0],
                  self[1] >= y[1],
                  self[2] >= y[2],
                  self[3] >= y[3])
    }
    
    #[inline(always)]
    pure fn equal(y: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(self[0] == y[0],
                  self[1] == y[1],
                  self[2] == y[2],
                  self[3] == y[3])
    }
    
    #[inline(always)]
    pure fn not_equal(y: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(self[0] != y[0],
                  self[1] != y[1],
                  self[2] != y[2],
                  self[3] != y[3])
    }
}

pub trait BooleanVector: Vector<bool> {
    pure fn any() -> bool;
    pure fn all() -> bool;
    pure fn not() -> self;
}

#[inline(always)] pub pure fn any<T:BooleanVector>(x: &T) -> bool { x.any() }
#[inline(always)] pub pure fn all<T:BooleanVector>(x: &T) -> bool { x.all() }
#[inline(always)] pub pure fn not<T:BooleanVector>(x: &T) -> T    { x.not() }

pub impl Vec2<bool>: BooleanVector {
    pure fn any() -> bool {
        self[0] || self[1]
    }
    
    pure fn all() -> bool {
        self[0] && self[1]
    }
    
    pure fn not() -> Vec2<bool> { 
        Vec2::new(!self[0], !self[1])
    }
}

pub impl Vec3<bool>: BooleanVector {
    pure fn any() -> bool {
        self[0] || self[1] || self[2]
    }
    
    pure fn all() -> bool {
        self[0] && self[1] && self[2]
    }
    
    pure fn not() -> Vec3<bool> { 
        Vec3::new(!self[0], !self[1], !self[2])
    }
}

pub impl Vec4<bool>: BooleanVector {
    pure fn any() -> bool {
        self[0] || self[1] || self[2] || self[3]
    }
    
    pure fn all() -> bool {
        self[0] && self[1] && self[2] && self[3]
    }
    
    pure fn not() -> Vec4<bool> { 
        Vec4::new(!self[0], !self[1], !self[2], !self[3])
    }
}