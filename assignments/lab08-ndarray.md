# Лабораторная 08. `ndarray` и линейная регрессия

## Цель

Перейти от «обычного Rust» к численным вычислениям: освоить `ndarray`, формы массивов, broadcasting, матричное умножение и обучить линейную регрессию градиентным спуском.

Это первая лаба ML-трека. До неё нужно закрыть labs 01–07.

## Что нужно знать до начала

- ownership, срезы, структуры, traits, `Result`/`Option`, итераторы;
- математика: производная, правило цепочки, матричное умножение;
- MSE и градиентный спуск на уровне идеи.

## Языковой бюджет

Разрешено: `ndarray` (`Array1`, `Array2`, `array!`, `s!`), циклы, итераторы, замыкания, `Option`.

Запрещено: `unsafe`, Candle/Burn/tch, ручные градиенты через `Vec<Vec<f64>>` (нужен именно `ndarray`), внешние ML-фреймворки.

## Проект

```bash
cd ~/rust-learning
cargo new --lib labs/lab08-linear-regression
cd labs/lab08-linear-regression
cargo add ndarray@0.17
```

Добавь в `members` корневого `Cargo.toml`:

```toml
"labs/lab08-linear-regression",
```

Запуск:

```bash
cargo test -p lab08-linear-regression
```

---

## Подготовительная часть

1. Прочитай [`docs/05-ml-ecosystem.md`](../docs/05-ml-ecosystem.md), раздел про `ndarray`.
2. Разберись с формой:

   ```rust
   use ndarray::array;

   let x = array![[1.0, 2.0], [3.0, 4.0]]; // shape (2, 2)
   let w = array![0.5, -0.5];               // shape (2,)
   let y = x.dot(&w);                       // shape (2,)
   ```

   Что получится? Проверь в тесте.

3. Посчитай вручную: `X = [[1,2],[3,4]]`, `w = [0.5,-0.5]`. Чему равен `Xw`?
4. Объясни, почему `x.dot(&w)` требует, чтобы `x.shape()[1] == w.len()`.
5. Создай проект, добавь `ndarray`, напиши тест на `dot`.

---

## Теоретическая часть

### Модель

Линейная регрессия:

```text
ŷ = Xw + b
```

- `X` — матрица признаков, shape `(n_samples, n_features)`;
- `w` — веса, shape `(n_features,)`;
- `b` — смещение (bias), скаляр;
- `ŷ` — предсказания, shape `(n_samples,)`.

### Функция потерь

MSE:

```text
L = (1/n) * Σ (ŷ_i - y_i)^2
```

### Градиенты

Обозначим `r = ŷ - y` (вектор ошибок, shape `(n,)`).

```text
dL/dw = (2/n) * X^T r
dL/db = (2/n) * Σ r_i
```

Почему так:

- `dL/dŷ_i = (2/n) * r_i`;
- `ŷ = Xw + b`, поэтому `dŷ/dw = X`, а `dŷ/db = 1`;
- по правилу цепочки получаем `X^T r` и сумму `r`.

### Шаг градиентного спуска

```text
w := w - learning_rate * dL/dw
b := b - learning_rate * dL/db
```

Слишком большой `learning_rate` — loss растёт/взрывается. Слишком маленький — сходится медленно.

---

## Обязательное задание

### Типы и функции

```rust
use ndarray::{Array1, Array2};

pub fn synthetic_data(n: usize) -> (Array2<f64>, Array1<f64>);

pub fn predict(x: &Array2<f64>, weights: &Array1<f64>, bias: f64) -> Array1<f64>;

pub fn mse(y_true: &Array1<f64>, y_pred: &Array1<f64>) -> Option<f64>;

pub fn gradients(
    x: &Array2<f64>,
    y_true: &Array1<f64>,
    weights: &Array1<f64>,
    bias: f64,
) -> Option<(Array1<f64>, f64)>;

pub fn train_linear_regression(
    x: &Array2<f64>,
    y_true: &Array1<f64>,
    learning_rate: f64,
    epochs: usize,
) -> Option<(Array1<f64>, f64, Vec<f64>)>;
```

### Контракт

1. `synthetic_data(n)`:
   - `n == 0` → две пустые структуры корректной формы: `Array2::zeros((0, 2))`, `Array1::zeros(0)`;
   - иначе `X` shape `(n, 2)`, `y` shape `(n,)`;
   - данные детерминированные (без `rand`), `y = 3*x1 - 2*x2 + 5 + маленький шум`;
   - `x1`, `x2` должны принимать разные значения, а не быть константами.
2. `predict`:
   - `X` shape `(n, d)`, `weights` shape `(d,)`;
   - возвращает `Xw + b` shape `(n,)`;
   - предполагает корректные формы (документируй это).
3. `mse`:
   - пустые массивы или разная длина → `None`;
   - иначе `(1/n) * Σ (t - p)^2`.
4. `gradients`:
   - пустой `X`, несовпадение `X.shape()[0] != y.len()`, `X.shape()[1] != weights.len()` → `None`;
   - иначе `(dw, db)` по формулам выше.
