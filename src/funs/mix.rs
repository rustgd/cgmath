pub trait Mix<B> {
    pure fn mix(y: &self, a: &self) -> self;
    pure fn mixb(y: &self, a: &B)   -> self;
}

#[inline(always)] pub pure fn mix<T:Mix<B>, B>(x: &T, y: &T, a: &T)  -> T { x.mix(y, a) }
#[inline(always)] pub pure fn mixb<T:Mix<B>, B>(x: &T, y: &T, a: &B) -> T { x.mixb(y, a) }

pub impl f32: Mix<bool> {
    #[inline(always)] pure fn mix(y: &f32, a: &f32)   -> f32 { self * (1f32 - (*a)) + (*y) * (*a) }
    #[inline(always)] pure fn mixb(y: &f32, a: &bool) -> f32 { if *a { *y } else { self } }
}

pub impl f64: Mix<bool> {
    #[inline(always)] pure fn mix(y: &f64, a: &f64)   -> f64 { self * (1f64 - (*a)) + (*y) * (*a) }
    #[inline(always)] pure fn mixb(y: &f64, a: &bool) -> f64 { if *a { *y } else { self } }
}

pub impl float: Mix<bool> {
    #[inline(always)] pure fn mix(y: &float, a: &float) -> float { self * (1f - (*a)) + (*y) * (*a) }
    #[inline(always)] pure fn mixb(y: &float, a: &bool) -> float { if *a { *y } else { self } }
}