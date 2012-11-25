use num::cast::cast;
use vec::{Vec2, Vec3, Vec4};

pub trait Approx {
    pure fn floor() -> self;
    pure fn trunc() -> self;
    pure fn round() -> self;
    // pure fn roundEven() -> self;
    pure fn ceil()  -> self;
    pure fn fract() -> self;
}

#[inline(always)] pub pure fn floor<T:Approx>(x: &T) -> T { x.floor() }
#[inline(always)] pub pure fn trunc<T:Approx>(x: &T) -> T { x.trunc() }
#[inline(always)] pub pure fn round<T:Approx>(x: &T) -> T { x.round() }
// #[inline(always)] pub pure fn roundEven<T:Approx>(x: &T) -> T { x.roundEven() }
#[inline(always)] pub pure fn ceil<T:Approx>(x: &T)  -> T { x.ceil() }
#[inline(always)] pub pure fn fract<T:Approx>(x: &T) -> T { x.fract() }

pub impl f32: Approx {
    #[inline(always)] pure fn floor() -> f32 { cast(cmath::c_float_utils::floor(self)) }
    #[inline(always)] pure fn trunc() -> f32 { cast(cmath::c_float_utils::trunc(self)) }
    #[inline(always)] pure fn round() -> f32 { cast(cmath::c_float_utils::round(self)) }
    // #[inline(always)] pure fn roundEven() -> f32 {}
    #[inline(always)] pure fn ceil()  -> f32 { cast(cmath::c_float_utils::ceil(self)) }
    #[inline(always)] pure fn fract() -> f32 { self - floor(&self) }
}

pub impl f64: Approx {
    #[inline(always)] pure fn floor() -> f64 { cast(cmath::c_double_utils::floor(self)) }
    #[inline(always)] pure fn trunc() -> f64 { cast(cmath::c_double_utils::trunc(self)) }
    #[inline(always)] pure fn round() -> f64 { cast(cmath::c_double_utils::round(self)) }
    // #[inline(always)] pure fn roundEven() -> f64 {}
    #[inline(always)] pure fn ceil()  -> f64 { cast(cmath::c_double_utils::ceil(self)) }
    #[inline(always)] pure fn fract() -> f64 { self - floor(&self) }
}

pub impl float: Approx {
    #[inline(always)] pure fn floor() -> float { cast(cmath::c_float_utils::floor(cast(self))) }
    #[inline(always)] pure fn trunc() -> float { cast(cmath::c_float_utils::trunc(cast(self))) }
    #[inline(always)] pure fn round() -> float { cast(cmath::c_float_utils::round(cast(self))) }
    // #[inline(always)] pure fn roundEven() -> float {}
    #[inline(always)] pure fn ceil()  -> float { cast(cmath::c_float_utils::ceil(cast(self))) }
    #[inline(always)] pure fn fract() -> float { self - floor(&self) }
}

pub impl<T:Copy Approx> Vec2<T>: Approx {
    #[inline(always)]
    pure fn floor() -> Vec2<T> {
        Vec2::new(floor(&self[0]),
                  floor(&self[1]))
    }
    
    #[inline(always)]
    pure fn trunc() -> Vec2<T> {
        Vec2::new(trunc(&self[0]),
                  trunc(&self[1]))
    }
    
    #[inline(always)]
    pure fn round() -> Vec2<T> {
        Vec2::new(round(&self[0]),
                  round(&self[1]))
    }
    
    // #[inline(always)]
    // pure fn ceil()  -> Vec2<T> {
    //     Vec2::new(roundEven(&self[0]),
    //               roundEven(&self[1]))
    // }
    
    #[inline(always)]
    pure fn ceil()  -> Vec2<T> {
        Vec2::new(ceil(&self[0]),
                  ceil(&self[1]))
    }
    
    #[inline(always)]
    pure fn fract() -> Vec2<T> {
        Vec2::new(fract(&self[0]),
                  fract(&self[1]))
    }
    
}

pub impl<T:Copy Approx> Vec3<T>: Approx {
    #[inline(always)]
    pure fn floor() -> Vec3<T> {
        Vec3::new(floor(&self[0]),
                  floor(&self[1]),
                  floor(&self[2]))
    }
    
    #[inline(always)]
    pure fn trunc() -> Vec3<T> {
        Vec3::new(trunc(&self[0]),
                  trunc(&self[1]),
                  trunc(&self[2]))
    }
    
    #[inline(always)]
    pure fn round() -> Vec3<T> {
        Vec3::new(round(&self[0]),
                  round(&self[1]),
                  round(&self[2]))
    }
    
    // #[inline(always)]
    // pure fn ceil()  -> Vec3<T> {
    //     Vec3::new(roundEven(&self[0]),
    //               roundEven(&self[1]),
    //               roundEven(&self[2]))
    // }
    
    #[inline(always)]
    pure fn ceil()  -> Vec3<T> {
        Vec3::new(ceil(&self[0]),
                  ceil(&self[1]),
                  ceil(&self[2]))
    }
    
    #[inline(always)]
    pure fn fract() -> Vec3<T> {
        Vec3::new(fract(&self[0]),
                  fract(&self[1]),
                  fract(&self[2]))
    }
    
}

pub impl<T:Copy Approx> Vec4<T>: Approx {
    #[inline(always)]
    pure fn floor() -> Vec4<T> {
        Vec4::new(floor(&self[0]),
                  floor(&self[1]),
                  floor(&self[2]),
                  floor(&self[3]))
    }
    
    #[inline(always)]
    pure fn trunc() -> Vec4<T> {
        Vec4::new(trunc(&self[0]),
                  trunc(&self[1]),
                  trunc(&self[2]),
                  trunc(&self[3]))
    }
    
    #[inline(always)]
    pure fn round() -> Vec4<T> {
        Vec4::new(round(&self[0]),
                  round(&self[1]),
                  round(&self[2]),
                  round(&self[3]))
    }
    
    // #[inline(always)]
    // pure fn ceil()  -> Vec4<T> {
    //     Vec4::new(roundEven(&self[0]),
    //               roundEven(&self[1]),
    //               roundEven(&self[2]),
    //               roundEven(&self[3]))
    // }
    
    #[inline(always)]
    pure fn ceil()  -> Vec4<T> {
        Vec4::new(ceil(&self[0]),
                  ceil(&self[1]),
                  ceil(&self[2]),
                  ceil(&self[3]))
    }
    
    #[inline(always)]
    pure fn fract() -> Vec4<T> {
        Vec4::new(fract(&self[0]),
                  fract(&self[1]),
                  fract(&self[2]),
                  fract(&self[3]))
    }
    
}