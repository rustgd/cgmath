use vec::{Vector, Vec2, Vec3, Vec4};

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