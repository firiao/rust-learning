//! Спека лабы 8. Эти тесты — твоё ТЗ: читай их и пиши код в src/lib.rs.

use l08_match::*;

#[test]
fn grades_by_range() {
    assert_eq!(grade(100), 'A');
    assert_eq!(grade(90), 'A');
    assert_eq!(grade(89), 'B');
    assert_eq!(grade(60), 'C');
    assert_eq!(grade(59), 'F');
    assert_eq!(grade(0), 'F');
}

#[test]
fn describes_points() {
    assert_eq!(describe_point(0, 0), "начало координат");
    assert_eq!(describe_point(0, 5), "на оси Y");
    assert_eq!(describe_point(3, 0), "на оси X");
    assert_eq!(describe_point(3, 3), "на диагонали");
    assert_eq!(describe_point(2, 5), "обычная точка");
}

#[test]
fn computes() {
    assert_eq!(apply('+', 2, 3), 5);
    assert_eq!(apply('-', 2, 3), -1);
    assert_eq!(apply('*', 2, 3), 6);
    assert_eq!(apply('/', 6, 3), 2);
    assert_eq!(apply('^', 2, 3), 0);
}
