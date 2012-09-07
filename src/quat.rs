import std::cmp::FuzzyEq;
import cmp::Ord;
import float::sqrt;
import num::Num;
import to_str::ToStr;
import mat::{mat3, mat4};
import vec::vec3;

// TODO: Unittests! I've probably made lots of mistakes...

//
//  Quaternion
//
trait Quaternion<T:Num Ord FuzzyEq> {
    pure fn dim() -> uint;
    
    pure fn index(&&index:uint) -> T;
    pure fn w() -> T;
    pure fn x() -> T;
    pure fn y() -> T;
    pure fn z() -> T;
    
    pure fn neg() -> self;
    
    pure fn mul_f(&&value:T) -> self;
    pure fn div_f(&&value:T) -> self;
    
    // pure fn mul_v(&&vec3) -> vec3;
    
    pure fn add_q(&&other:self) -> self;
    pure fn sub_q(&&other:self) -> self;
    pure fn mul_q(&&other:self) -> self;
    
    pure fn exact_eq(&&other:self) -> bool;
    pure fn fuzzy_eq(&&other:self) -> bool;
    pure fn eq(&&other:self) -> bool;
    
    pure fn conjugate() -> self;
    pure fn inverse() -> self;
    pure fn magnitude2() -> T;
    pure fn magnitude() -> T;
    
    pure fn to_mat3() -> mat3;
    pure fn to_mat4() -> mat4;
}






//
//  Quat struct definition
//
struct quat { data:[float * 4] }

//
//  Constants
//
#[inline(always)] pure fn quat_zero()     -> quat { quat(0f, 0f, 0f, 0f) }
#[inline(always)] pure fn quat_identity() -> quat { quat(1f, 0f, 0f, 0f) }

//
//  Quat Constructor
//
#[inline(always)]
pure fn quat(w:float, x:float, y:float, z:float) -> quat {
    quat { data: [ w, x, y, z ] }
}

//
//  Quaternion Implementation
//
impl quat: Quaternion<float> {
    #[inline(always)]
    pure fn dim() -> uint { 4 }
    
    #[inline(always)]
    pure fn index(&&i: uint) -> float {
        self.data[i]
    }
    
    #[inline(always)]
    pure fn neg() -> quat {
        quat(-self[0], -self[1], -self[2], -self[3])
    }
    
    #[inline(always)] pure fn w() -> float { self.data[0] }
    #[inline(always)] pure fn x() -> float { self.data[1] }
    #[inline(always)] pure fn y() -> float { self.data[2] }
    #[inline(always)] pure fn z() -> float { self.data[3] }
    
    #[inline(always)]
    pure fn mul_f(&&value:float) -> quat {
        quat(self[0] * value,
             self[1] * value,
             self[2] * value,
             self[3] * value)
    }
    
    #[inline(always)]
    pure fn div_f(&&value:float) -> quat {
        quat(self[0] / value,
             self[1] / value,
             self[2] / value,
             self[3] / value)
    }
    
    #[inline(always)]
    pure fn add_q(&&other:quat) -> quat{
        quat(self[0] + other[0],
             self[1] + other[1],
             self[2] + other[2],
             self[3] + other[3])
    }
    
    #[inline(always)]
    pure fn sub_q(&&other:quat) -> quat{
        quat(self[0] - other[0],
             self[1] - other[1],
             self[2] - other[2],
             self[3] - other[3])
    }
    
    #[inline(always)]
    pure fn mul_q(&&other:quat) -> quat {
        quat(self.w()*other.w() - self.x()*other.x() - self.y()*other.y() - self.z()*other.z(),
             self.w()*other.x() + self.x()*other.w() + self.y()*other.z() - self.z()*other.y(),
             self.w()*other.y() + self.y()*other.w() + self.z()*other.x() - self.x()*other.z(),
             self.w()*other.z() + self.z()*other.w() + self.x()*other.y() - self.y()*other.x())
    }
    
    #[inline(always)]
    pure fn exact_eq(&&other:quat) -> bool {
        self[0] == other[0] &&
        self[1] == other[1] &&
        self[2] == other[2] &&
        self[3] == other[3]
    }
    
    #[inline(always)]
    pure fn fuzzy_eq(&&other: quat) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2]) &&
        self[3].fuzzy_eq(&other[3])
    }
    
    #[inline(always)]
    pure fn eq(&&other:quat) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn conjugate() -> quat {
        quat(self.w(), -self.x(), -self.y(), -self.z())
    }
    
    #[inline(always)]
    pure fn inverse() -> quat {
        self.conjugate().mul_f((1f / self.magnitude2()))
    }
    
    #[inline(always)]
    pure fn magnitude2() -> float {
        self.w() * self.w() +
        self.x() * self.x() +
        self.y() * self.y() +
        self.z() * self.z()
    }
    
    #[inline(always)]
    pure fn magnitude() -> float {
        sqrt(self.magnitude2())
    }
    
    #[inline(always)]
    pure fn to_mat3() -> mat3 {
        let x2 = self.x() + self.x();
        let y2 = self.y() + self.y();
        let z2 = self.z() + self.z();
        
        let xx2 = x2 * self.x();
        let xy2 = x2 * self.y();
        let xz2 = x2 * self.z();
        
        let yy2 = y2 * self.y();
        let yz2 = y2 * self.z();
        let zz2 = z2 * self.z();
        
        let wy2 = y2 * self.w();
        let wz2 = z2 * self.w();
        let wx2 = x2 * self.w();
        
        return mat3(1f - yy2 - zz2,      xy2 - wz2,      xz2 + wy2,
                         xy2 + wz2, 1f - xx2 - zz2,      yz2 - wx2,
                         xz2 - wy2,      yz2 + wx2, 1f - xx2 - yy2);
    }
    
    #[inline(always)]
    pure fn to_mat4() -> mat4 {
        self.to_mat3().to_mat4()
    }
}

//
//  Convert To String
//
impl quat: ToStr {
    fn to_str() -> ~str {
        fmt!("quat[ %f, %f, %f, %f ]", self.w(), self.x(), self.y(), self.z())
    }
}