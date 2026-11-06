//! «Угадай число» — первая законченная программа курса.
//!
//! Запуск: `cargo run -p guess_game`.

use std::cmp::Ordering;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

/// Маленький xorshift64* генератор.
///
/// Для игры достаточно; в реальных проектах используют крейт `rand`.
struct Rng(u64);

impl Rng {
    fn from_time() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        Self::from_seed(seed)
    }

    fn from_seed(seed: u64) -> Self {
        Self(seed | 1)
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Случайное число в диапазоне `[low, high]`.
    fn range(&mut self, low: u32, high: u32) -> u32 {
        debug_assert!(low <= high);
        let span = high as u64 - low as u64 + 1;
        low + (self.next_u64() % span) as u32
    }
}

fn parse_guess(input: &str) -> Option<u32> {
    input.trim().parse().ok()
}

fn compare_guess(guess: u32, secret: u32) -> Ordering {
    guess.cmp(&secret)
}

fn main() {
    println!("Угадай число от 1 до 100!");
    println!("Введи число и нажми Enter. Для выхода — Ctrl+D или Ctrl+C.\n");

    let mut rng = Rng::from_time();
    let secret = rng.range(1, 100);
    let mut attempts = 0;

    loop {
        print!("Твой вариант: ");
        io::stdout().flush().expect("не удалось сбросить stdout");

        let mut input = String::new();
        let bytes = io::stdin()
            .read_line(&mut input)
            .expect("не удалось прочитать строку");

        if bytes == 0 {
            println!("\nПока!");
            break;
        }

        let Some(guess) = parse_guess(&input) else {
            println!("Это не число, попробуй ещё раз.");
            continue;
        };

        if !(1..=100).contains(&guess) {
            println!("Число должно быть от 1 до 100.");
            continue;
        }

        attempts += 1;

        match compare_guess(guess, secret) {
            Ordering::Less => println!("Больше!"),
            Ordering::Greater => println!("Меньше!"),
            Ordering::Equal => {
                println!("Угадал! Попыток: {attempts}");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rng_is_deterministic_for_seed() {
        let mut a = Rng::from_seed(42);
        let mut b = Rng::from_seed(42);

        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn rng_range_stays_within_bounds() {
        let mut rng = Rng::from_seed(7);

        for _ in 0..10_000 {
            let value = rng.range(1, 100);
            assert!((1..=100).contains(&value), "value = {value}");
        }
    }

    #[test]
    fn parses_valid_guess() {
        assert_eq!(parse_guess("42"), Some(42));
        assert_eq!(parse_guess("  42\n"), Some(42));
    }

    #[test]
    fn rejects_invalid_guess() {
        assert_eq!(parse_guess(""), None);
        assert_eq!(parse_guess("abc"), None);
        assert_eq!(parse_guess("-1"), None);
        assert_eq!(parse_guess("3.14"), None);
    }

    #[test]
    fn compares_guess_with_secret() {
        assert_eq!(compare_guess(50, 42), Ordering::Greater);
        assert_eq!(compare_guess(42, 42), Ordering::Equal);
        assert_eq!(compare_guess(10, 42), Ordering::Less);
    }
}
