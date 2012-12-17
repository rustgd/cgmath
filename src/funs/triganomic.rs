/**
 * Angle and Trigonometry Functions
 *
 * This module corresponds to Section 8.1 of the [GLSL 4.30.6 specification]
 * (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).
 */

use numeric::funs::*;
use numeric::traits::*;
use numeric::types::angle::{Angle, Radians};
use numeric::types::float::Float;

use vec::{Vec3, Vec2, Vec4};

// // Trig

// pub impl<T:Copy Float, A:Copy Angle<T>> Vec2<A>: Trig<Vec2<T>>  {
//     #[inline(always)]
//     pure fn sin(&self) -> Vec2<T> {
//         Vec2::new(sin(&self[0]),
//                   sin(&self[1]))
//     }
    
//     #[inline(always)]
//     pure fn cos(&self) -> Vec2<T> {
//         Vec2::new(cos(&self[0]),
//                   cos(&self[1]))
//     }
    
//     #[inline(always)]
//     pure fn tan(&self) -> Vec2<T> {
//         Vec2::new(tan(&self[0]),
//                   tan(&self[1]))
//     }
// }

// pub impl<T:Copy Float, A:Copy Angle<T>> Vec3<A>: Trig<Vec3<T>>  {
//     #[inline(always)]
//     pure fn sin(&self) -> Vec3<T> {
//         Vec3::new(sin(&self[0]),
//                   sin(&self[1]),
//                   sin(&self[2]))
//     }
    
//     #[inline(always)]
//     pure fn cos(&self) -> Vec3<T> {
//         Vec3::new(cos(&self[0]),
//                   cos(&self[1]),
//                   cos(&self[2]))
//     }
    
//     #[inline(always)]
//     pure fn tan(&self) -> Vec3<T> {
//         Vec3::new(tan(&self[0]),
//                   tan(&self[1]),
//                   tan(&self[2]))
//     }
// }

// pub impl<T:Copy Float, A:Copy Angle<T>> Vec4<A>: Trig<Vec4<T>>  {
//     #[inline(always)]
//     pure fn sin(&self) -> Vec4<T> {
//         Vec4::new(sin(&self[0]),
//                   sin(&self[1]),
//                   sin(&self[2]),
//                   sin(&self[3]))
//     }
    
//     #[inline(always)]
//     pure fn cos(&self) -> Vec4<T> {
//         Vec4::new(cos(&self[0]),
//                   cos(&self[1]),
//                   cos(&self[2]),
//                   cos(&self[3]))
//     }
    
//     #[inline(always)]
//     pure fn tan(&self) -> Vec4<T> {
//         Vec4::new(tan(&self[0]),
//                   tan(&self[1]),
//                   tan(&self[2]),
//                   tan(&self[3]))
//     }
// }

// // InvTrig

// pub impl<T:Copy Float InvTrig> Vec2<T>: InvTrig<Vec2<Radians<T>>>  {
//     #[inline(always)]
//     pure fn asin(&self) -> Vec2<Radians<T>> {
//         Vec2::new(asin(&self[0]),
//                   asin(&self[1]))
//     }
    
//     #[inline(always)]
//     pure fn acos(&self) -> Vec2<Radians<T>> {
//         Vec2::new(acos(&self[0]),
//                   acos(&self[1]))
//     }
    
//     #[inline(always)]
//     pure fn atan(&self) -> Vec2<Radians<T>> {
//         Vec2::new(atan(&self[0]),
//                   atan(&self[1]))
//     }
// }

// pub impl<T:Copy Float InvTrig> Vec3<T>: InvTrig<Vec3<Radians<T>>>  {
//     #[inline(always)]
//     pure fn asin(&self) -> Vec3<Radians<T>> {
//         Vec3::new(asin(&self[0]),
//                   asin(&self[1]),
//                   asin(&self[2]))
//     }
    
//     #[inline(always)]
//     pure fn acos(&self) -> Vec3<Radians<T>> {
//         Vec3::new(acos(&self[0]),
//                   acos(&self[1]),
//                   acos(&self[2]))
//     }
    
//     #[inline(always)]
//     pure fn atan(&self) -> Vec3<Radians<T>> {
//         Vec3::new(atan(&self[0]),
//                   atan(&self[1]),
//                   atan(&self[2]))
//     }
// }

// pub impl<T:Copy Float InvTrig> Vec4<T>: InvTrig<Vec4<Radians<T>>>  {
//     #[inline(always)]
//     pure fn asin(&self) -> Vec4<Radians<T>> {
//         Vec4::new(asin(&self[0]),
//                   asin(&self[1]),
//                   asin(&self[2]),
//                   asin(&self[3]))
//     }
    
//     #[inline(always)]
//     pure fn acos(&self) -> Vec4<Radians<T>> {
//         Vec4::new(acos(&self[0]),
//                   acos(&self[1]),
//                   acos(&self[2]),
//                   acos(&self[3]))
//     }
    
//     #[inline(always)]
//     pure fn atan(&self) -> Vec4<Radians<T>> {
//         Vec4::new(atan(&self[0]),
//                   atan(&self[1]),
//                   atan(&self[2]),
//                   atan(&self[3]))
//     }
// }

// Hyp

pub impl <T:Copy Hyp> Vec2<T>: Hyp {
    #[inline(always)]
    pure fn sinh(&self) -> Vec2<T> {
        Vec2::new(sinh(&self[0]),
                  sinh(&self[1]))
    }
    
    #[inline(always)]
    pure fn cosh(&self) -> Vec2<T> {
        Vec2::new(cosh(&self[0]),
                  cosh(&self[1]))
    }
    
    #[inline(always)]
    pure fn tanh(&self) -> Vec2<T> {
        Vec2::new(tanh(&self[0]),
                  tanh(&self[1]))
    }
}

pub impl <T:Copy Hyp> Vec3<T>: Hyp {
    #[inline(always)]
    pure fn sinh(&self) -> Vec3<T> {
        Vec3::new(sinh(&self[0]),
                  sinh(&self[1]),
                  sinh(&self[2]))
    }
    
    #[inline(always)]
    pure fn cosh(&self) -> Vec3<T> {
        Vec3::new(cosh(&self[0]),
                  cosh(&self[1]),
                  cosh(&self[2]))
    }
    
    #[inline(always)]
    pure fn tanh(&self) -> Vec3<T> {
        Vec3::new(tanh(&self[0]),
                  tanh(&self[1]),
                  tanh(&self[2]))
    }
}

pub impl <T:Copy Hyp> Vec4<T>: Hyp  {
    #[inline(always)]
    pure fn sinh(&self) -> Vec4<T> {
        Vec4::new(sinh(&self[0]),
                  sinh(&self[1]),
                  sinh(&self[2]),
                  sinh(&self[3]))
    }
    
    #[inline(always)]
    pure fn cosh(&self) -> Vec4<T> {
        Vec4::new(cosh(&self[0]),
                  cosh(&self[1]),
                  cosh(&self[2]),
                  cosh(&self[3]))
    }
    
    #[inline(always)]
    pure fn tanh(&self) -> Vec4<T> {
        Vec4::new(tanh(&self[0]),
                  tanh(&self[1]),
                  tanh(&self[2]),
                  tanh(&self[3]))
    }
}