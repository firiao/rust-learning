//! Замыкания: захват, `move`, `Fn`/`FnMut`, замыкание как параметр и результат.

/// Применить функцию дважды: f(f(x)).
pub fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    todo!()
}

/// Вернуть замыкание, которое прибавляет `n`.
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    // `todo!()` здесь не подойдёт: возвращаемый тип должен быть замыканием.
    move |_x| todo!("верни x + n")
}

/// Сколько элементов удовлетворяют предикату.
pub fn count_if<F: Fn(&i32) -> bool>(values: &[i32], predicate: F) -> usize {
    todo!()
}

/// Вернуть счётчик: каждое вызове увеличивает и возвращает новое значение.
///
/// Первый вызов возвращает 1.
pub fn make_counter() -> impl FnMut() -> i32 {
    move || todo!("верни счётчик")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_twice() {
        assert_eq!(apply_twice(|n| n + 1, 5), 7);
        assert_eq!(apply_twice(|n| n * n, 3), 81);
    }

    #[test]
    fn adder_captures() {
        let add5 = make_adder(5);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(-3), 2);

        let add10 = make_adder(10);
        assert_eq!(add10(0), 10);
    }

    #[test]
    fn counts_with_predicate() {
        let values = [1, 2, 3, 4, 5];
        assert_eq!(count_if(&values, |n| n.rem_euclid(2) == 0), 2);
        assert_eq!(count_if(&values, |n| *n > 2), 3);
        assert_eq!(count_if(&[], |_| true), 0);
    }

    #[test]
    fn counter_keeps_state() {
        let mut next = make_counter();
        assert_eq!(next(), 1);
        assert_eq!(next(), 2);
        assert_eq!(next(), 3);
    }

    #[test]
    fn counter_is_independent() {
        let mut a = make_counter();
        let mut b = make_counter();
        a();
        a();
        assert_eq!(a(), 3);
        assert_eq!(b(), 1);
    }
}
