use mat::Mat3;
use quat::Quat;
use vec::Vec3;

/**
 * A trait that includes some common rotation methods, constructors and conversions
 */
pub trait Rotation<T> {
    static pure fn from<R: Rotation<T>>(rot: R) -> self;
    
    static pure fn from_angle_x(radians: T) -> self;
    static pure fn from_angle_y(radians: T) -> self;
    static pure fn from_angle_z(radians: T) -> self;
    static pure fn from_angle_xyz(radians_x: T, radians_y: T, radians_z: T) -> self;
    static pure fn from_angle_axis(radians: T, axis: &Vec3<T>) -> self;
    static pure fn from_axes(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> self;
    static pure fn look_at(dir: &Vec3<T>, up: &Vec3<T>) -> self;
    
    pure fn concat(&self, other: &self) -> self;
    pure fn rotate_vec(&self, vec: &Vec3<T>) -> Vec3<T>;
    
    pure fn to_mat3(&self) -> Mat3<T>;
    pure fn to_quat(&self) -> Quat<T>;
    // pure fn to_euler(&self) -> Euler<T>;
}