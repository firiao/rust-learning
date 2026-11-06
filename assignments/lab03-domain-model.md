# Лабораторная 03. Structs, enums, методы и pattern matching

## Цель

Научиться моделировать предметную область через `struct` и `enum`, писать методы в `impl`, использовать `Option<T>` и `match` без «забытых» вариантов.

Предметная область — ML-эксперимент: датасет, признаки, метки, конфиг модели, функция активации, метрика.

## Что нужно знать до начала

- lab01–lab02;
- `String`, `Vec`, срезы;
- ownership и заимствование;
- `Option<T>`;
- `match`/`if let`.

## Языковой бюджет

Разрешено: `struct`, `enum`, `impl`, `match`, `Option`, `#[derive]`, `&self`/`&mut self`/`self`, `format!`, циклы, методы.

Запрещено: трейты (кроме `derive`), generics, замыкания, итераторы сверх `for`, `Result` (это lab05), внешние крейты.

## Файлы

- Условие: этот файл.
- Код: [`../labs/lab03-domain-model/src/lib.rs`](../labs/lab03-domain-model/src/lib.rs).
- Запуск: `cargo test -p lab03-domain-model`.

## Что уже есть

- `Sample` с полями `features: Vec<f64>` и `label: f64`;
- `Dataset` с полями `name: String` и `samples: Vec<Sample>`;
- enum'ы `Task`, `Activation`, `Metric`;
- `ModelConfig` с полями `hidden_layers`, `activation`, `learning_rate`;
- реализованные `Sample::new`, `Dataset::new`, `len`, `is_empty`, `Task::description`, `ModelConfig::new`, `Activation::name`.

Нужно реализовать:

- `Dataset::feature_dim`, `mean_label`, `label_range`;
- `Activation::apply`, `derivative`;
- `ModelConfig::is_valid`, `describe`;
- `Metric::score`.

Часть тестов помечена `#[ignore]`.

---

## Подготовительная часть

1. Запусти `cargo test -p lab03-domain-model`.
2. Открой `src/lib.rs` и выпиши, какие поля есть у каждого типа.
3. Ответь: почему `Task`, `Activation` и `Metric` — enum'ы, а `Sample` и `Dataset` — структуры?
4. Почему `Dataset::feature_dim` возвращает `Option<usize>`, а не `usize`?
5. Добавь тест на `Dataset::len` для пустого датасета и датасета из двух samples.

---

## Обязательное задание

Реализуй методы, не меняя публичные сигнатуры.

### `Dataset`

```rust
pub fn feature_dim(&self) -> Option<usize>;
pub fn mean_label(&self) -> Option<f64>;
pub fn label_range(&self) -> Option<(f64, f64)>;
```

Контракт:

1. `feature_dim` для пустого датасета → `None`.
2. Если у всех samples одинаковая длина `features`, вернуть `Some(длина)`; включая случай `0`.
3. Если длины различаются → `None`.
4. `mean_label` для пустого датасета → `None`; иначе `Some(среднее)`.
5. `label_range` для пустого датасета → `None`; иначе `Some((min, max))`.
6. Методы не изменяют датасет и не паникуют.

### `Activation`

```rust
pub fn apply(&self, x: f64) -> f64;
pub fn derivative(&self, x: f64) -> f64;
```

Контракт:

1. `Relu`: `max(0, x)`; производная `1.0` при `x > 0`, иначе `0.0`.
2. `Sigmoid`: `1 / (1 + exp(-x))`; производная `s * (1 - s)`, где `s = apply(x)`.
3. `Tanh`: `x.tanh()`; производная `1 - t * t`.
4. Для `NaN` поведение не определено; не добавляй специальную обработку.

### `ModelConfig`

```rust
pub fn is_valid(&self) -> bool;
pub fn describe(&self) -> String;
```

Контракт:

1. `is_valid`: `learning_rate` конечен (`is_finite()`) и строго больше нуля.
2. `is_valid`: каждый размер скрытого слоя строго больше нуля.
3. Пустой `hidden_layers` допустим (линейная модель).
4. `describe` возвращает строку вида `"layers=[8, 4], activation=relu, lr=0.001"`.
5. Формат `describe` стабилен: его проверяют тесты.

### `Metric`

```rust
pub fn score(&self, predictions: &[f64], targets: &[f64]) -> Option<f64>;
```

Контракт:

1. Если срезы пустые или их длины различаются → `None`.
2. `Mse`: среднее `(prediction - target)^2`.
3. `Mae`: среднее `|prediction - target|`.
4. `Accuracy`: доля случаев, где `(prediction >= 0.5) == (target >= 0.5)`.
5. Метрика не изменяет срезы и не паникует.

### Тесты

Сними `#[ignore]` и добавь тесты:

- `feature_dim`: пустой датасет, одинаковые размеры, разные размеры, нулевая размерность;
- `mean_label`: пустой, один sample, отрицательные метки;
- `label_range`: пустой, один sample, отрицательные/положительные;
- `Activation`: значения в 0, положительные/отрицательные, производные;
- `ModelConfig`: валидный/невалидный lr, нулевой слой, пустые слои, формат `describe`;
- `Metric`: MSE/MAE/Accuracy на известных значениях, пустые срезы, разная длина.

