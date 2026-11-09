//! Спека лабы 10. Эти тесты — твоё ТЗ: читай их и пиши код в src/lib.rs.

use l10_errors::*;

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
