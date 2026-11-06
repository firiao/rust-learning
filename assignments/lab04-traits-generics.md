# Лабораторная 04. Трейты, generics и trait objects

## Цель

Понять, как в Rust делается полиморфизм без наследования: трейты, default-методы, generics, trait bounds, `dyn Trait` и операторные трейты.

## Что нужно знать до начала

- lab01–lab03;
- структуры, enum'ы, методы;
- `Option`, `match`;
- ownership и ссылки.

## Языковой бюджет

Разрешено: `trait`, `impl Trait for Type`, generics, trait bounds, `Box<dyn Trait>`, `std::fmt`, `std::ops`, `Option`.

Запрещено: макросы кроме `derive`, `unsafe`, внешние крейты, async.

## Проект

Создай крейт:

```bash
cd ~/rust-learning
cargo new --lib labs/lab04-metrics
```

Добавь его в `members` корневого `Cargo.toml`:

```toml
members = [
    # ...
    "labs/lab04-metrics",
]
```

Запуск:

```bash
cargo test -p lab04-metrics
```

---

## Подготовительная часть

1. Прочитай про трейты в официальной книге (глава 10) или в [`docs/02-rust-after-python-java.md`](../docs/02-rust-after-python-java.md).
2. Объясни себе разницу:
   - `fn f<T: Display>(x: T)` — статическая диспетчеризация;
   - `fn f(x: &dyn Display)` — динамическая диспетчеризация.
3. Создай минимальный трейт:

   ```rust
   trait Describable {
       fn name(&self) -> &'static str;
       fn describe(&self) -> String {
           format!("я — {}", self.name())
       }
   }
   ```

   Реализуй его для двух разных структур. Вызови `describe()` у обеих.
4. Ответь: почему в Rust нет `extends`, и что использовать вместо него?

---

## Обязательное задание

### 1. Трейт `Metric`

```rust
pub trait Metric {
    fn name(&self) -> &'static str;
    fn score(&self, y_true: &[f64], y_pred: &[f64]) -> Option<f64>;
    fn higher_is_better(&self) -> bool {
        false
    }
}
```

Реализуй:

- `Mse` — среднее квадратов ошибок;
- `Mae` — среднее модулей ошибок;
- `Accuracy` — доля совпадений после порога 0.5.

Контракт:

1. `score` возвращает `None`, если срезы пустые или длины различаются.
2. `Mse`/`Mae` считают по формуле, без паники.
3. `Accuracy`: `(pred >= 0.5) == (target >= 0.5)`.
4. `higher_is_better`: `false` для MSE/MAE, `true` для Accuracy.
5. `name`: `"mse"`, `"mae"`, `"accuracy"`.

### 2. Generic-функция

```rust
pub fn evaluate<M: Metric>(
    metric: &M,
    y_true: &[f64],
    y_pred: &[f64],
) -> Option<f64>;
```

Просто вызывает `metric.score(...)`. Смысл — показать статическую диспетчеризацию: компилятор мономорфизирует функцию под конкретный тип.

### 3. Trait objects

```rust
pub fn best_metric<'a>(
    metrics: &'a [Box<dyn Metric>],
    y_true: &[f64],
    y_pred: &[f64],
) -> Option<(&'a str, f64)>;
```

Контракт:

1. Для каждой метрики вызвать `score`; `None` пропускать.
2. Если все вернули `None` — вернуть `None`.
3. Если `higher_is_better()` — выбирать максимум, иначе минимум.
4. При равенстве — первая метрика в списке.
5. Вернуть имя метрики и её score.

Пример использования:

```rust
let metrics: Vec<Box<dyn Metric>> = vec![
    Box::new(Mse),
    Box::new(Mae),
    Box::new(Accuracy),
];

let best = best_metric(&metrics, &[0.0, 1.0], &[0.1, 0.9]);
```

### 4. Операторные трейты

