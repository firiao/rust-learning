//! Свои ошибки, `Display`, оператор `?`.

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// Строка пуста или состоит из пробелов.
    Empty,
    /// Не удалось разобрать число; хранит исходный кусок.
    NotANumber(String),
    /// Число вне допустимого диапазона.
    OutOfRange { value: i64, min: i64, max: i64 },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for ParseError {}

/// Возраст: целое от 0 до 150 включительно.
pub fn parse_age(text: &str) -> Result<u8, ParseError> {
    todo!()
}

/// Сумма чисел, разделённых запятыми. Пустые куски игнорируются.
///
/// `"1, 2, 3"` -> Ok(6); `""` и `", ,"` -> Err(Empty).
pub fn sum_csv(text: &str) -> Result<i64, ParseError> {
    todo!()
}

/// Среднее по тем же числам. Используй `?` и `sum_csv`.
pub fn average_csv(text: &str) -> Result<f64, ParseError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_age() {
        assert_eq!(parse_age("42"), Ok(42));
        assert_eq!(parse_age("  7 "), Ok(7));
        assert_eq!(parse_age("150"), Ok(150));
    }

    #[test]
    fn rejects_empty() {
        assert_eq!(parse_age(""), Err(ParseError::Empty));
        assert_eq!(parse_age("   "), Err(ParseError::Empty));
    }

    #[test]
    fn rejects_garbage() {
        assert_eq!(
            parse_age("сорок два"),
            Err(ParseError::NotANumber("сорок два".to_string()))
        );
    }

    #[test]
    fn rejects_out_of_range() {
        assert_eq!(
            parse_age("151"),
            Err(ParseError::OutOfRange { value: 151, min: 0, max: 150 })
        );
        assert_eq!(
            parse_age("-1"),
            Err(ParseError::OutOfRange { value: -1, min: 0, max: 150 })
        );
    }

    #[test]
    fn displays_russian_messages() {
        assert_eq!(ParseError::Empty.to_string(), "пустая строка");
        assert_eq!(
            ParseError::NotANumber("abc".into()).to_string(),
            "не число: \"abc\""
        );
        assert_eq!(
            ParseError::OutOfRange { value: 200, min: 0, max: 150 }.to_string(),
            "200 вне диапазона 0..=150"
        );
    }

    #[test]
    fn sums_csv() {
        assert_eq!(sum_csv("1, 2, 3"), Ok(6));
        assert_eq!(sum_csv(""), Err(ParseError::Empty));
        assert_eq!(sum_csv(", ,"), Err(ParseError::Empty));
    }

    #[test]
    fn propagates_with_question_mark() {
        assert_eq!(
            sum_csv("1, x, 3"),
            Err(ParseError::NotANumber("x".to_string()))
        );
        assert!(matches!(average_csv("1, x"), Err(ParseError::NotANumber(_))));
    }

    #[test]
    fn averages() {
        assert_eq!(average_csv("2, 4"), Ok(3.0));
        assert_eq!(average_csv("10"), Ok(10.0));
        assert_eq!(average_csv(""), Err(ParseError::Empty));
    }
}
