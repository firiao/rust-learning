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
    /// Делить на ноль нельзя.
    DivisionByZero,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for ParseError {}

/// Разбирает целое число (с пробелами по краям).
///
/// Пусто -> `Empty`, не число -> `NotANumber`.
pub fn parse_number(text: &str) -> Result<i64, ParseError> {
    todo!()
}

/// Возраст: целое от 0 до 150 включительно.
///
/// Должен использовать `parse_number` и `?`, а не повторять разбор.
pub fn parse_age(text: &str) -> Result<u8, ParseError> {
    todo!()
}

/// Деление. На ноль -> `DivisionByZero`.
pub fn divide(a: i64, b: i64) -> Result<f64, ParseError> {
    todo!()
}

/// Средний возраст двух людей.
///
/// Оба аргумента разбираются как возраст; если хоть один плохой — вернуть его ошибку.
/// Внутри — два `?` и никакого `match`.
pub fn average_age(first: &str, second: &str) -> Result<f64, ParseError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_numbers() {
        assert_eq!(parse_number("42"), Ok(42));
        assert_eq!(parse_number("  -7 "), Ok(-7));
        assert_eq!(parse_number(""), Err(ParseError::Empty));
        assert_eq!(
            parse_number("сорок два"),
            Err(ParseError::NotANumber("сорок два".to_string()))
        );
    }

    #[test]
    fn parses_ages() {
        assert_eq!(parse_age("42"), Ok(42));
        assert_eq!(parse_age("150"), Ok(150));
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
    fn parse_age_reuses_parse_number() {
        // если parse_number возвращает Empty, parse_age обязан вернуть ту же ошибку
        assert_eq!(parse_age("   "), Err(ParseError::Empty));
        assert_eq!(
            parse_age("abc"),
            Err(ParseError::NotANumber("abc".to_string()))
        );
    }

    #[test]
    fn divides() {
        assert_eq!(divide(10, 2), Ok(5.0));
        assert_eq!(divide(1, 4), Ok(0.25));
        assert_eq!(divide(5, 0), Err(ParseError::DivisionByZero));
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
        assert_eq!(
            ParseError::DivisionByZero.to_string(),
            "деление на ноль"
        );
    }

    #[test]
    fn propagates_with_question_mark() {
        assert_eq!(average_age("20", "30").unwrap(), 25.0);
        assert_eq!(average_age("", "30"), Err(ParseError::Empty));
        assert_eq!(
            average_age("20", "abc"),
            Err(ParseError::NotANumber("abc".to_string()))
        );
        assert_eq!(
            average_age("20", "200"),
            Err(ParseError::OutOfRange { value: 200, min: 0, max: 150 })
        );
    }
}
