//! Лаба 8. match.
//!
//! Реализация — твоя задача. Спека лежит в `tests/spec.rs`.
pub fn grade(value: &u8) -> char {
    match {
        90..=100 => 'A',
        75..=89 => 'B',
        60..=74 => 'C',
        _ => 'F',
    }
}

pub fn describe_point(x: &, y: &) -> String {
    match (x, y) {
        (0, 0) => "начало координат",
        (0, _) => "на оси Y",
        (_, 0) => "на оси X",
        (a, b) if a == b => "на диагонали",
        _ => "Обычная точка",
    }
}

pub fn apply(op: &char, a: &i32, b: &i32) -> i32 {
    match &op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => a,
    }
}