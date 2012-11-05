use cmp::{Eq, Ord};

use vector::{Vec2, Vec3, Vec4};

pub trait RelationalVector<BVec> {
    pure fn less_than(other: &self) -> BVec;
    pure fn less_than_equal(other: &self) -> BVec;
    pure fn greater_than(other: &self) -> BVec;
    pure fn greater_than_equal(other: &self) -> BVec;
    pure fn equal(other: &self) -> BVec;
    pure fn not_equal(other: &self) -> BVec;
}

#[inline(always)] pub pure fn less_than         <T:RelationalVector<BVec>, BVec>(x: &T, y: &T) -> BVec { x.less_than(y)          }
#[inline(always)] pub pure fn less_than_equal   <T:RelationalVector<BVec>, BVec>(x: &T, y: &T) -> BVec { x.less_than_equal(y)    }
#[inline(always)] pub pure fn greater_than      <T:RelationalVector<BVec>, BVec>(x: &T, y: &T) -> BVec { x.greater_than(y)       }
#[inline(always)] pub pure fn greater_than_equal<T:RelationalVector<BVec>, BVec>(x: &T, y: &T) -> BVec { x.greater_than_equal(y) }
#[inline(always)] pub pure fn equal             <T:RelationalVector<BVec>, BVec>(x: &T, y: &T) -> BVec { x.equal(y)              }
#[inline(always)] pub pure fn not_equal         <T:RelationalVector<BVec>, BVec>(x: &T, y: &T) -> BVec { x.not_equal(y)          }

pub trait BooleanVector {
    pub fn any() -> bool;
    pub fn all() -> bool;
    pub fn not() -> self;
}

#[inline(always)] pub fn any<T:BooleanVector>(x: &T) -> bool { x.any() }
#[inline(always)] pub fn all<T:BooleanVector>(x: &T) -> bool { x.all() }
#[inline(always)] pub fn not<T:BooleanVector>(x: &T) -> T    { x.not() }




pub impl<T:Copy Ord Eq> Vec2<T>: RelationalVector<Vec2<bool>> {
    #[inline(always)]
    pure fn less_than(other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(self[0] < other[0],
                  self[1] < other[1])
    }
    
    #[inline(always)]
    pure fn less_than_equal(other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(self[0] <= other[0],
                  self[1] <= other[1])
    }
    
    #[inline(always)]
    pure fn greater_than(other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(self[0] > other[0],
                  self[1] > other[1])
    }
    
    #[inline(always)]
    pure fn greater_than_equal(other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(self[0] >= other[0],
                  self[1] >= other[1])
    }
    
    #[inline(always)]
    pure fn equal(other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(self[0] == other[0],
                  self[1] == other[1])
    }
    
    #[inline(always)]
    pure fn not_equal(other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(self[0] != other[0],
                  self[1] != other[1])
    }
}

pub impl Vec2<bool>: BooleanVector {
    pub fn any() -> bool {
        self[0] || self[1]
    }
    
    pub fn all() -> bool {
        self[0] && self[1]
    }
    
    pub fn not() -> Vec2<bool> { 
        Vec2::new(!self[0], !self[1])
    }
}



pub impl<T:Copy Ord Eq> Vec3<T>: RelationalVector<Vec3<bool>> {
    #[inline(always)]
    pure fn less_than(other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(self[0] < other[0],
                  self[1] < other[1],
                  self[2] < other[2])
    }
    
    #[inline(always)]
    pure fn less_than_equal(other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(self[0] <= other[0],
                  self[1] <= other[1],
                  self[2] <= other[2])
    }
    
    #[inline(always)]
    pure fn greater_than(other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(self[0] > other[0],
                  self[1] > other[1],
                  self[2] > other[2])
    }
    
    #[inline(always)]
    pure fn greater_than_equal(other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(self[0] >= other[0],
                  self[1] >= other[1],
                  self[2] >= other[2])
    }
    
    #[inline(always)]
    pure fn equal(other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(self[0] == other[0],
                  self[1] == other[1],
                  self[2] == other[2])
    }
    
    #[inline(always)]
    pure fn not_equal(other: &Vec3<T>) -> Vec3<bool> {
        Vec3::new(self[0] != other[0],
                  self[1] != other[1],
                  self[2] != other[2])
    }
}

pub impl Vec3<bool>: BooleanVector {
    pub fn any() -> bool {
        self[0] || self[1] || self[2]
    }
    
    pub fn all() -> bool {
        self[0] && self[1] && self[2]
    }
    
    pub fn not() -> Vec3<bool> { 
        Vec3::new(!self[0], !self[1], !self[2])
    }
}




pub impl<T:Copy Ord Eq> Vec4<T>: RelationalVector<Vec4<bool>> {
    #[inline(always)]
    pure fn less_than(other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(self[0] < other[0],
                  self[1] < other[1],
                  self[2] < other[2],
                  self[3] < other[3])
    }
    
    #[inline(always)]
    pure fn less_than_equal(other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(self[0] <= other[0],
                  self[1] <= other[1],
                  self[2] <= other[2],
                  self[3] <= other[3])
    }
    
    #[inline(always)]
    pure fn greater_than(other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(self[0] > other[0],
                  self[1] > other[1],
                  self[2] > other[2],
                  self[3] > other[3])
    }
    
    #[inline(always)]
    pure fn greater_than_equal(other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(self[0] >= other[0],
                  self[1] >= other[1],
                  self[2] >= other[2],
                  self[3] >= other[3])
    }
    
    #[inline(always)]
    pure fn equal(other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(self[0] == other[0],
                  self[1] == other[1],
                  self[2] == other[2],
                  self[3] == other[3])
    }
    
    #[inline(always)]
    pure fn not_equal(other: &Vec4<T>) -> Vec4<bool> {
        Vec4::new(self[0] != other[0],
                  self[1] != other[1],
                  self[2] != other[2],
                  self[3] != other[3])
    }
}

pub impl Vec4<bool>: BooleanVector {
    pub fn any() -> bool {
        self[0] || self[1] || self[2] || self[3]
    }
    
    pub fn all() -> bool {
        self[0] && self[1] && self[2] && self[3]
    }
    
    pub fn not() -> Vec4<bool> { 
        Vec4::new(!self[0], !self[1], !self[2], !self[3])
    }
}