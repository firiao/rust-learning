//! Сопоставление со случаями через `match`.

/// Оценка по баллам: 90..=100 -> 'A', 75..=89 -> 'B', 60..=74 -> 'C', иначе 'F'.
pub fn grade(score: u8) -> char {
    todo!()
}

/// Где находится точка: строками человеческим языком.
///
/// (0,0) -> "начало координат"; (0,_) -> "на оси Y"; (_,0) -> "на оси X";
/// (a,b) если a == b -> "на диагонали"; иначе -> "обычная точка".
pub fn describe_point(x: i32, y: i32) -> String {
    todo!()
}

/// Простой калькулятор: "+", "-", "*", "/". Неизвестная операция -> 0.
///
/// Деление на ноль не обрабатываем — это будет лаба про `Result`.
pub fn apply(op: &str, a: i32, b: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grades_by_range() {
        assert_eq!(grade(100), 'A');
        assert_eq!(grade(90), 'A');
        assert_eq!(grade(89), 'B');
        assert_eq!(grade(60), 'C');
        assert_eq!(grade(59), 'F');
        assert_eq!(grade(0), 'F');
    }

    #[test]
    fn describes_points() {
        assert_eq!(describe_point(0, 0), "начало координат");
        assert_eq!(describe_point(0, 5), "на оси Y");
        assert_eq!(describe_point(3, 0), "на оси X");
        assert_eq!(describe_point(3, 3), "на диагонали");
        assert_eq!(describe_point(2, 5), "обычная точка");
    }

    #[test]
    fn computes() {
        assert_eq!(apply("+", 2, 3), 5);
        assert_eq!(apply("-", 2, 3), -1);
        assert_eq!(apply("*", 2, 3), 6);
        assert_eq!(apply("/", 6, 3), 2);
        assert_eq!(apply("^", 2, 3), 0);
    }
}