---

## Пошаговый план

1. Реализуй `feature_dim`: проверь пустоту, возьми длину первого sample, пройди по остальным.
2. Реализуй `mean_label` и `label_range` циклами. Для min/max можно начать с первого элемента и обновлять в цикле.
3. Реализуй `Activation::apply` через `match self`.
4. Реализуй `derivative`. Для sigmoid не вызывай `apply` дважды без нужды; сохрани `s` в переменную.
5. Реализуй `ModelConfig::is_valid` и `describe`. Для `describe` используй `Activation::name`.
6. Реализуй `Metric::score`: сначала общая проверка длины, затем `match self`.
7. Сними `#[ignore]`, добавь тесты, запусти:

   ```bash
   cargo test -p lab03-domain-model
   cargo clippy -p lab03-domain-model -- -D warnings
   cargo fmt
   ```

---

## Дополнительная часть

Выбери одно или несколько:

- `Dataset::filter_by_label(&self, label: f64) -> Dataset` — новый датасет с samples, у которых метка равна заданной;
- `Dataset::feature_means(&self) -> Option<Vec<f64>>` — среднее по каждому признаку; `None`, если датасет пуст или размерности разные;
- `ModelConfig::builder()` — простой builder без внешних крейтов;
- реализуй `std::fmt::Display` для `Dataset` и `ModelConfig` вместо `describe` (или в дополнение);
- `Task::infer(&Dataset) -> Option<Task>` — если все метки 0/1, то бинарная классификация; если метки целые ≥ 0, но не только 0/1 — многоклассовая; иначе регрессия. Объясни, почему это эвристика.

---

## Критерии приемки

- [ ] `cargo test -p lab03-domain-model` проходит;
- [ ] нет `#[ignore]` в тестах лабораторной;
- [ ] `match` по enum'ам не содержит `_ =>` там, где важно обработать все варианты;
- [ ] методы не паникуют на пустых данных;
- [ ] `describe` соответствует формату;
- [ ] `cargo clippy -p lab03-domain-model -- -D warnings` без предупреждений;
- [ ] `cargo fmt --check` проходит.

---

## На защите

1. Чем `struct` отличается от `enum`? Когда что использовать?
2. Почему `Task` — enum, а не набор констант/строк?
3. Что даёт `#[derive(Debug, Clone, PartialEq)]`?
4. Чем `&self`, `&mut self` и `self` отличаются? Когда нужен каждый?
5. Почему `feature_dim` возвращает `Option<usize>`? Что было бы в Java?
6. Почему `match` по `Activation` не требует `_`?
7. Что произойдёт, если добавить новый вариант в `Activation` и не обновить `match`?
8. Почему `Accuracy` сравнивает с порогом 0.5?
9. Чем `f64::is_finite()` отличается от `is_nan()`?
10. Как бы ты расширил модель для многоклассовой классификации?

---

## Подсказки (открывать после попытки)

<details>
<summary>feature_dim</summary>

```rust
pub fn feature_dim(&self) -> Option<usize> {
    let first = self.samples.first()?;
    let dim = first.features.len();

    for sample in &self.samples {
        if sample.features.len() != dim {
            return None;
        }
    }

    Some(dim)
}
```

</details>

<details>
<summary>Activation</summary>

```rust
pub fn apply(&self, x: f64) -> f64 {
    match self {
        Activation::Relu => x.max(0.0),
        Activation::Sigmoid => 1.0 / (1.0 + (-x).exp()),
        Activation::Tanh => x.tanh(),
    }
}

pub fn derivative(&self, x: f64) -> f64 {
    match self {
        Activation::Relu => if x > 0.0 { 1.0 } else { 0.0 },
        Activation::Sigmoid => {
            let s = self.apply(x);
            s * (1.0 - s)
        }
        Activation::Tanh => {
            let t = self.apply(x);
            1.0 - t * t
        }
    }
}
```

</details>

<details>
<summary>Metric</summary>

```rust
pub fn score(&self, predictions: &[f64], targets: &[f64]) -> Option<f64> {
    if predictions.is_empty() || predictions.len() != targets.len() {
        return None;
    }

    let n = predictions.len() as f64;

    match self {
        Metric::Mse => {
            let mut sum = 0.0;
            for (p, t) in predictions.iter().zip(targets) {
                sum += (*p - *t).powi(2);
            }
            Some(sum / n)
        }
        Metric::Mae => {
            let mut sum = 0.0;
            for (p, t) in predictions.iter().zip(targets) {
                sum += (*p - *t).abs();
            }
            Some(sum / n)
        }
        Metric::Accuracy => {
            let mut correct = 0;
            for (p, t) in predictions.iter().zip(targets) {
                if (*p >= 0.5) == (*t >= 0.5) {
                    correct += 1;
                }
            }
            Some(correct as f64 / n)
        }
    }
}
```

`zip` — итератор; если ещё не проходил lab05, замени на индексы `for i in 0..predictions.len()`.

</details>
