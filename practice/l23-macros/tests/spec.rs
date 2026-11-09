//! Спека лабы 23. Эти тесты — твоё ТЗ: читай их и пиши код в src/lib.rs.

use l23_macros::*;

#[test]
fn sums() {
    assert_eq!(sum_all!(1, 2, 3), 6);
    assert_eq!(sum_all!(5), 5);
}

#[test]
fn finds_max() {
    assert_eq!(max_of!(3, 7, 2), 7);
    assert_eq!(max_of!(10, 1), 10);
}

#[test]
fn makes_strings() {
    assert_eq!(strings!("а", "б"), vec!["а".to_string(), "б".to_string()]);
}
