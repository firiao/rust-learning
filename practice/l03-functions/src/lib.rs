//! Функции и возврат значений. Только то, что было в лекциях 1–3.

/// Сумма двух чисел.
pub fn add(a: i32, b: i32) -> i32 {
    todo!()
}

/// Квадрат числа.
pub fn square(x: i32) -> i32 {
    todo!()
}

/// Объём прямоугольного параллелепипеда.
pub fn volume(width: f64, height: f64, depth: f64) -> f64 {
    todo!()
}

/// Среднее трёх чисел.
pub fn average3(a: f64, b: f64, c: f64) -> f64 {
    todo!()
}

/// Печатает строку и возвращает её длину в символах.
///
/// Обрати внимание: функция и печатает, и возвращает.
pub fn print_and_count(text: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-2, 2), 0);
    }

    #[test]
    fn squares() {
        assert_eq!(square(4), 16);
        assert_eq!(square(0), 0);
        assert_eq!(square(-3), 9);
    }

    #[test]
    fn computes_volume() {
        assert_eq!(volume(2.0, 3.0, 4.0), 24.0);
        assert_eq!(volume(1.0, 1.0, 1.0), 1.0);
    }

    #[test]
    fn averages() {
        assert_eq!(average3(1.0, 2.0, 3.0), 2.0);
        assert_eq!(average3(10.0, 10.0, 10.0), 10.0);
    }

    #[test]
    fn prints_and_returns() {
        assert_eq!(print_and_count("привет"), 6);
        assert_eq!(print_and_count(""), 0);
    }
}
