//! Спека лабы 28. Эти тесты — твоё ТЗ: читай их и пиши код в src/lib.rs.

use l28_inference::*;

#[test]
fn softmax_is_distribution() {
    let p = softmax(&[1.0, 2.0, 3.0]);
    assert_eq!(p.len(), 3);
    let sum: f64 = p.iter().sum();
    assert!((sum - 1.0).abs() < 1e-9, "сумма = {sum}");
    assert!(p[2] > p[1] && p[1] > p[0]);
}

#[test]
fn softmax_survives_big_numbers() {
    let p = softmax(&[1000.0, 1001.0]);
    assert!(p.iter().all(|v| v.is_finite()), "{p:?}");
    let sum: f64 = p.iter().sum();
    assert!((sum - 1.0).abs() < 1e-9);
}

#[test]
fn argmax_finds_index() {
    assert_eq!(argmax(&[0.1, 2.0, 0.7]), Some(1));
    assert_eq!(argmax(&[5.0]), Some(0));
    assert_eq!(argmax(&[]), None);
}

#[test]
fn parses_scores() {
    assert_eq!(parse_scores("[0.1, 2.0, 0.7]").unwrap(), vec![0.1, 2.0, 0.7]);
    assert!(parse_scores("[1, \"два\"]").is_err());
}

#[test]
fn picks_top_class() {
    let (idx, prob) = top_class("[0.1, 2.0, 0.7]").unwrap();
    assert_eq!(idx, 1);
    assert!(prob > 0.5 && prob < 1.0, "p = {prob}");
}
