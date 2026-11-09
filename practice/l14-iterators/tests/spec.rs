//! Спека лабы 14. Эти тесты — твоё ТЗ: читай их и пиши код в src/lib.rs.

use l14_iterators::*;

#[test]
fn computes_mean() {
    assert_eq!(mean(&[2.0, 4.0]), Some(3.0));
    assert_eq!(mean(&[]), None);
}

#[test]
fn computes_mse() {
    let t = [3.0, -0.5, 2.0, 7.0];
    let p = [2.5, 0.0, 2.0, 8.0];
    assert_eq!(mse(&t, &p), Some(0.375));
    assert_eq!(mse(&[], &[]), None);
    assert_eq!(mse(&[1.0], &[1.0, 2.0]), None);
}

#[test]
fn squares_evens() {
    assert_eq!(evens_squared(&[1, 2, 3, 4, 5, 6]), vec![4, 16, 36]);
    assert!(evens_squared(&[1, 3, 5]).is_empty());
}

#[test]
fn finds_first_big() {
    assert_eq!(first_big(&[1, 2, 10, 20], 5), Some(10));
    assert_eq!(first_big(&[1, 2], 5), None);
}

#[test]
fn multiplies() {
    assert_eq!(product(&[2, 3, 4]), 24);
    assert_eq!(product(&[]), 1);
}
