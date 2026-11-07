//! Геометрия.

/// Площадь прямоугольника.
///
/// # Примеры
///
/// ```
/// use l16_modules::geometry::area;
/// assert_eq!(area(2.0, 3.0), 6.0);
/// ```
pub fn area(width: f64, height: f64) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_of_square() {
        assert_eq!(area(2.0, 2.0), 4.0);
    }

    #[test]
    fn area_of_rectangle() {
        assert_eq!(area(3.0, 4.0), 12.0);
    }
}
