//! JSON через `serde`: структура <-> текст.

use serde::{Deserialize, Serialize};

/// Параметры обучения.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// Обязательное поле.
    pub hidden_dim: usize,
    /// Если в JSON поля нет — берётся значение по умолчанию.
    #[serde(default = "default_epochs")]
    pub epochs: usize,
    #[serde(default)]
    pub verbose: bool,
}

pub fn default_epochs() -> usize {
    2000
}

/// Сериализовать в строку JSON.
pub fn to_json(config: &Config) -> String {
    todo!()
}

/// Разобрать из строки JSON.
pub fn from_json(text: &str) -> Result<Config, serde_json::Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Config {
        Config { hidden_dim: 16, epochs: 500, verbose: true }
    }

    #[test]
    fn roundtrip() {
        let json = to_json(&sample());
        let back = from_json(&json).unwrap();
        assert_eq!(back, sample());
    }

    #[test]
    fn json_contains_fields() {
        let json = to_json(&sample());
        assert!(json.contains("\"hidden_dim\":16"));
        assert!(json.contains("\"epochs\":500"));
    }

    #[test]
    fn defaults_are_applied() {
        let cfg = from_json(r#"{ "hidden_dim": 8 }"#).unwrap();
        assert_eq!(cfg.epochs, 2000);
        assert!(!cfg.verbose);
    }

    #[test]
    fn missing_required_field_is_error() {
        assert!(from_json(r#"{ "epochs": 10 }"#).is_err());
    }

    #[test]
    fn wrong_type_is_error() {
        assert!(from_json(r#"{ "hidden_dim": "много" }"#).is_err());
    }
}