5. `train_linear_regression`:
   - те же проверки форм, что и `gradients`; при ошибке → `None`;
   - начальные `weights = zeros(d)`, `bias = 0.0`;
   - на каждой эпохе: предсказание, `mse`, градиенты, обновление;
   - возвращает `(weights, bias, losses)`, где `losses.len() == epochs`;
   - `epochs == 0` → `(zeros, 0.0, vec![])`;
   - `learning_rate <= 0` или `NaN` → `None`.

### Ожидаемое качество

На `synthetic_data(200)`, `learning_rate = 0.1`, `epochs = 5000`:

- loss должен убывать;
- веса должны быть близки к `[3.0, -2.0]` с точностью ~0.1–0.2;
- bias близок к `5.0` с точностью ~0.1–0.2.

Точные допуски зависят от шума; в тестах используй разумный `1e-1` или `2e-1` и проверяй, что финальный loss меньше начального в несколько раз.

### Тесты

- `synthetic_data`: формы, детерминированность (два вызова дают одинаковые данные), `n=0`;
- `predict`: ручной пример `X=[[1,2],[3,4]]`, `w=[0.5,-0.5]`, `b=0`;
- `mse`: известные значения, пустые, разная длина;
- `gradients`: ручной пример, форма `dw`, `db`;
- `train`: loss убывает, веса приближаются к истинным, `epochs=0`, неверные формы, `lr<=0`.

---

## Пошаговый план

1. Создай проект, добавь `ndarray`.
2. Реализуй `synthetic_data`. Начни с простого:
   ```rust
   let x1 = (i as f64 * 0.1).sin();
   let x2 = (i as f64 * 0.17).cos();
   let noise = (i as f64 * 0.37).sin() * 0.01;
   ```
3. Реализуй `predict` через `x.dot(weights) + bias` (скаляр прибавится к каждому элементу через broadcasting).
4. Реализуй `mse` через `zip`/`map`/`sum`.
5. Реализуй `gradients`:
   ```rust
   let prediction = predict(x, weights, bias);
   let residual = &prediction - y_true;
   let dw = x.t().dot(&residual) * (2.0 / n as f64);
   let db = residual.sum() * (2.0 / n as f64);
   ```
6. Реализуй `train_linear_regression` циклом.
7. Напиши тесты, проверь сходимость.
8. Запусти:

   ```bash
   cargo test -p lab08-linear-regression
   cargo clippy -p lab08-linear-regression -- -D warnings
   cargo fmt
   ```

---

## Дополнительная часть

Выбери одно или несколько:

- стандартизируй признаки `(x - mean) / std` и сравни скорость сходимости;
- реализуй mini-batch SGD;
- добавь полиномиальные признаки `[x1, x2, x1^2, x1*x2, x2^2]` и проверь, что линейная модель по-прежнему работает;
- реши нормальное уравнение `w = (X^T X)^-1 X^T y` для 2 признаков вручную (без `ndarray-linalg`) и сравни с GD;
- построй график loss в терминале (ASCII);
- сравни `f32` и `f64` по скорости и точности.

---

## Критерии приемки

- [ ] `cargo test -p lab08-linear-regression` проходит;
- [ ] используются `Array1`/`Array2`, а не `Vec<Vec<f64>>`;
- [ ] `predict`, `mse`, `gradients`, `train` соответствуют контракту;
- [ ] loss убывает, веса приближаются к истинным;
- [ ] есть тесты форм и ошибок;
- [ ] `cargo clippy -p lab08-linear-regression -- -D warnings` без предупреждений;
- [ ] `cargo fmt --check` проходит.

---

## На защите

1. Что такое shape `(n, d)`? Что означает каждое измерение?
2. Чем `x.dot(&w)` отличается от `x * &w`? Где работает broadcasting?
3. Почему в `dL/dw` появляется `X^T`?
4. Выведи `dL/db`.
5. Что произойдёт при слишком большом `learning_rate`?
6. Почему `mse` возвращает `Option`?
7. Почему `synthetic_data` должен быть детерминированным?
8. Чем `Array1` отличается от `Vec<f64>`?
9. Почему `train` возвращает историю loss, а не только веса?
10. Как бы ты добавил регуляризацию L2?

---

## Подсказки (открывать после попытки)

<details>
<summary>predict и mse</summary>

```rust
pub fn predict(x: &Array2<f64>, weights: &Array1<f64>, bias: f64) -> Array1<f64> {
    x.dot(weights) + bias
}

pub fn mse(y_true: &Array1<f64>, y_pred: &Array1<f64>) -> Option<f64> {
    if y_true.is_empty() || y_true.len() != y_pred.len() {
        return None;
    }

    let sum: f64 = y_true
        .iter()
        .zip(y_pred)
        .map(|(t, p)| (t - p).powi(2))
        .sum();

    Some(sum / y_true.len() as f64)
}
```

</details>

<details>
<summary>gradients</summary>

```rust
pub fn gradients(
    x: &Array2<f64>,
    y_true: &Array1<f64>,
    weights: &Array1<f64>,
    bias: f64,
) -> Option<(Array1<f64>, f64)> {
    let n = x.nrows();
    if n == 0 || x.ncols() != weights.len() || n != y_true.len() {
        return None;
    }

    let prediction = predict(x, weights, bias);
    let residual = &prediction - y_true;
    let scale = 2.0 / n as f64;

    let dw = x.t().dot(&residual) * scale;
    let db = residual.sum() * scale;

    Some((dw, db))
}
```

</details>
