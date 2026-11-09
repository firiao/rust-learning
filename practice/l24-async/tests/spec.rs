//! Спека лабы 24. Эти тесты — твоё ТЗ: читай их и пиши код в src/lib.rs.

use l24_async::*;

#[tokio::test]
async fn doubles() {
    assert_eq!(double_after(1, 21).await, 42);
}

#[tokio::test]
async fn sums() {
    assert_eq!(sum_all(vec![1, 2, 3]).await, 6);
    assert_eq!(sum_all(vec![]).await, 0);
}

#[test]
fn runs_async_from_sync() {
    assert_eq!(run_blocking(), 42);
}
