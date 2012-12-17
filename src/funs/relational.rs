/**
 * Vector Relational Functions
 *
 * This module corresponds to Section 8.7 of the [GLSL 4.30.6 specification]
 * (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).
 */
 
use core::cmp::{Eq, Ord};
use vec::{Vector, Vec2, Vec3, Vec4};

/**
 * Component-wise vector comparison methods
 */
pub trait OrdinalVector<T, BoolVec>: Vector<T> {
    /**
     * Component-wise compare of `self < other`
     */
    pure fn less_than(&self, other: &self) -> BoolVec;
    
    /**
     * Component-wise compare of `self <= other`
     */
    pure fn less_than_equal(&self, other: &self) -> BoolVec;
    
    /**
     * Component-wise compare of `self > other`
     */
    pure fn greater_than(&self, other: &self) -> BoolVec;
    
    /**
     * Component-wise compare of `self >= other`
     */
    pure fn greater_than_equal(&self, other: &self) -> BoolVec;
}

/**
 * Component-wise equality comparison methods
 */
pub trait EquableVector<T, BoolVec>: Vector<T> {
    /**
     * Component-wise compare of `self == other`
     */
    pure fn equal(&self, other: &self) -> BoolVec;
    
    /**
     * Component-wise compare of `self != other`
     */
    pure fn not_equal(&self, other: &self) -> BoolVec;
}

/**
 * A vector with boolean components
 */
pub trait BooleanVector: Vector<bool> {
    /**
     * # Return value
     *
     * `true` if of any component is `true`
     */
    pure fn any(&self) -> bool;
    
    /**
     * # Return value
     *
     * `true` only if all components are `true`
     */
    pure fn all(&self) -> bool;
    
    /**
     * # Return value
     *
     * the component-wise logical complement
     */
    pure fn not(&self) -> self;
}

pub impl<T:Copy Ord> Vec2<T>: OrdinalVector<T, Vec2<bool>> {
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
}

pub impl<T:Copy Eq> Vec2<T>: EquableVector<T, Vec2<bool>> {
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

pub impl<T:Copy Ord> Vec3<T>: OrdinalVector<T, Vec3<bool>> {
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
}

pub impl<T:Copy Eq> Vec3<T>: EquableVector<T, Vec3<bool>> {
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

pub impl<T:Copy Ord> Vec4<T>: OrdinalVector<T, Vec4<bool>> {
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
}

pub impl<T:Copy Eq> Vec4<T>: EquableVector<T, Vec4<bool>> {
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