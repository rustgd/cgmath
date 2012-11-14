// TODO

pub trait EulerAngles<T> {
    // to_mat3
    // to_mat4
    // to_quat
}

pub struct Euler<T> { x: T, y: T, z: T }    // pitch / yaw / roll

pub mod Euler {
    #[inline(always)]
    pub pure fn new<T>(x: T, y: T, z: T) -> Euler<T> {
        Euler { x: move x, y: move y, z: move z }
    }
    
    // from_mat3
    // from_quat
}

pub impl<T> Euler<T>: EulerAngles {}