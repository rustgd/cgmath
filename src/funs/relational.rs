/**
 * Vector Relational Functions
 *
 * This module corresponds to Section 8.7 of the [GLSL 4.30.6 specification]
 * (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).
 */

use core::cmp::{Eq, Ord};
use vec::{Vector, Vec2, Vec3, Vec4};

pub trait RelVector<BVec> {
    pure fn less_than(&self, other: &self) -> BVec;
    pure fn less_than_equal(&self, other: &self) -> BVec;
    pure fn greater_than(&self, other: &self) -> BVec;
    pure fn greater_than_equal(&self, other: &self) -> BVec;
    pure fn equal(&self, other: &self) -> BVec;
    pure fn not_equal(&self, other: &self) -> BVec;
}

#[inline(always)] pub pure fn less_than         <T:RelVector<BV>, BV>(x: &T, y: &T) -> BV { x.less_than(y)          }
#[inline(always)] pub pure fn less_than_equal   <T:RelVector<BV>, BV>(x: &T, y: &T) -> BV { x.less_than_equal(y)    }
#[inline(always)] pub pure fn greater_than      <T:RelVector<BV>, BV>(x: &T, y: &T) -> BV { x.greater_than(y)       }
#[inline(always)] pub pure fn greater_than_equal<T:RelVector<BV>, BV>(x: &T, y: &T) -> BV { x.greater_than_equal(y) }
#[inline(always)] pub pure fn equal             <T:RelVector<BV>, BV>(x: &T, y: &T) -> BV { x.equal(y)              }
#[inline(always)] pub pure fn not_equal         <T:RelVector<BV>, BV>(x: &T, y: &T) -> BV { x.not_equal(y)          }

pub impl<T:Copy Ord Eq> Vec2<T>: RelVector<Vec2<bool>> {
    #[inline(always)]
    pure fn less_than(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(self[0] < other[0],
                  self[1] < other[1])
    }
    
    #[inline(always)]
    pure fn less_than_equal(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(self[0] <= other[0],
                  self[1] <= other[1])
    }
    
    #[inline(always)]
    pure fn greater_than(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(self[0] > other[0],
                  self[1] > other[1])
    }
    
    #[inline(always)]
    pure fn greater_than_equal(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(self[0] >= other[0],
                  self[1] >= other[1])
    }
    
    #[inline(always)]
    pure fn equal(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(self[0] == other[0],
                  self[1] == other[1])
    }
    
    #[inline(always)]
    pure fn not_equal(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(self[0] != other[0],
                  self[1] != other[1])
    }
}

pub impl<T:Copy Ord Eq> Vec3<T>: RelVector<Vec3<bool>> {
    #[inline(always)]
    pure fn less_than(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(self[0] < other[0],
                  self[1] < other[1],
                  self[2] < other[2])
    }
    
    #[inline(always)]
    pure fn less_than_equal(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(self[0] <= other[0],
                  self[1] <= other[1],
                  self[2] <= other[2])
    }
    
    #[inline(always)]
    pure fn greater_than(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(self[0] > other[0],
                  self[1] > other[1],
                  self[2] > other[2])
    }
    
    #[inline(always)]
    pure fn greater_than_equal(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(self[0] >= other[0],
                  self[1] >= other[1],
                  self[2] >= other[2])
    }
    
    #[inline(always)]
    pure fn equal(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(self[0] == other[0],
                  self[1] == other[1],
                  self[2] == other[2])
    }
    
    #[inline(always)]
    pure fn not_equal(&self, other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(self[0] != other[0],
                  self[1] != other[1],
                  self[2] != other[2])
    }
}

pub impl<T:Copy Ord Eq> Vec4<T>: RelVector<Vec4<bool>> {
    #[inline(always)]
    pure fn less_than(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(self[0] < other[0],
                  self[1] < other[1],
                  self[2] < other[2],
                  self[3] < other[3])
    }
    
    #[inline(always)]
    pure fn less_than_equal(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(self[0] <= other[0],
                  self[1] <= other[1],
                  self[2] <= other[2],
                  self[3] <= other[3])
    }
    
    #[inline(always)]
    pure fn greater_than(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(self[0] > other[0],
                  self[1] > other[1],
                  self[2] > other[2],
                  self[3] > other[3])
    }
    
    #[inline(always)]
    pure fn greater_than_equal(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(self[0] >= other[0],
                  self[1] >= other[1],
                  self[2] >= other[2],
                  self[3] >= other[3])
    }
    
    #[inline(always)]
    pure fn equal(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(self[0] == other[0],
                  self[1] == other[1],
                  self[2] == other[2],
                  self[3] == other[3])
    }
    
    #[inline(always)]
    pure fn not_equal(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(self[0] != other[0],
                  self[1] != other[1],
                  self[2] != other[2],
                  self[3] != other[3])
    }
}

pub trait BooleanVector: Vector<bool> {
    pure fn any(&self) -> bool;
    pure fn all(&self) -> bool;
    pure fn not(&self) -> self;
}

#[inline(always)] pub pure fn any<T:BooleanVector>(x: &T) -> bool { x.any() }
#[inline(always)] pub pure fn all<T:BooleanVector>(x: &T) -> bool { x.all() }
#[inline(always)] pub pure fn not<T:BooleanVector>(x: &T) -> T    { x.not() }

pub impl Vec2<bool>: BooleanVector {
    #[inline(always)]
    pure fn any(&self) -> bool {
        self[0] || self[1]
    }
    
    #[inline(always)]
    pure fn all(&self) -> bool {
        self[0] && self[1]
    }
    
    #[inline(always)]
    pure fn not(&self) -> Vec2<bool> { 
        Vec2::new(!self[0], !self[1])
    }
}

pub impl Vec3<bool>: BooleanVector {
    #[inline(always)]
    pure fn any(&self) -> bool {
        self[0] || self[1] || self[2]
    }
    
    #[inline(always)]
    pure fn all(&self) -> bool {
        self[0] && self[1] && self[2]
    }
    
    #[inline(always)]
    pure fn not(&self) -> Vec3<bool> { 
        Vec3::new(!self[0], !self[1], !self[2])
    }
}

pub impl Vec4<bool>: BooleanVector {
    #[inline(always)]
    pure fn any(&self) -> bool {
        self[0] || self[1] || self[2] || self[3]
    }
    
    #[inline(always)]
    pure fn all(&self) -> bool {
        self[0] && self[1] && self[2] && self[3]
    }
    
    #[inline(always)]
    pure fn not(&self) -> Vec4<bool> { 
        Vec4::new(!self[0], !self[1], !self[2], !self[3])
    }
}