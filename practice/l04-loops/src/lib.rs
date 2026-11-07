#![allow(clippy::manual_is_multiple_of)]
// `is_multiple_of` появится в лекции 13; пока используем `%`, а линт отключаем.
//! Условия и циклы.

/// Сумма чисел от 1 до `limit` включительно, через `while`.
pub fn sum_while(limit: u32) -> u32 {
    todo!()
}

/// Сколько цифр в числе (0 — одна цифра).
pub fn count_digits(n: u64) -> u32 {
    todo!()
}

/// Наибольшая цифра числа. Для 0 — 0.
pub fn max_digit(n: u64) -> u8 {
    todo!()
}

/// Простое ли число. Через `for` по диапазону.
pub fn is_prime(n: u32) -> bool {
    todo!()
}

/// Печатает таблицу умножения для `n` от 1 до 5 и ничего не возвращает.
pub fn print_table(n: u32) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_with_while() {
        assert_eq!(sum_while(0), 0);
        assert_eq!(sum_while(1), 1);
        assert_eq!(sum_while(5), 15);
        assert_eq!(sum_while(100), 5050);
    }

    #[test]
    fn counts_digits() {
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(7), 1);
        assert_eq!(count_digits(42), 2);
        assert_eq!(count_digits(999), 3);
        assert_eq!(count_digits(1_000_000), 7);
    }

    #[test]
    fn finds_max_digit() {
        assert_eq!(max_digit(0), 0);
        assert_eq!(max_digit(5731), 7);
        assert_eq!(max_digit(1111), 1);
        assert_eq!(max_digit(908), 9);
    }

    #[test]
    fn detects_primes() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(97));
        assert!(!is_prime(100));
    }

    #[test]
    fn prints_table_without_returning() {
        // печать проверяем косвенно: функция должна просто отработать
        print_table(3);
        let _unit: () = print_table(1);
    }
}