Создай:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}
```

Реализуй:

- `std::ops::Add` — покомпонентно;
- `std::ops::Sub` — покомпонентно;
- `std::ops::Mul<f64>` — умножение на скаляр;
- `std::fmt::Display` — формат `"(x, y)"`, например `"(1.5, -2)"`.

Проверь, что работают выражения `v1 + v2`, `v1 - v2`, `v * 2.0`, `println!("{v}")`.

### Тесты

Добавь тесты:

- MSE/MAE/Accuracy на известных значениях;
- пустые срезы и разная длина → `None`;
- `higher_is_better`;
- `evaluate` с конкретной метрикой;
- `best_metric`: максимум для Accuracy, минимум для MSE, все `None`, равенство;
- `Vector2`: сложение, вычитание, умножение, `Display`.

---

## Пошаговый план

1. Создай проект и добавь его в workspace.
2. Определи трейт `Metric` с default-методом.
3. Реализуй `Mse`, `Mae`, `Accuracy`. Начни с одной, протестируй, потом остальные.
4. Напиши `evaluate<M: Metric>`.
5. Собери `Vec<Box<dyn Metric>>` и реализуй `best_metric`. Обрати внимание на lifetime: возвращаемое имя живёт столько же, сколько список метрик.
6. Реализуй `Vector2` и операторные трейты.
7. Запусти:

   ```bash
   cargo test -p lab04-metrics
   cargo clippy -p lab04-metrics -- -D warnings
   cargo fmt
   ```

---

## Дополнительная часть

Выбери одно или несколько:

- добавь метрику `R2` (коэффициент детерминации): `1 - SS_res / SS_tot`; продумай, что делать при `SS_tot == 0`;
- сделай `enum AnyMetric { Mse, Mae, Accuracy }` и реализуй `Metric` для него — сравнение статического и динамического подхода;
- реализуй `Metric for Box<dyn Metric>` через делегирование;
- добавь `impl std::ops::Neg for Vector2`;
- сделай `fn print_all(metrics: &[Box<dyn Metric>])` и объясни, почему `&[Box<dyn Metric>]` лучше, чем `&[&dyn Metric]`, в данном случае.

---

## Критерии приемки

- [ ] `cargo test -p lab04-metrics` проходит;
- [ ] трейт `Metric` имеет default-метод `higher_is_better`;
- [ ] есть и generic-функция, и `dyn Trait`;
- [ ] `best_metric` корректно учитывает `higher_is_better`;
- [ ] `Vector2` реализует `Add`, `Sub`, `Mul<f64>`, `Display`;
- [ ] нет `unsafe` и внешних крейтов;
- [ ] `cargo clippy -p lab04-metrics -- -D warnings` без предупреждений.

---

## На защите

1. Чем трейт отличается от интерфейса Java? Что общего?
2. Что такое default-метод и зачем он нужен?
3. Чем `fn f<T: Metric>(x: &T)` отличается от `fn f(x: &dyn Metric)`? Плюсы/минусы.
4. Почему `dyn Metric` требует `Box` или ссылку?
5. Что такое object safety? Почему не любой трейт можно сделать `dyn`?
6. Зачем нужен `?Sized`? Где ты его встречал?
7. Что такое orphan rule и почему нельзя реализовать внешний трейт для внешнего типа?
8. Чем `Display` отличается от `Debug`?
9. Почему `Vector2` лучше передавать по значению (он `Copy`), а `Vec<f64>` — по ссылке?
10. Как бы ты добавил новую метрику, не меняя `best_metric`?

---

## Подсказки (открывать после попытки)

<details>
<summary>Mse</summary>

```rust
pub struct Mse;

impl Metric for Mse {
    fn name(&self) -> &'static str { "mse" }

    fn score(&self, y_true: &[f64], y_pred: &[f64]) -> Option<f64> {
        if y_true.is_empty() || y_true.len() != y_pred.len() {
            return None;
        }

        let mut sum = 0.0;
        for (t, p) in y_true.iter().zip(y_pred) {
            sum += (t - p).powi(2);
        }
        Some(sum / y_true.len() as f64)
    }
}
```

</details>

<details>
<summary>best_metric</summary>

```rust
pub fn best_metric<'a>(
    metrics: &'a [Box<dyn Metric>],
    y_true: &[f64],
    y_pred: &[f64],
) -> Option<(&'a str, f64)> {
    let mut best: Option<(&'a str, f64)> = None;

    for metric in metrics {
        let Some(score) = metric.score(y_true, y_pred) else {
            continue;
        };

        best = match best {
            None => Some((metric.name(), score)),
            Some((best_name, best_score)) => {
                let better = if metric.higher_is_better() {
                    score > best_score
                } else {
                    score < best_score
                };

                if better {
                    Some((metric.name(), score))
                } else {
                    Some((best_name, best_score))
                }
            }
        };
    }

    best
}
```

`let Some(...) = ... else { ... };` — let-else, стабилен с Rust 1.65.

</details>
