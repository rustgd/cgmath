use cgmath::aabb::*;
use cgmath::point::{Point2, Point3};
use cgmath::vector::{Vec2, Vec3};

#[test]
fn test_aabb() {
    let aabb = Aabb2::new(Point2::new(-20, 30), Point2::new(10, -10));
    assert_eq!(aabb.min(), &Point2::new(-20, -10));
    assert_eq!(aabb.max(), &Point2::new(10, 30));
    assert_eq!(aabb.dim(), Vec2::new(30, 40));
    assert_eq!(aabb.volume(), 30 * 40);
    assert_eq!(aabb.center(), Point2::new(-5, 10));

    assert!(aabb.contains(&Point2::new(0, 0)));
    assert!(!aabb.contains(&Point2::new(-50, -50)));
    assert!(!aabb.contains(&Point2::new(50, 50)));

    assert_eq!(aabb.grow(&Point2::new(0, 0)), aabb);
    assert_eq!(aabb.grow(&Point2::new(100, 100)),
        Aabb2::new(Point2::new(-20, -10), Point2::new(100, 100)));
    assert_eq!(aabb.grow(&Point2::new(-100, -100)),
        Aabb2::new(Point2::new(-100, -100), Point2::new(10, 30)));

    let aabb = Aabb3::new(Point3::new(-20, 30, 5), Point3::new(10, -10, -5));
    assert_eq!(aabb.min(), &Point3::new(-20, -10, -5));
    assert_eq!(aabb.max(), &Point3::new(10, 30, 5));
    assert_eq!(aabb.dim(), Vec3::new(30, 40, 10));
    assert_eq!(aabb.volume(), 30 * 40 * 10);
    assert_eq!(aabb.center(), Point3::new(-5, 10, 0));

    assert!(aabb.contains(&Point3::new(0, 0, 0)));
    assert!(!aabb.contains(&Point3::new(-100, 0, 0)));
    assert!(!aabb.contains(&Point3::new(100, 0, 0)));
    assert!(aabb.contains(&Point3::new(9, 29, -1)));
    assert!(!aabb.contains(&Point3::new(10, 30, 5)));
    assert!(aabb.contains(&Point3::new(-20, -10, -5)));
    assert!(!aabb.contains(&Point3::new(-21, -11, -6)));

    assert_eq!(aabb.add_v(&Vec3::new(1, 2, 3)),
        Aabb3::new(Point3::new(-19, 32, 8), Point3::new(11, -8, -2)));

    assert_eq!(aabb.mul_s(2),
        Aabb3::new(Point3::new(-40, -20, -10), Point3::new(20, 60, 10)));

    assert_eq!(aabb.mul_v(&Vec3::new(1, 2, 3)),
        Aabb3::new(Point3::new(-20, -20, -15), Point3::new(10, 60, 15)));
}
