//! Спека лабы 26. Эти тесты — твоё ТЗ: читай их и пиши код в src/lib.rs.

use l26_mlp::*;

#[test]
fn forward_shape_and_range() {
    let net = Mlp::new(8, 1);
    let (h, p) = net.forward(&[1.0, 0.0]);
    assert_eq!(h.len(), 8);
    assert!(p > 0.0 && p < 1.0, "p = {p}");
    assert!(h.iter().all(|v| *v >= 0.0));
}

#[test]
fn trains_xor() {
    let (net, history) = train_xor(8, 0.1, 10_000, 1);
    let first = *history.first().unwrap();
    let last = *history.last().unwrap();
    assert!(last < first, "loss должен убывать: {first} -> {last}");
    assert!(last < 0.01, "loss = {last}");
    assert_eq!(accuracy(&net), 1.0);
}
