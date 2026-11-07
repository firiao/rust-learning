//! Ошибки в практике: `thiserror` для библиотеки, `anyhow` для приложения.

use anyhow::{Context, Result};
use thiserror::Error;

/// Ошибка библиотеки: вызывающий должен уметь её различать.
#[derive(Debug, Error, PartialEq)]
pub enum LoadError {
    #[error("пустой ввод")]
    Empty,

    #[error("не число: {0}")]
    NotANumber(String),

    #[error("{0} слишком велико")]
    TooBig(i64),
}

/// Разбирает положительное число, меньшее 1000.
pub fn parse_small(text: &str) -> Result<u32, LoadError> {
    todo!()
}

/// Прикладная функция: добавляет контекст и работает с `anyhow`.
pub fn load_and_double(text: &str) -> Result<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid() {
        assert_eq!(parse_small("42"), Ok(42));
        assert_eq!(parse_small("  7 "), Ok(7));
    }

    #[test]
    fn reports_errors() {
        assert_eq!(parse_small(""), Err(LoadError::Empty));
        assert_eq!(
            parse_small("abc"),
            Err(LoadError::NotANumber("abc".to_string()))
        );
        assert_eq!(parse_small("5000"), Err(LoadError::TooBig(5000)));
    }

    #[test]
    fn error_messages_are_human_readable() {
        assert_eq!(LoadError::Empty.to_string(), "пустой ввод");
        assert_eq!(LoadError::NotANumber("abc".into()).to_string(), "не число: abc");
        assert_eq!(LoadError::TooBig(5000).to_string(), "5000 слишком велико");
    }

    #[test]
    fn doubles_on_success() {
        assert_eq!(load_and_double("21").unwrap(), 42);
    }

    #[test]
    fn adds_context_on_error() {
        let err = load_and_double("abc").unwrap_err();
        let text = format!("{err:#}");
        assert!(text.contains("не удалось разобрать"), "было: {text}");
        assert!(text.contains("не число: abc"), "было: {text}");
    }
}
