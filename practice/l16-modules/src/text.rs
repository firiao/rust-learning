//! Работа с текстом.

/// Обрезает пробелы по краям, приводит к нижнему регистру
/// и схлопывает внутренние пробелы в один.
///
/// ```
/// use l16_modules::normalize;
/// assert_eq!(normalize("  Привет   Мир  "), "привет мир");
/// assert_eq!(normalize("   "), "");
/// ```
pub fn normalize(text: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collapses_spaces() {
        assert_eq!(normalize("  а   б  "), "а б");
    }

    #[test]
    fn empty_is_empty() {
        assert_eq!(normalize("   "), "");
    }
}
