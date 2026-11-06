//! Функции и возврат значений.

/// Классический FizzBuzz для одного числа.
///
/// Кратно 15 -> "FizzBuzz", кратно 3 -> "Fizz", кратно 5 -> "Buzz",
/// иначе — само число строкой.
pub fn fizzbuzz(n: u32) -> String {
    todo!()
}

/// Сколько шагов гипотезы Коллатца нужно, чтобы дойти до 1.
///
/// Чётное: n / 2, нечётное: 3n + 1. Для n = 1 — ноль шагов.
pub fn collatz_steps(start: u64) -> u32 {
    todo!()
}

/// Число Фибоначчи, итеративно. fib(0) = 0, fib(1) = 1.
pub fn fib(n: u32) -> u64 {
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
    fn fizzbuzz_cases() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(98), "98");
    }

    #[test]
    fn collatz_known_values() {
        assert_eq!(collatz_steps(1), 0);
        assert_eq!(collatz_steps(2), 1);
        assert_eq!(collatz_steps(6), 8);
        assert_eq!(collatz_steps(27), 111);
    }

    #[test]
    fn fibonacci() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(10), 55);
        assert_eq!(fib(30), 832_040);
        assert_eq!(fib(50), 12_586_269_025);
    }

    #[test]
    fn prints_and_returns() {
        assert_eq!(print_and_count("привет"), 6);
        assert_eq!(print_and_count(""), 0);
    }
}
