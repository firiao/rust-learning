# Лабораторная 06. Модули, тесты и документация

## Цель

Научиться структурировать крейт: модули, видимость, публичный API, unit- и integration-тесты, doc-тесты, `cargo doc` и `clippy`.

## Что нужно знать до начала

- lab01–lab05;
- структуры, ошибки, итераторы;
- `cargo test`.

## Языковой бюджет

Разрешено: `mod`, `pub`, `pub(crate)`, `use`, `crate::`, `super::`, `#[cfg(test)]`, `tests/`, doc-комментарии `///`, `//!`.

Запрещено: `unsafe`, async, внешние крейты.

## Проект

Возьми код lab05 как основу:

```bash
cd ~/rust-learning
cp -r labs/lab05-dataset-parser labs/lab06-modules
```

Удали `target/` и `Cargo.lock` внутри копии, поменяй `name` в `Cargo.toml` на `lab06-modules`, добавь в `members` корневого `Cargo.toml`:

```toml
"labs/lab06-modules",
```

Запуск:

```bash
cargo test -p lab06-modules
```

---

## Подготовительная часть

1. Нарисуй дерево модулей, которое получится:

   ```text
   crate
   ├── dataset
   ├── error
   ├── metrics
   └── parser
   ```

2. Объясни разницу:
   - `pub` — видно снаружи крейта;
   - `pub(crate)` — видно внутри крейта, но не снаружи;
   - без модификатора — видно только в текущем модуле и его потомках (с оговорками).
3. Ответь: чем unit-тест в `src/` отличается от integration-теста в `tests/`?
4. Создай пустой модуль `mod greetings;` и файл `src/greetings.rs` с функцией. Вызови её из `lib.rs`.

---

## Обязательное задание

### Структура

```text
lab06-modules/
├── Cargo.toml
├── src/
│   ├── lib.rs          ← только документация, `mod` и `pub use`
│   ├── dataset.rs      ← Sample, Dataset и методы
│   ├── error.rs        ← DatasetError, Display, Error
│   ├── parser.rs       ← parse_dataset
│   └── metrics.rs      ← mse, mae, accuracy
└── tests/
    └── public_api.rs   ← интеграционные тесты
```

### Требования

1. `lib.rs` не содержит бизнес-логики: только `mod ...;`, `pub use ...;` и crate-level документация `//!`.
2. Публичный API остаётся удобным:
   ```rust
   use lab06_modules::{parse_dataset, Dataset, DatasetError, mse};
   ```
3. Внутренние детали (например, вспомогательная функция разбора строки) — `pub(crate)` или приватные.
4. В каждом модуле есть unit-тесты на его поведение.
5. В `tests/public_api.rs` есть интеграционные тесты, которые используют только публичный API.
6. Хотя бы одна функция имеет doc-комментарий с примером, который проходит как doc-тест.
7. Включи:
   ```rust
   #![warn(missing_docs)]
   ```
   и задокументируй все публичные элементы (или осознанно подави предупреждение, объяснив почему).
8. `cargo doc --no-deps` собирается без предупреждений (кроме, возможно, `missing_docs`, если решишь оставить `warn`).

### Публичный API

Сохрани минимум:

```rust
pub struct Sample { pub features: Vec<f64>, pub label: f64 }
pub struct Dataset { pub samples: Vec<Sample> }

pub enum DatasetError {
    Empty,
    WrongColumnCount { line: usize, expected: usize, actual: usize },
    ParseFloat { line: usize, column: usize, value: String },
}

pub fn parse_dataset(input: &str) -> Result<Dataset, DatasetError>;
pub fn mse(y_true: &[f64], y_pred: &[f64]) -> Option<f64>;
pub fn mae(y_true: &[f64], y_pred: &[f64]) -> Option<f64>;
pub fn accuracy(y_true: &[f64], y_pred: &[f64]) -> Option<f64>;
```

Методы `Dataset` (`len`, `is_empty`, `feature_dim`, ...) можно оставить.

### Тесты

- Unit-тесты: парсер, ошибки, метрики, методы Dataset.
- Integration-тесты: импорт из `lab06_modules`, сценарий «распарсить → посчитать метрику».
- Doc-тест: пример `parse_dataset` в документации.
- Проверь ошибки: пустой ввод, неверное число колонок, нечисловое значение.

---

## Пошаговый план

1. Скопируй lab05, переименуй пакет.
2. Создай файлы модулей и перенеси код.
3. В `lib.rs` оставь только `mod`/`pub use` и документацию.
4. Исправь `use`-пути: `crate::dataset::Dataset`, `super::error::DatasetError` и т.д.
5. Перенеси unit-тесты в соответствующие модули.
6. Добавь `tests/public_api.rs`.
7. Добавь doc-комментарии и doc-тест.
8. Запусти:

   ```bash
   cargo test -p lab06-modules
   cargo clippy -p lab06-modules -- -D warnings
   cargo fmt
   cargo doc -p lab06-modules --no-deps
   ```

---

## Дополнительная часть

Выбери одно или несколько:

- сделай поля `Sample`/`Dataset` приватными и добавь конструкторы/геттеры; сравни удобство API;
- добавь `pub mod prelude` с удобными реэкспортами;
- сделай `DatasetError` через `thiserror` и сравни с ручным `Display`;
- добавь `#[cfg(test)] mod test_helpers` с общей функцией создания датасета;
- настрой `#![deny(missing_docs)]` и доведи документацию до полного покрытия;
- добавь пример в `examples/` (`cargo run --example ...`), который читает CSV из файла.

---

## Критерии приемки

- [ ] `cargo test -p lab06-modules` проходит;
- [ ] `lib.rs` не содержит бизнес-логики;
- [ ] публичный API не шире необходимого;
- [ ] есть unit-, integration- и doc-тесты;
- [ ] `cargo clippy -p lab06-modules -- -D warnings` без предупреждений;
- [ ] `cargo fmt --check` проходит;
- [ ] `cargo doc -p lab06-modules --no-deps` собирается.

---

## На защите

1. Чем `pub` отличается от `pub(crate)`? Приведи пример, где это важно.
2. Как работает путь `super::super::foo`?
3. Почему integration-тесты не видят приватные функции? Это плюс или минус?
4. Чем `use` отличается от `mod`?
5. Зачем нужен `pub use` (re-export)?
6. Что такое doc-тест и зачем он нужен?
7. Почему `lib.rs` лучше держать тонким?
8. Как бы ты организовал крейт с 20 модулями?
9. Что такое `#![warn(missing_docs)]` и почему это полезно?
10. Чем `cargo doc` отличается от `rustdoc` напрямую?

---

## Подсказки (открывать после попытки)

<details>
<summary>lib.rs</summary>

```rust
//! Библиотека для разбора датасетов и расчёта метрик.
//!
//! ```
//! use lab06_modules::{parse_dataset, mse};
//!
//! let dataset = parse_dataset("1,2\n3,4\n").unwrap();
//! assert_eq!(dataset.len(), 2);
//! ```
//!
//! ```

mod dataset;
mod error;
mod metrics;
mod parser;

pub use dataset::{Dataset, Sample};
pub use error::DatasetError;
pub use metrics::{accuracy, mae, mse};
pub use parser::parse_dataset;
```

(Убери лишний пустой doc-блок; он здесь для демонстрации.)

</details>

<details>
<summary>parser.rs</summary>

```rust
use crate::dataset::{Dataset, Sample};
use crate::error::DatasetError;

pub fn parse_dataset(input: &str) -> Result<Dataset, DatasetError> {
    // ...
}
```

</details>
