//! Текст и числа. Только то, что было в лекциях 1–2.

/// Количество символов (не байт!).
pub fn char_count(text: &str) -> usize {
    todo!()
}

/// Верхний регистр + "!" на конце.
pub fn shout(text: &str) -> String {
    todo!()
}

/// Длиннее ли текст, чем `limit` символов.
pub fn is_longer_than(text: &str, limit: usize) -> bool {
    todo!()
}

/// Сколько процентов составляет `part` от `total`.
pub fn percent(part: f64, total: f64) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_chars_not_bytes() {
        assert_eq!(char_count("привет"), 6);
        assert_eq!(char_count("hello"), 5);
        assert_eq!(char_count(""), 0);
    }

    #[test]
    fn len_counts_bytes() {
        // это не проверка функции, а напоминание: len() считает байты
        assert_eq!("привет".len(), 12);
        assert_eq!(char_count("привет"), 6);
    }

    #[test]
    fn shouts() {
        assert_eq!(shout("привет"), "ПРИВЕТ!");
        assert_eq!(shout("Привет, мир"), "ПРИВЕТ, МИР!");
        assert_eq!(shout(""), "!");
    }

    #[test]
    fn compares_length() {
        assert!(is_longer_than("привет", 5));
        assert!(!is_longer_than("привет", 6));
        assert!(!is_longer_than("", 0));
        assert!(is_longer_than("ёж", 1));
    }

    #[test]
    fn computes_percent() {
        assert_eq!(percent(50.0, 200.0), 25.0);
        assert_eq!(percent(1.0, 4.0), 25.0);
        assert_eq!(percent(0.0, 10.0), 0.0);
    }
}
