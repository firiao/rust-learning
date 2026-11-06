//! Lab 03: structs, enums, методы и pattern matching.
//!
//! Реализуй методы, помеченные `todo!()`.
//! Условие: `assignments/lab03-domain-model.md`.

/// Один пример: вектор признаков и целевая метка.
#[derive(Debug, Clone, PartialEq)]
pub struct Sample {
    pub features: Vec<f64>,
    pub label: f64,
}

impl Sample {
    /// Создаёт пример.
    pub fn new(features: Vec<f64>, label: f64) -> Self {
        Self { features, label }
    }
}

/// Набор примеров с именем.
#[derive(Debug, Clone, PartialEq)]
pub struct Dataset {
    pub name: String,
    pub samples: Vec<Sample>,
}

impl Dataset {
    /// Создаёт датасет.
    pub fn new(name: &str, samples: Vec<Sample>) -> Self {
        Self {
            name: name.to_string(),
            samples,
        }
    }

    /// Количество примеров.
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// `true`, если примеров нет.
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Размерность признаков, если она согласована.
    pub fn feature_dim(&self) -> Option<usize> {
        let _ = self;
        todo!("реализуй feature_dim и сними #[ignore] с тестов")
    }

    /// Среднее значение метки.
    pub fn mean_label(&self) -> Option<f64> {
        let _ = self;
        todo!("реализуй mean_label и сними #[ignore] с тестов")
    }

    /// Минимум и максимум метки.
    pub fn label_range(&self) -> Option<(f64, f64)> {
        let _ = self;
        todo!("реализуй label_range и сними #[ignore] с тестов")
    }
}

/// Тип задачи.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Task {
    Regression,
    BinaryClassification,
    MulticlassClassification,
}

impl Task {
    /// Человекочитаемое описание задачи.
    pub fn description(&self) -> &'static str {
        match self {
            Task::Regression => "регрессия",
            Task::BinaryClassification => "бинарная классификация",
            Task::MulticlassClassification => "многоклассовая классификация",
        }
    }
}

/// Функция активации.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Activation {
    Relu,
    Sigmoid,
    Tanh,
}

impl Activation {
    /// Имя для конфигов и вывода.
    pub fn name(&self) -> &'static str {
        match self {
            Activation::Relu => "relu",
            Activation::Sigmoid => "sigmoid",
            Activation::Tanh => "tanh",
        }
    }

    /// Применяет активацию к скаляру.
    pub fn apply(&self, x: f64) -> f64 {
        let _ = (self, x);
        todo!("реализуй apply и сними #[ignore] с тестов")
    }

    /// Производная активации в точке `x`.
    pub fn derivative(&self, x: f64) -> f64 {
        let _ = (self, x);
        todo!("реализуй derivative и сними #[ignore] с тестов")
    }
}

/// Конфигурация модели.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelConfig {
    pub hidden_layers: Vec<usize>,
    pub activation: Activation,
    pub learning_rate: f64,
}

impl ModelConfig {
    /// Создаёт конфигурацию.
    pub fn new(hidden_layers: Vec<usize>, activation: Activation, learning_rate: f64) -> Self {
        Self {
            hidden_layers,
            activation,
            learning_rate,
        }
    }

    /// Проверяет, что конфигурация имеет смысл.
    pub fn is_valid(&self) -> bool {
        let _ = self;
        todo!("реализуй is_valid и сними #[ignore] с тестов")
    }

    /// Краткое описание конфигурации.
    pub fn describe(&self) -> String {
        let _ = self;
        todo!("реализуй describe и сними #[ignore] с тестов")
    }
}

/// Метрика качества.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Metric {
    Mse,
    Mae,
    Accuracy,
}

