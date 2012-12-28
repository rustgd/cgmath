use numeric::types::angle::Angle;

use mat::Mat3;
use quat::Quat;
use vec::Vec3;

/**
 * A trait that includes some common rotation methods, constructors and conversions
 */
pub trait Rotation<T> {
    static pure fn from<R: Rotation<T>>(rot: R) -> self;
    
    static pure fn from_angle_x<A:Angle<T>>(theta: A) -> self;
    static pure fn from_angle_y<A:Angle<T>>(theta: A) -> self;
    static pure fn from_angle_z<A:Angle<T>>(theta: A) -> self;
    static pure fn from_angle_xyz<A:Angle<T>>(x: A, y: A, z: A) -> self;
    static pure fn from_angle_axis<A:Angle<T>>(theta: A, axis: &Vec3<T>) -> self;
    static pure fn from_axes(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> self;
    static pure fn look_at(dir: &Vec3<T>, up: &Vec3<T>) -> self;
    
    pure fn concat(&self, other: &self) -> self;
    pure fn rotate_vec(&self, vec: &Vec3<T>) -> Vec3<T>;
    
    pure fn to_mat3(&self) -> Mat3<T>;
    pure fn to_quat(&self) -> Quat<T>;
    // pure fn to_euler(&self) -> Euler<T>;
}