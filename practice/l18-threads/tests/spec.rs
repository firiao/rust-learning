//! Спека лабы 18. Эти тесты — твоё ТЗ: читай их и пиши код в src/lib.rs.

use l18_threads::*;

#[test]
fn sums_in_parallel() {
    let data: Vec<i64> = (1..=1000).collect();
    let expected: i64 = data.iter().sum();
    assert_eq!(parallel_sum(&data, 4), expected);
    assert_eq!(parallel_sum(&data, 1), expected);
    assert_eq!(parallel_sum(&data, 0), expected);
    assert_eq!(parallel_sum(&[], 4), 0);
}

#[test]
fn handles_odd_sizes() {
    let data = vec![1, 2, 3, 4, 5, 6, 7];
    assert_eq!(parallel_sum(&data, 3), 28);
    assert_eq!(parallel_sum(&data, 100), 28);
}

#[test]
fn sums_via_channel() {
    assert_eq!(sum_via_channel(10), 55);
    assert_eq!(sum_via_channel(1), 1);
}

#[test]
fn increments_shared_counter() {
    let counter = Arc::new(Mutex::new(0));
    increment_concurrently(Arc::clone(&counter), 4, 100);
    assert_eq!(*counter.lock().unwrap(), 400);
}

#[test]
fn counter_survives_zero_threads() {
    let counter = Arc::new(Mutex::new(0));
    increment_concurrently(Arc::clone(&counter), 0, 100);
    assert_eq!(*counter.lock().unwrap(), 0);
}
