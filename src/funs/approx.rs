use num::cast::cast;

pub trait Approx {
    pure fn floor() -> self;
    pure fn trunc() -> self;
    pure fn round() -> self;
    // pure fn roundEven() -> self;
    pure fn ceil()  -> self;
    // pure fn fract() -> self;
}

#[inline(always)] pub pure fn floor<T:Approx>(x: T) -> T { x.floor() }
#[inline(always)] pub pure fn trunc<T:Approx>(x: T) -> T { x.trunc() }
#[inline(always)] pub pure fn round<T:Approx>(x: T) -> T { x.round() }
// #[inline(always)] pub pure fn roundEven<T:Approx>(x: T) -> T { x.roundEven() }
#[inline(always)] pub pure fn ceil<T:Approx>(x: T)  -> T { x.ceil() }
// #[inline(always)] pub pure fn fract<T:Approx>(x: T) -> T { x.fract() }

pub impl f32: Approx {
    #[inline(always)] pure fn floor() -> f32 { cast(cmath::c_float_utils::floor(self)) }
    #[inline(always)] pure fn trunc() -> f32 { cast(cmath::c_float_utils::trunc(self)) }
    #[inline(always)] pure fn round() -> f32 { cast(cmath::c_float_utils::round(self)) }
    // #[inline(always)] pure fn roundEven() -> f32 {}
    #[inline(always)] pure fn ceil()  -> f32 { cast(cmath::c_float_utils::ceil(self)) }
    // #[inline(always)] pure fn fract() -> f32 {}
}

pub impl f64: Approx {
    #[inline(always)] pure fn floor() -> f64 { cast(cmath::c_double_utils::floor(self)) }
    #[inline(always)] pure fn trunc() -> f64 { cast(cmath::c_double_utils::trunc(self)) }
    #[inline(always)] pure fn round() -> f64 { cast(cmath::c_double_utils::round(self)) }
    // #[inline(always)] pure fn roundEven() -> f64 {}
    #[inline(always)] pure fn ceil()  -> f64 { cast(cmath::c_double_utils::ceil(self)) }
    // #[inline(always)] pure fn fract() -> f64 {}
}

pub impl float: Approx {
    #[inline(always)] pure fn floor() -> float { cast(cmath::c_float_utils::floor(cast(self))) }
    #[inline(always)] pure fn trunc() -> float { cast(cmath::c_float_utils::trunc(cast(self))) }
    #[inline(always)] pure fn round() -> float { cast(cmath::c_float_utils::round(cast(self))) }
    // #[inline(always)] pure fn roundEven() -> float {}
    #[inline(always)] pure fn ceil()  -> float { cast(cmath::c_float_utils::ceil(cast(self))) }
    // #[inline(always)] pure fn fract() -> float {}
}