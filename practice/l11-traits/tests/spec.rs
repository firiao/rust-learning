//! Спека лабы 11. Эти тесты — твоё ТЗ: читай их и пиши код в src/lib.rs.

use l11_traits::*;

fn sample() -> (Circle, Square) {
    (Circle { radius: 1.0 }, Square { side: 2.0 })
}

#[test]
fn areas_are_correct() {
    let (c, s) = sample();
    assert!((c.area() - std::f64::consts::PI).abs() < 1e-9);
    assert_eq!(s.area(), 4.0);
}

#[test]
fn default_method_works() {
    let (c, _) = sample();
    assert_eq!(c.describe(), "круг площадью 3.14");
}

#[test]
fn sums_heterogeneous_list() {
    let (c, s) = sample();
    let shapes: [&dyn Shape; 2] = [&c, &s];
    let total = total_area(&shapes);
    assert!((total - (std::f64::consts::PI + 4.0)).abs() < 1e-9);
    assert_eq!(total_area(&[]), 0.0);
}

#[test]
fn describes_heterogeneous_list() {
    let (c, s) = sample();
    let shapes: [&dyn Shape; 2] = [&c, &s];
    let text = describe_all(&shapes);
    assert_eq!(text, "круг площадью 3.14\nквадрат площадью 4.00");
}
