//! Перцептрон с нуля, без внешних зависимостей.
//!
//! Это «ML-нулевой» проект курса: минимум Rust и минимум ML,
//! но уже настоящая модель, обучение и accuracy.
//!
//! Запуск: `cargo run -p perceptron`.

/// Простейший перцептрон с двумя входами.
#[derive(Debug, Clone)]
struct Perceptron {
    weights: [f64; 2],
    bias: f64,
    learning_rate: f64,
}

impl Perceptron {
    fn new(learning_rate: f64) -> Self {
        Self {
            weights: [0.0; 2],
            bias: 0.0,
            learning_rate,
        }
    }

    /// Линейная комбинация `w·x + b` до пороговой функции.
    fn raw(&self, input: [f64; 2]) -> f64 {
        self.weights[0] * input[0] + self.weights[1] * input[1] + self.bias
    }

    /// Пороговая функция: 1, если `raw >= 0`, иначе 0.
    fn predict(&self, input: [f64; 2]) -> u8 {
        if self.raw(input) >= 0.0 { 1 } else { 0 }
    }

    /// Обучение по правилу перцептрона.
    ///
    /// Для каждого примера: `error = target - prediction`.
    /// Если ошибка не нулевая, обновляем веса и bias.
    fn train(&mut self, data: &[([f64; 2], u8)], epochs: usize) {
        for _ in 0..epochs {
            let mut errors = 0;

            for &(input, target) in data {
                let prediction = self.predict(input);
                let error = target as f64 - prediction as f64;

                if error != 0.0 {
                    self.weights[0] += self.learning_rate * error * input[0];
                    self.weights[1] += self.learning_rate * error * input[1];
                    self.bias += self.learning_rate * error;
                    errors += 1;
                }
            }

            // Если за эпоху не было ошибок — дальше учить нечего.
            if errors == 0 {
                break;
            }
        }
    }

    /// Доля правильных ответов; 0.0 для пустого набора.
    fn accuracy(&self, data: &[([f64; 2], u8)]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let correct = data
            .iter()
            .filter(|(input, target)| self.predict(*input) == *target)
            .count();

        correct as f64 / data.len() as f64
    }
}

fn or_data() -> Vec<([f64; 2], u8)> {
    vec![
        ([0.0, 0.0], 0),
        ([0.0, 1.0], 1),
        ([1.0, 0.0], 1),
        ([1.0, 1.0], 1),
    ]
}

fn and_data() -> Vec<([f64; 2], u8)> {
    vec![
        ([0.0, 0.0], 0),
        ([0.0, 1.0], 0),
        ([1.0, 0.0], 0),
        ([1.0, 1.0], 1),
    ]
}

fn main() {
    println!("=== Перцептрон: OR ===");
    let data = or_data();
    let mut model = Perceptron::new(0.1);

    println!("До обучения: accuracy = {:.2}", model.accuracy(&data));
    model.train(&data, 100);

    println!("После обучения: accuracy = {:.2}", model.accuracy(&data));
    println!("weights = {:?}, bias = {:.4}", model.weights, model.bias);
    println!();

    for (input, target) in &data {
        let prediction = model.predict(*input);
        println!(
            "  {:?} -> предсказание {}, цель {}",
            input, prediction, target
        );
    }

    println!();
    println!("=== Перцептрон: AND ===");
    let data = and_data();
    let mut model = Perceptron::new(0.1);
    model.train(&data, 100);
    println!("accuracy = {:.2}", model.accuracy(&data));

    println!();
    println!("Попробуй XOR: один перцептрон его не решает.");
    println!("Это задача для MLP из lab09.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_model_has_zero_parameters() {
        let model = Perceptron::new(0.1);
        assert_eq!(model.weights, [0.0, 0.0]);
        assert_eq!(model.bias, 0.0);
    }

    #[test]
    fn predicts_with_manual_weights() {
        let model = Perceptron {
            weights: [1.0, 1.0],
            bias: -0.5,
            learning_rate: 0.1,
        };

        assert_eq!(model.predict([0.0, 0.0]), 0);
        assert_eq!(model.predict([0.0, 1.0]), 1);
        assert_eq!(model.predict([1.0, 0.0]), 1);
        assert_eq!(model.predict([1.0, 1.0]), 1);
    }

    #[test]
    fn learns_or() {
        let data = or_data();
        let mut model = Perceptron::new(0.1);
        model.train(&data, 100);
        assert_eq!(model.accuracy(&data), 1.0);
    }

    #[test]
    fn learns_and() {
        let data = and_data();
        let mut model = Perceptron::new(0.1);
        model.train(&data, 100);
        assert_eq!(model.accuracy(&data), 1.0);
    }

    #[test]
    fn accuracy_of_empty_is_zero() {
        let model = Perceptron::new(0.1);
        assert_eq!(model.accuracy(&[]), 0.0);
    }
}
