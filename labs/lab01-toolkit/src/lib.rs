//! Lab 01: функции, типы и тесты.
//!
//! Реализуй `is_prime`, `is_palindrome` и `average`.
//! Условие: `assignments/lab01-toolkit.md`.

/// Возвращает `true`, если число чётное.
///
/// Работает и для отрицательных чисел, и для нуля.
pub fn is_even(number: i64) -> bool {
    number % 2 == 0
}

/// Возвращает `true`, если число простое.
///
/// Простое число — целое больше 1, которое делится только на 1 и на само себя.
pub fn is_prime(number: i64) -> bool {
    let _ = number;
    todo!("реализуй is_prime и сними #[ignore] с тестов")
}

/// Возвращает `true`, если строка читается одинаково в обе стороны.
///
/// Регистр и пробелы значимы. Пустая строка — палиндром.
pub fn is_palindrome(text: &str) -> bool {
    let _ = text;
    todo!("реализуй is_palindrome и сними #[ignore] с тестов")
}

/// Возвращает среднее арифметическое, или `None` для пустого среза.
///
/// Входной срез не изменяется.
pub fn average(values: &[f64]) -> Option<f64> {
    let _ = values;
    todo!("реализуй average и сними #[ignore] с тестов")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_positive_is_even() {
        assert!(is_even(8));
    }

    #[test]
    fn even_negative_is_even() {
        assert!(is_even(-8));
    }

    #[test]
    fn even_zero_is_even() {
        assert!(is_even(0));
    }

    #[test]
    fn odd_is_not_even() {
        assert!(!is_even(7));
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь is_prime"]
    fn prime_two_is_prime() {
        assert!(is_prime(2));
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь is_prime"]
    fn prime_one_is_not_prime() {
        assert!(!is_prime(1));
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь is_prime"]
    fn prime_square_of_prime_is_not_prime() {
        assert!(!is_prime(49));
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь is_prime"]
    fn prime_ninety_seven_is_prime() {
        assert!(is_prime(97));
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь is_palindrome"]
    fn palindrome_russian_word() {
        assert!(is_palindrome("шалаш"));
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь is_palindrome"]
    fn palindrome_is_case_sensitive() {
        assert!(!is_palindrome("Шалаш"));
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь is_palindrome"]
    fn empty_string_is_palindrome() {
        assert!(is_palindrome(""));
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь is_palindrome"]
    fn spaces_are_significant() {
        assert!(!is_palindrome("а роза упала на лапу азора"));
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь average"]
    fn average_of_empty_slice_is_none() {
        assert_eq!(average(&[]), None);
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь average"]
    fn average_of_two_values() {
        assert_eq!(average(&[2.0, 4.0]), Some(3.0));
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь average"]
    fn average_of_one_value() {
        assert_eq!(average(&[1.5]), Some(1.5));
    }
}
