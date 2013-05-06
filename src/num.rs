pub trait NumAssign {
    fn add_assign(&mut self, other: &Self);
    fn sub_assign(&mut self, other: &Self);
    fn mul_assign(&mut self, other: &Self);
    fn div_assign(&mut self, other: &Self);
    fn rem_assign(&mut self, other: &Self);
}

macro_rules! impl_NumAssign(
    ($T:ty) => (
        impl NumAssign for $T {
            #[inline(always)] fn add_assign(&mut self, other: &$T) { *self += *other }
            #[inline(always)] fn sub_assign(&mut self, other: &$T) { *self -= *other }
            #[inline(always)] fn mul_assign(&mut self, other: &$T) { *self *= *other }
            #[inline(always)] fn div_assign(&mut self, other: &$T) { *self /= *other }
            #[inline(always)] fn rem_assign(&mut self, other: &$T) { *self %= *other }
        }
    )
)

impl_NumAssign!(float)
impl_NumAssign!(f32)
impl_NumAssign!(f64)

impl_NumAssign!(int)
impl_NumAssign!(i8)
impl_NumAssign!(i16)
impl_NumAssign!(i32)
impl_NumAssign!(i64)

impl_NumAssign!(uint)
impl_NumAssign!(u8)
impl_NumAssign!(u16)
impl_NumAssign!(u32)
impl_NumAssign!(u64)