impl Metric {
    /// Считает метрику; `None`, если данные некорректны.
    pub fn score(&self, predictions: &[f64], targets: &[f64]) -> Option<f64> {
        let _ = (self, predictions, targets);
        todo!("реализуй score и сними #[ignore] с тестов")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(features: Vec<f64>, label: f64) -> Sample {
        Sample::new(features, label)
    }

    #[test]
    fn creates_sample() {
        let s = sample(vec![1.0, 2.0], 1.0);
        assert_eq!(s.features, vec![1.0, 2.0]);
        assert_eq!(s.label, 1.0);
    }

    #[test]
    fn dataset_len_and_empty() {
        let empty = Dataset::new("empty", vec![]);
        assert_eq!(empty.len(), 0);
        assert!(empty.is_empty());

        let dataset = Dataset::new("data", vec![sample(vec![1.0], 0.0)]);
        assert_eq!(dataset.len(), 1);
        assert!(!dataset.is_empty());
    }

    #[test]
    fn task_description() {
        assert_eq!(Task::Regression.description(), "регрессия");
        assert_eq!(
            Task::BinaryClassification.description(),
            "бинарная классификация"
        );
    }

    #[test]
    fn activation_names() {
        assert_eq!(Activation::Relu.name(), "relu");
        assert_eq!(Activation::Sigmoid.name(), "sigmoid");
        assert_eq!(Activation::Tanh.name(), "tanh");
    }

    #[test]
    fn creates_model_config() {
        let config = ModelConfig::new(vec![8, 4], Activation::Relu, 0.001);
        assert_eq!(config.hidden_layers, vec![8, 4]);
        assert_eq!(config.activation, Activation::Relu);
        assert_eq!(config.learning_rate, 0.001);
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь feature_dim"]
    fn feature_dim_of_empty_is_none() {
        let dataset = Dataset::new("empty", vec![]);
        assert_eq!(dataset.feature_dim(), None);
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь feature_dim"]
    fn feature_dim_of_consistent_samples() {
        let dataset = Dataset::new(
            "data",
            vec![sample(vec![1.0, 2.0], 0.0), sample(vec![3.0, 4.0], 1.0)],
        );
        assert_eq!(dataset.feature_dim(), Some(2));
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь feature_dim"]
    fn feature_dim_of_inconsistent_samples_is_none() {
        let dataset = Dataset::new(
            "data",
            vec![sample(vec![1.0, 2.0], 0.0), sample(vec![3.0], 1.0)],
        );
        assert_eq!(dataset.feature_dim(), None);
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь mean_label"]
    fn mean_label_of_empty_is_none() {
        let dataset = Dataset::new("empty", vec![]);
        assert_eq!(dataset.mean_label(), None);
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь mean_label"]
    fn mean_label_of_samples() {
        let dataset = Dataset::new("data", vec![sample(vec![1.0], 1.0), sample(vec![2.0], 3.0)]);
        assert_eq!(dataset.mean_label(), Some(2.0));
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь label_range"]
    fn label_range_of_empty_is_none() {
        let dataset = Dataset::new("empty", vec![]);
        assert_eq!(dataset.label_range(), None);
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь label_range"]
    fn label_range_of_samples() {
        let dataset = Dataset::new(
            "data",
            vec![
                sample(vec![1.0], -1.0),
                sample(vec![2.0], 2.0),
                sample(vec![3.0], 0.0),
            ],
        );
        assert_eq!(dataset.label_range(), Some((-1.0, 2.0)));
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь Activation::apply"]
    fn relu_works() {
        assert_eq!(Activation::Relu.apply(-1.0), 0.0);
        assert_eq!(Activation::Relu.apply(2.0), 2.0);
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь Activation::apply"]
    fn sigmoid_and_tanh_work() {
        assert!((Activation::Sigmoid.apply(0.0) - 0.5).abs() < 1e-12);
        assert!(Activation::Tanh.apply(0.0).abs() < 1e-12);
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь Activation::derivative"]
    fn relu_derivative_works() {
        assert_eq!(Activation::Relu.derivative(-1.0), 0.0);
        assert_eq!(Activation::Relu.derivative(2.0), 1.0);
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь Activation::derivative"]
    fn sigmoid_derivative_at_zero_is_quarter() {
        assert!((Activation::Sigmoid.derivative(0.0) - 0.25).abs() < 1e-12);
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь ModelConfig::is_valid"]
    fn config_validity() {
        assert!(ModelConfig::new(vec![8], Activation::Relu, 0.001).is_valid());
        assert!(ModelConfig::new(vec![], Activation::Relu, 0.001).is_valid());
        assert!(!ModelConfig::new(vec![8], Activation::Relu, 0.0).is_valid());
        assert!(!ModelConfig::new(vec![8], Activation::Relu, -0.1).is_valid());
        assert!(!ModelConfig::new(vec![8], Activation::Relu, f64::NAN).is_valid());
        assert!(!ModelConfig::new(vec![0], Activation::Relu, 0.001).is_valid());
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь ModelConfig::describe"]
    fn config_description_format() {
        let config = ModelConfig::new(vec![8, 4], Activation::Relu, 0.001);
        assert_eq!(
            config.describe(),
            "layers=[8, 4], activation=relu, lr=0.001"
        );
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь Metric::score"]
    fn mse_score() {
        assert_eq!(Metric::Mse.score(&[1.0, 2.0], &[1.0, 2.0]), Some(0.0));
        assert_eq!(Metric::Mse.score(&[1.0, 2.0], &[2.0, 4.0]), Some(2.5));
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь Metric::score"]
    fn mae_score() {
        assert_eq!(Metric::Mae.score(&[1.0, 2.0], &[2.0, 4.0]), Some(1.5));
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь Metric::score"]
    fn accuracy_score() {
        assert_eq!(Metric::Accuracy.score(&[0.1, 0.9], &[0.0, 1.0]), Some(1.0));
        assert_eq!(Metric::Accuracy.score(&[0.6, 0.9], &[0.0, 1.0]), Some(0.5));
    }

    #[test]
    #[ignore = "убери #[ignore], когда реализуешь Metric::score"]
    fn metric_returns_none_for_bad_input() {
        assert_eq!(Metric::Mse.score(&[], &[]), None);
        assert_eq!(Metric::Mse.score(&[1.0], &[]), None);
        assert_eq!(Metric::Mse.score(&[1.0, 2.0], &[1.0]), None);
    }
}
