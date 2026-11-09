#![allow(clippy::manual_is_multiple_of)]
// `is_multiple_of` появится в лекции 13; пока используем `%`, а линт отключаем.
//! Условия и циклы.

/// Сумма чисел от 1 до `limit` включительно, через `while`.
pub fn sum_while(limit: u32) -> u32 {
    let mut result: u32 = 0;
    for i in 1..=limit {
        result += i;
    }
    result
}

/// Сколько цифр в числе (0 — одна цифра).
pub fn count_digits(n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut n = n;
    let mut count = 0;
    while n > 0 {
        count += 1;
        n /= 10;
    }
    count
}

/// Наибольшая цифра числа. Для 0 — 0.
pub fn max_digit(mut n: u64) -> u8 {
    let mut digit_max: u8 = 0;
    if n < 10 {
        let n: u8 = n.try_into().unwrap();
        return n;
    }

    while n > 0 {
        let mut digit_n: u8 = { n % 10 }.try_into().unwrap();
        if digit_n > digit_max {
            digit_max = digit_n;
        }
        digit_n = { n % 10 }.try_into().unwrap();
        n = n / 10;
    }
    digit_max
}

/// Простое ли число. Через `for` по диапазону.
pub fn is_prime(n: u32) -> bool {
    if n == 0 {
        return false;
    }
    if n == 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// Печатает таблицу умножения для `n` от 1 до 5 и ничего не возвращает.
pub fn print_table(n: u32) {
    for i in 1..=5 {
        println!("{} * {} = {}", n, i, n * i);
    }
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
