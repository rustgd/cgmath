use vector::{Vec2, Vec3, Vec4};

pub trait BooleanVector {
    pub fn any() -> bool;
    pub fn all() -> bool;
    pub fn not() -> self;
}

#[inline(always)] pub fn any<T:BooleanVector>(x: &T) -> bool { x.any() }
#[inline(always)] pub fn all<T:BooleanVector>(x: &T) -> bool { x.all() }
#[inline(always)] pub fn not<T:BooleanVector>(x: &T) -> T    { x.not() }

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