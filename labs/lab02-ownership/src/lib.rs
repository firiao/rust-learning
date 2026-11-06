//! Lab 02: ownership, borrowing и срезы.
//!
//! Реализуй `first_word`, `longest` и `split_even_odd`.
//! Условие: `assignments/lab02-ownership.md`.

/// Возвращает первое слово строки.
///
/// Ведущие пробелы игнорируются. Если слов нет, возвращается `""`.
/// Результат — срез входной строки, без выделения памяти.
pub fn first_word(text: &str) -> &str {
    let _ = text;
    todo!("реализуй first_word и сними #[ignore] с тестов")
}

/// Возвращает более длинную из двух строк.
///
/// При равной длине возвращает левую. Результат — срез одного из входов.
pub fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    let _ = (left, right);
    todo!("реализуй longest и сними #[ignore] с тестов")
}

/// Считает слова, разделённые пробельными символами.
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Переводит строку в верхний регистр на месте.
pub fn shout_in_place(text: &mut String) {
    *text = text.to_uppercase();
}

/// Разделяет числа на чётные и нечётные, сохраняя порядок.
pub fn split_even_odd(values: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let _ = values;
    todo!("реализуй split_even_odd и сними #[ignore] с тестов")
}

/// Сумма элементов; для пустого среза — 0.
pub fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

/// Забирает владение строкой и возвращает её длину в байтах.
pub fn consume_and_len(text: String) -> usize {
    text.len()
}

/// Возвращает приветствие; имя не забирает.
pub fn greet(name: &str) -> String {
    format!("Привет, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_words() {
        assert_eq!(count_words("hello world"), 2);
    }

    #[test]
    fn counts_no_words_in_whitespace() {
        assert_eq!(count_words("   \t\n"), 0);
    }

    #[test]
    fn counts_words_with_multiple_spaces() {
        assert_eq!(count_words("one   two"), 2);
    }

    #[test]
    fn shout_uppercases_ascii() {
        let mut text = String::from("hello");
        shout_in_place(&mut text);
        assert_eq!(text, "HELLO");
    }

    #[test]
    fn shout_uppercases_cyrillic() {
        let mut text = String::from("привет");
        shout_in_place(&mut text);
        assert_eq!(text, "ПРИВЕТ");
    }

    #[test]
    fn sum_of_empty_slice_is_zero() {
        assert_eq!(sum(&[]), 0);
    }

    #[test]
    fn sum_of_values() {
        assert_eq!(sum(&[1, 2, 3]), 6);
        assert_eq!(sum(&[-1, 1]), 0);
    }

    #[test]
    fn consume_returns_byte_length() {
        assert_eq!(consume_and_len(String::from("hello")), 5);
        assert_eq!(consume_and_len(String::from("привет")), 12);
    }

    #[test]
    fn greet_returns_new_string() {
        assert_eq!(greet("Аня"), "Привет, Аня!");
        assert_eq!(greet(""), "Привет, !");
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь first_word"]
    fn first_word_of_sentence() {
        assert_eq!(first_word("hello world"), "hello");
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь first_word"]
    fn first_word_skips_leading_whitespace() {
        assert_eq!(first_word("   leading"), "leading");
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь first_word"]
    fn first_word_of_whitespace_is_empty() {
        assert_eq!(first_word("   \t\n"), "");
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь first_word"]
    fn first_word_of_empty_is_empty() {
        assert_eq!(first_word(""), "");
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь first_word"]
    fn first_word_supports_cyrillic() {
        assert_eq!(first_word("привет мир"), "привет");
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь longest"]
    fn longest_returns_left_when_longer() {
        assert_eq!(longest("long", "hi"), "long");
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь longest"]
    fn longest_returns_right_when_longer() {
        assert_eq!(longest("hi", "long"), "long");
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь longest"]
    fn longest_returns_left_on_tie() {
        assert_eq!(longest("abc", "xyz"), "abc");
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь split_even_odd"]
    fn split_even_odd_of_empty() {
        assert_eq!(split_even_odd(&[]), (vec![], vec![]));
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь split_even_odd"]
    fn split_even_odd_keeps_order() {
        assert_eq!(
            split_even_odd(&[1, 2, 3, 4, -5, -6]),
            (vec![2, 4, -6], vec![1, 3, -5])
        );
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь split_even_odd"]
    fn split_even_odd_all_even() {
        assert_eq!(split_even_odd(&[2, 4, 6]), (vec![2, 4, 6], vec![]));
    }
}
