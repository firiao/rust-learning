//! Спека лабы 7. Эти тесты — твоё ТЗ: читай их и пиши код в src/lib.rs.

use l07_structs::*;

#[test]
fn point_distance() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(3.0, 4.0);
    assert_eq!(a.distance_to(&b), 5.0);
    assert_eq!(b.distance_to(&a), 5.0);
}

#[test]
fn point_translate() {
    let mut p = Point::new(1.0, 2.0);
    p.translate(3.0, -1.0);
    assert_eq!(p, Point::new(4.0, 1.0));
}

#[test]
fn rectangle_measurements() {
    let r = Rectangle::new(3.0, 4.0);
    assert_eq!(r.area(), 12.0);
    assert_eq!(r.perimeter(), 14.0);
    assert!(!r.is_square());
    assert!(Rectangle::new(5.0, 5.0).is_square());
}

#[test]
fn rectangle_scale() {
    let mut r = Rectangle::new(2.0, 3.0);
    r.scale(2.0);
    assert_eq!(r, Rectangle::new(4.0, 6.0));
    assert_eq!(r.area(), 24.0);
}

#[test]
fn derived_traits_work() {
    let r = Rectangle::new(1.0, 1.0);
    let copy = r; // Copy: r остаётся живым
    assert_eq!(r, copy);
    assert_eq!(format!("{r:?}"), "Rectangle { width: 1.0, height: 1.0 }");
}
