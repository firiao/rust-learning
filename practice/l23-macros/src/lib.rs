//! Свои макросы `macro_rules!`.
//!
//! Сейчас каждый макрос разворачивается в вызов `not_implemented()`:
//! он компилируется (тип выводится из места вызова), но паникует.
//! Замени тела так, чтобы тесты позеленели.

/// Временная заглушка: тип `T` выводится из контекста.
#[allow(dead_code)]
pub fn not_implemented<T>() -> T {
    todo!("макрос ещё не реализован")
}

/// Сумма любого числа выражений: `sum_all!(1, 2, 3)` -> 6.
#[macro_export]
macro_rules! sum_all {
    ($($x:expr),*) => {
        $crate::not_implemented::<i32>()
    };
}

/// Максимум из двух и более выражений: `max_of!(3, 7, 2)` -> 7.
#[macro_export]
macro_rules! max_of {
    ($first:expr, $($rest:expr),+) => {
        $crate::not_implemented::<i32>()
    };
}

/// Вектор строк из литералов: `strings!("а", "б")` -> vec!["а", "б"].
#[macro_export]
macro_rules! strings {
    ($($x:expr),*) => {
        $crate::not_implemented::<Vec<String>>()
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn sums() {
        assert_eq!(sum_all!(1, 2, 3), 6);
        assert_eq!(sum_all!(5), 5);
    }

    #[test]
    fn finds_max() {
        assert_eq!(max_of!(3, 7, 2), 7);
        assert_eq!(max_of!(10, 1), 10);
    }

    #[test]
    fn makes_strings() {
        assert_eq!(strings!("а", "б"), vec!["а".to_string(), "б".to_string()]);
    }
}
