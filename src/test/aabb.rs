use cgmath::aabb::*;
use cgmath::point::{Point2, Point3};
use cgmath::vector::{Vector2, Vector3};

#[test]
fn test_aabb() {
    let aabb = Aabb2::new(Point2::new(-20i, 30i), Point2::new(10i, -10i));
    assert_eq!(aabb.min(), &Point2::new(-20i, -10i));
    assert_eq!(aabb.max(), &Point2::new(10i, 30i));
    assert_eq!(aabb.dim(), Vector2::new(30i, 40i));
    assert_eq!(aabb.volume(), 30i * 40i);
    assert_eq!(aabb.center(), Point2::new(-5i, 10i));

    assert!(aabb.contains(&Point2::new(0i, 0i)));
    assert!(!aabb.contains(&Point2::new(-50i, -50i)));
    assert!(!aabb.contains(&Point2::new(50i, 50i)));

    assert_eq!(aabb.grow(&Point2::new(0i, 0i)), aabb);
    assert_eq!(aabb.grow(&Point2::new(100i, 100i)),
        Aabb2::new(Point2::new(-20i, -10i), Point2::new(100i, 100i)));
    assert_eq!(aabb.grow(&Point2::new(-100i, -100i)),
        Aabb2::new(Point2::new(-100i, -100i), Point2::new(10i, 30i)));

    let aabb = Aabb3::new(Point3::new(-20i, 30i, 5i), Point3::new(10i, -10i, -5i));
    assert_eq!(aabb.min(), &Point3::new(-20i, -10i, -5i));
    assert_eq!(aabb.max(), &Point3::new(10i, 30i, 5i));
    assert_eq!(aabb.dim(), Vector3::new(30i, 40i, 10i));
    assert_eq!(aabb.volume(), 30i * 40i * 10i);
    assert_eq!(aabb.center(), Point3::new(-5i, 10i, 0i));

    assert!(aabb.contains(&Point3::new(0i, 0i, 0i)));
    assert!(!aabb.contains(&Point3::new(-100i, 0i, 0i)));
    assert!(!aabb.contains(&Point3::new(100i, 0i, 0i)));
    assert!(aabb.contains(&Point3::new(9i, 29i, -1i)));
    assert!(!aabb.contains(&Point3::new(10i, 30i, 5i)));
    assert!(aabb.contains(&Point3::new(-20i, -10i, -5i)));
    assert!(!aabb.contains(&Point3::new(-21i, -11i, -6i)));

    assert_eq!(aabb.add_v(&Vector3::new(1i, 2i, 3i)),
        Aabb3::new(Point3::new(-19i, 32i, 8i), Point3::new(11i, -8i, -2i)));

    assert_eq!(aabb.mul_s(2i),
        Aabb3::new(Point3::new(-40i, -20i, -10i), Point3::new(20i, 60i, 10i)));

    assert_eq!(aabb.mul_v(&Vector3::new(1i, 2i, 3i)),
        Aabb3::new(Point3::new(-20i, -20i, -15i), Point3::new(10i, 60i, 15i)));
}
