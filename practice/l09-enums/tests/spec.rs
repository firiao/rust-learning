//! Спека лабы 9. Эти тесты — твоё ТЗ: читай их и пиши код в src/lib.rs.

use l09_enums::*;

#[test]
fn areas() {
    assert_eq!(Shape::Rectangle { width: 3.0, height: 4.0 }.area(), 12.0);
    assert_eq!(Shape::Empty.area(), 0.0);
    let circle = Shape::Circle { radius: 1.0 }.area();
    assert!((circle - std::f64::consts::PI).abs() < 1e-9);
}

#[test]
fn has_area_never_panics() {
    assert!(Shape::Circle { radius: 1.0 }.has_area());
    assert!(!Shape::Empty.has_area());
    assert!(!Shape::Rectangle { width: 0.0, height: 5.0 }.has_area());
}

#[test]
fn divides_safely() {
    assert_eq!(safe_divide(10.0, 2.0), Some(5.0));
    assert_eq!(safe_divide(1.0, 0.0), None);
}

#[test]
fn finds_even() {
    assert_eq!(first_even_index(&[1, 3, 4, 6]), Some(2));
    assert_eq!(first_even_index(&[1, 3, 5]), None);
    assert_eq!(first_even_index(&[]), None);
}

#[test]
fn finds_longest_word() {
    assert_eq!(longest_word("я учу rust"), Some("rust"));
    assert_eq!(longest_word(""), None);
    assert_eq!(longest_word("   "), None);
}

#[test]
fn describes_option() {
    assert_eq!(describe(Some(7)), "есть число 7");
    assert_eq!(describe(Some(0)), "есть ноль");
    assert_eq!(describe(Some(-3)), "есть отрицательное -3");
    assert_eq!(describe(None), "пусто");
}
