// Интеграционный тест: смотрит на крейт снаружи, как пользователь.
use l16_modules::{area, normalize};

#[test]
fn full_pipeline() {
    let raw = ["  Привет  ", " МИР ", "  "];
    let cleaned: Vec<String> = raw
        .iter()
        .map(|s| normalize(s))
        .filter(|s| !s.is_empty())
        .collect();
    assert_eq!(cleaned, vec!["привет".to_string(), "мир".to_string()]);
    assert_eq!(area(2.0, 3.0), 6.0);
}
