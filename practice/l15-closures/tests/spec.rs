//! Спека лабы 15. Эти тесты — твоё ТЗ: читай их и пиши код в src/lib.rs.

use l15_closures::*;

#[test]
fn applies_twice() {
    assert_eq!(apply_twice(|n| n + 1, 5), 7);
    assert_eq!(apply_twice(|n| n * n, 3), 81);
}

#[test]
fn adder_captures() {
    let add5 = make_adder(5);
    assert_eq!(add5(10), 15);
    assert_eq!(add5(-3), 2);

    let add10 = make_adder(10);
    assert_eq!(add10(0), 10);
}

#[test]
fn counts_with_predicate() {
    let values = [1, 2, 3, 4, 5];
    assert_eq!(count_if(&values, |n| *n == 2), 1);
    assert_eq!(count_if(&values, |n| *n > 2), 3);
    assert_eq!(count_if(&[], |_| true), 0);
}

#[test]
fn counter_keeps_state() {
    let mut next = make_counter();
    assert_eq!(next(), 1);
    assert_eq!(next(), 2);
    assert_eq!(next(), 3);
}

#[test]
fn counter_is_independent() {
    let mut a = make_counter();
    let mut b = make_counter();
    a();
    a();
    assert_eq!(a(), 3);
    assert_eq!(b(), 1);
}
