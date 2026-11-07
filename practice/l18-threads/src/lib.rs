//! Потоки: `scope`, каналы, `Arc<Mutex>`.

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// Сумма среза, посчитанная в `threads` потоках.
///
/// `threads = 0` или `1` — обычная однопоточная сумма.
pub fn parallel_sum(values: &[i64], threads: usize) -> i64 {
    todo!()
}

/// Сумма 1..=n, посчитанная в отдельном потоке и переданная через канал.
pub fn sum_via_channel(n: u64) -> u64 {
    todo!()
}

/// Увеличивает счётчик `threads` потоками, по `times` раз каждый.
pub fn increment_concurrently(counter: Arc<Mutex<i64>>, threads: usize, times: usize) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
