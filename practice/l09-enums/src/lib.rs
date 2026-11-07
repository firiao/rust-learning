#![allow(clippy::manual_is_multiple_of)]
// `is_multiple_of` появится в лекции 13; пока используем `%`.
//! Перечисления и `Option`.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Empty,
}

impl Shape {
    /// Площадь. У Empty — 0.0.
    pub fn area(&self) -> f64 {
        todo!()
    }

    /// Есть ли у фигуры ненулевая площадь.
    pub fn has_area(&self) -> bool {
        todo!()
    }
}

/// Деление без паники: на ноль — None.
pub fn safe_divide(a: f64, b: f64) -> Option<f64> {
    todo!()
}

/// Индекс первого чётного числа.
pub fn first_even_index(values: &[i32]) -> Option<usize> {
    todo!()
}

/// Самое длинное слово, если текст не пуст.
pub fn longest_word(text: &str) -> Option<&str> {
    todo!()
}

/// Человеческое описание Option.
///
/// None -> "пусто"; Some(0) -> "есть ноль";
/// Some(n) если n < 0 -> "есть отрицательное {n}"; иначе -> "есть число {n}".
pub fn describe(value: Option<i32>) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn areas() {
        assert_eq!(Shape::Rectangle { width: 3.0, height: 4.0 }.area(), 12.0);
        assert_eq!(Shape::Empty.area(), 0.0);
        let circle = Shape::Circle { radius: 1.0 }.area();
        assert!((circle - std::f64::consts::PI).abs() < 1e-9);
    }

    #[test]
    fn has_area_never_panics() {
        assert!(Shape::Circle { radius: 1.0 }.has_area());
        assert!(!Shape::Empty.has_area());
        assert!(!Shape::Rectangle { width: 0.0, height: 5.0 }.has_area());
    }

    #[test]
    fn divides_safely() {
        assert_eq!(safe_divide(10.0, 2.0), Some(5.0));
        assert_eq!(safe_divide(1.0, 0.0), None);
    }

    #[test]
    fn finds_even() {
        assert_eq!(first_even_index(&[1, 3, 4, 6]), Some(2));
        assert_eq!(first_even_index(&[1, 3, 5]), None);
        assert_eq!(first_even_index(&[]), None);
    }

    #[test]
    fn finds_longest_word() {
        assert_eq!(longest_word("я учу rust"), Some("rust"));
        assert_eq!(longest_word(""), None);
        assert_eq!(longest_word("   "), None);
    }

    #[test]
    fn describes_option() {
        assert_eq!(describe(Some(7)), "есть число 7");
        assert_eq!(describe(Some(0)), "есть ноль");
        assert_eq!(describe(Some(-3)), "есть отрицательное -3");
        assert_eq!(describe(None), "пусто");
    }
}
