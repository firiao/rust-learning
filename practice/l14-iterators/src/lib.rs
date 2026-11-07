//! Итераторы: адаптеры и потребители.

/// Среднее арифметическое. Пустой срез -> None.
pub fn mean(values: &[f64]) -> Option<f64> {
    todo!()
}

/// Среднеквадратичная ошибка. Пусто или разная длина -> None.
pub fn mse(y_true: &[f64], y_pred: &[f64]) -> Option<f64> {
    todo!()
}

/// Квадраты чётных чисел.
pub fn evens_squared(values: &[i32]) -> Vec<i32> {
    todo!()
}

/// Первое число больше `limit`.
pub fn first_big(values: &[i32], limit: i32) -> Option<i32> {
    todo!()
}

/// Произведение всех элементов. Пустой срез -> 1 (нейтральный элемент).
pub fn product(values: &[i64]) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_mean() {
        assert_eq!(mean(&[2.0, 4.0]), Some(3.0));
        assert_eq!(mean(&[]), None);
    }

    #[test]
    fn computes_mse() {
        let t = [3.0, -0.5, 2.0, 7.0];
        let p = [2.5, 0.0, 2.0, 8.0];
        assert_eq!(mse(&t, &p), Some(0.375));
        assert_eq!(mse(&[], &[]), None);
        assert_eq!(mse(&[1.0], &[1.0, 2.0]), None);
    }

    #[test]
    fn squares_evens() {
        assert_eq!(evens_squared(&[1, 2, 3, 4, 5, 6]), vec![4, 16, 36]);
        assert!(evens_squared(&[1, 3, 5]).is_empty());
    }

    #[test]
    fn finds_first_big() {
        assert_eq!(first_big(&[1, 2, 10, 20], 5), Some(10));
        assert_eq!(first_big(&[1, 2], 5), None);
    }

    #[test]
    fn multiplies() {
        assert_eq!(product(&[2, 3, 4]), 24);
        assert_eq!(product(&[]), 1);
    }
}
