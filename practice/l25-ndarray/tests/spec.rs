//! Спека лабы 25. Эти тесты — твоё ТЗ: читай их и пиши код в src/lib.rs.

use l25_ndarray::*;

#[test]
fn predicts_with_known_weights() {
    let (x, _) = dataset();
    let w = Array1::from_vec(vec![3.0, 2.0]);
    let pred = predict(&x, &w, 1.0);
    assert_eq!(pred.to_vec(), vec![4.0, 3.0, 6.0, 9.0]);
}

#[test]
fn computes_mse() {
    let pred = Array1::from_vec(vec![1.0, 2.0]);
    let y = Array1::from_vec(vec![2.0, 4.0]);
    assert!((mse(&pred, &y) - 2.5).abs() < 1e-9);
}

#[test]
fn one_step_reduces_loss() {
    let (x, y) = dataset();
    let w = Array1::zeros(2);
    let (w2, b2, loss) = step(&x, &y, &w, 0.0, 0.01);
    let before = mse(&predict(&x, &w, 0.0), &y);
    let after = mse(&predict(&x, &w2, b2), &y);
    assert!(after < before, "loss не уменьшился: {before} -> {after}");
    assert!(loss >= 0.0);
}

#[test]
fn trains_line() {
    let (x, y) = dataset();
    let (w, b) = train(&x, &y, 0.01, 2000);
    assert!((w[0] - 3.0).abs() < 0.05, "w[0] = {}", w[0]);
    assert!((w[1] - 2.0).abs() < 0.05, "w[1] = {}", w[1]);
    assert!((b - 1.0).abs() < 0.05, "b = {b}");
}
