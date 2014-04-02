use cgmath::sphere::*;
use cgmath::point::*;
use cgmath::vector::*;
use cgmath::ray::*;
use cgmath::approx::ApproxEq;
use cgmath::intersect::Intersect;

#[test]
fn test_intersection() {
    let sphere = Sphere {center: Point3::new(0f64,0f64,0f64), radius: 1f64};
    let r0 = Ray::new(Point3::new(0f64, 0f64, 5f64), Vec3::new(0f64, 0f64, -5f64).normalize());
    let r1 = Ray::new(Point3::new(1f64.cos(), 0f64, 5f64), Vec3::new(0f64, 0f64, -5f64).normalize());
    let r2 = Ray::new(Point3::new(1f64, 0f64, 5f64), Vec3::new(0f64, 0f64, -5f64).normalize());
    let r3 = Ray::new(Point3::new(2f64, 0f64, 5f64), Vec3::new(0f64, 0f64, -5f64).normalize());
    assert_eq!((sphere,r0).intersection(), Some(Point3::new(0f64, 0f64, 1f64)));
    assert!((sphere,r1).intersection().unwrap().approx_eq( &Point3::new(1f64.cos(), 0f64, 1f64.sin()) ));
    assert_eq!((sphere,r2).intersection(), Some(Point3::new(1f64, 0f64, 0f64)));
    assert_eq!((sphere,r3).intersection(), None);
}
