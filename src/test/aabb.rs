use cgmath::aabb::{Aabb2, Aabb3};
use cgmath::point::{Point2, Point3};

#[test]
fn test_aabb() {
    let aabb = Aabb2::new(&Point2::new(-20f64, 30f64), &Point2::new(10f64, -10f64));
    assert_eq!(aabb.min, Point2::new(-20f64, -10f64));
    assert_eq!(aabb.max, Point2::new(10f64, 30f64));

    let aabb = Aabb3::new(&Point3::new(-20f64, 30f64, 0f64), &Point3::new(10f64, -10f64, -5f64));
    assert_eq!(aabb.min, Point3::new(-20f64, -10f64, -5f64));
    assert_eq!(aabb.max, Point3::new(10f64, 30f64, 0f64));
}
