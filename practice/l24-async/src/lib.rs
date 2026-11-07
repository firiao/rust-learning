//! `async`/`await` и tokio.

use std::time::Duration;

/// Асинхронно подождать чуть-чуть и вернуть `value * 2`.
pub async fn double_after(yield_ms: u64, value: i64) -> i64 {
    todo!()
}

/// Асинхронно сложить все значения.
pub async fn sum_all(values: Vec<i64>) -> i64 {
    todo!()
}

/// Запустить async-код из обычной (синхронной) функции.
pub fn run_blocking() -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
