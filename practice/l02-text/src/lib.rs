//! Работа с текстом.

/// Количество символов (не байт!).
pub fn char_count(text: &str) -> usize {
    todo!()
}

/// Первые буквы слов, заглавными, через ". " и точкой в конце.
///
/// "иван петров" -> "И. П."
/// ""            -> ""
pub fn initials(full_name: &str) -> String {
    todo!()
}

/// Заменяет все ASCII-цифры на '*', остальное не трогает.
pub fn mask_digits(text: &str) -> String {
    todo!()
}

/// Верхний регистр + "!" на конце.
pub fn shout(text: &str) -> String {
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
        assert_eq!(char_count("ёж"), 2);
    }

    #[test]
    fn len_counts_bytes() {
        // это не проверка функции, а напоминание: len() считает байты
        assert_eq!("привет".len(), 12);
        assert_eq!(char_count("привет"), 6);
    }

    #[test]
    fn makes_initials() {
        assert_eq!(initials("иван петров"), "И. П.");
        assert_eq!(initials("анна"), "А.");
        assert_eq!(initials("  мария   ивановна  петрова "), "М. И. П.");
        assert_eq!(initials(""), "");
    }

    #[test]
    fn masks_digits() {
        assert_eq!(mask_digits("карта 4276 1234"), "карта **** ****");
        assert_eq!(mask_digits("нет цифр"), "нет цифр");
        assert_eq!(mask_digits("987"), "***");
    }

    #[test]
    fn shouts() {
        assert_eq!(shout("привет"), "ПРИВЕТ!");
        assert_eq!(shout("Привет, мир"), "ПРИВЕТ, МИР!");
        assert_eq!(shout(""), "!");
    }
}
