# Лабораторная 09. MLP с нуля: forward и backprop

## Цель

Собрать многослойный перцептрон (MLP) на `ndarray` и **вручную** реализовать forward pass, функцию потерь, backprop и градиентный спуск. Это лабораторная, после которой Candle/Burn перестанут быть магией.

## Что нужно знать до начала

- lab08 (`ndarray`, формы, `dot`, broadcasting);
- производная, правило цепочки;
- ReLU, sigmoid, BCE;
- матричное умножение и транспонирование.

## Языковой бюджет

Разрешено: `ndarray`, структуры, методы, `Option`, итераторы, циклы.

Запрещено: Candle/Burn/tch, autograd-крейты, `unsafe`. **Все градиенты считаются руками.**

## Проект

```bash
cd ~/rust-learning
cargo new --lib labs/lab09-mlp-scratch
cd labs/lab09-mlp-scratch
cargo add ndarray@0.17
```

Добавь в `members` корневого `Cargo.toml`:

```toml
"labs/lab09-mlp-scratch",
```

Запуск:

```bash
cargo test -p lab09-mlp-scratch
```

---

## Подготовительная часть

1. Реализуй и протестируй скалярные функции:

   ```rust
   fn relu(x: f64) -> f64 { x.max(0.0) }
   fn relu_derivative(x: f64) -> f64 { if x > 0.0 { 1.0 } else { 0.0 } }
   fn sigmoid(x: f64) -> f64 { 1.0 / (1.0 + (-x).exp()) }
   fn bce(y: f64, p: f64) -> f64 {
       let p = p.clamp(1e-7, 1.0 - 1e-7);
       -(y * p.ln() + (1.0 - y) * (1.0 - p).ln())
   }
   ```

2. Посчитай вручную производную `sigmoid'(x) = sigmoid(x) * (1 - sigmoid(x))`.

3. Для сети `2 → 1` без скрытого слоя с sigmoid и BCE выведи `dL/dz` и `dL/dw`. Почему получается `p - y`?

4. Создай проект, добавь `ndarray`, напиши тест на `sigmoid(0) == 0.5`.

---

## Теоретическая часть

### Архитектура

```text
x (n, 2)
  │
  ▼
z1 = x W1^T + b1      (n, h)
h  = ReLU(z1)         (n, h)
  │
  ▼
z2 = h W2^T + b2      (n, 1)
p  = sigmoid(z2)      (n, 1)
  │
  ▼
BCE loss
```

- `W1` shape `(h, 2)`, `b1` shape `(h,)`;
- `W2` shape `(1, h)`, `b2` shape `(1,)`;
- `n` — число примеров, `h` — размер скрытого слоя.

### Loss

Binary Cross-Entropy:

```text
L = -(1/n) Σ [ y log(p) + (1-y) log(1-p) ]
```

`p` обрезаем в `[1e-7, 1 - 1e-7]`, чтобы не получить `log(0) = -inf`.

### Backprop

Для sigmoid + BCE магия упрощается:

```text
dz2 = p - y                         (n, 1)
dW2 = (1/n) * dz2^T h               (1, h)
db2 = (1/n) * Σ dz2                 (1,)
dh  = dz2 W2                        (n, h)
dz1 = dh * ReLU'(z1)                (n, h)
dW1 = (1/n) * dz1^T x               (h, 2)
db1 = (1/n) * Σ dz1                 (h,)
```

Обновление:

```text
W1 -= lr * dW1
b1 -= lr * db1
W2 -= lr * dW2
b2 -= lr * db2
```

Важно: `hidden = ReLU(z1)`, поэтому `ReLU'(z1)` можно восстановить как `1`, если `hidden > 0`, иначе `0` (в нуле производная 0).

### Данные XOR

```text
x1  x2  y
0   0   0
0   1   1
1   0   1
1   1   0
```

Один слой (линейная модель) XOR не решает. Скрытый слой с ReLU — решает.

---

## Обязательное задание

### Типы

```rust
use ndarray::{Array1, Array2};

#[derive(Debug, Clone)]
pub struct Mlp {
    pub w1: Array2<f64>, // (hidden, input)
    pub b1: Array1<f64>, // (hidden,)
    pub w2: Array2<f64>, // (output, hidden)
    pub b2: Array1<f64>, // (output,)
}

#[derive(Debug, Clone)]
pub struct Gradients {
    pub dw1: Array2<f64>,
    pub db1: Array1<f64>,
    pub dw2: Array2<f64>,
    pub db2: Array1<f64>,
}

pub fn xor_data() -> (Array2<f64>, Array2<f64>);
```

### `Mlp`

```rust
impl Mlp {
    pub fn new(input_dim: usize, hidden_dim: usize, seed: u64) -> Self;
    pub fn forward(&self, x: &Array2<f64>) -> (Array2<f64>, Array2<f64>);
    pub fn backward(
        &self,
        x: &Array2<f64>,
        y: &Array2<f64>,
        hidden: &Array2<f64>,
        output: &Array2<f64>,
    ) -> Option<Gradients>;
    pub fn train(
        &mut self,
        x: &Array2<f64>,
        y: &Array2<f64>,
        learning_rate: f64,
        epochs: usize,
    ) -> Option<Vec<f64>>;
    pub fn predict(&self, x: &Array2<f64>) -> Array1<f64>;
    pub fn accuracy(&self, x: &Array2<f64>, y: &Array2<f64>) -> Option<f64>;
}
```

### Контракт

1. `xor_data` возвращает `X` shape `(4, 2)` и `y` shape `(4, 1)`.
2. `new`:
   - `input_dim > 0`, `hidden_dim > 0`, иначе паника с понятным сообщением;
   - веса инициализируются детерминированно по `seed` (без `rand`);
   - `w1` shape `(hidden, input)`, `b1` shape `(hidden,)`, `w2` shape `(1, hidden)`, `b2` shape `(1,)`;
   - веса не все нули; разумный диапазон — `[-0.5, 0.5]` или инициализация He.
3. `forward`:
   - `x` shape `(n, input)`;
   - возвращает `(hidden, output)` с формами `(n, hidden)` и `(n, 1)`;
   - `hidden >= 0`, `0 < output < 1`.
4. `backward`:
   - проверяет формы; при несовпадении → `None`;
   - `n == 0` → `None`;
   - возвращает градиенты тех же форм, что и параметры;
   - не содержит `NaN`/`inf` на нормальных данных.
5. `train`:
   - проверяет формы и `learning_rate > 0` и конечен; иначе `None`;
   - начальные параметры — из `new`;
   - на каждой эпохе: forward, BCE, backward, обновление;
   - возвращает историю loss длиной `epochs`;
   - `epochs == 0` → `Some(vec![])`.
6. `predict`:
   - возвращает `Array1<f64>` из 0/1 по порогу 0.5.
7. `accuracy`:
   - пустые данные или несовпадение длины → `None`;
   - иначе доля правильных ответов.
8. BCE loss считай внутри `train` (отдельная публичная функция приветствуется).

### Ожидаемое качество

На XOR, `hidden_dim = 8`, `learning_rate = 0.5`, `epochs = 10_000`:

- loss должен убывать;
- accuracy должна быть `>= 0.99`;
- выходы на четырёх примерах должны быть близки к 0/1.

Если не сходится — проверь:

- не перепутаны ли `W1`/`W2` и транспонирования;
- правильно ли считается `dz1`;
- не забыто ли деление на `n`;
- не слишком ли большой/маленький `lr`;
- не все ли веса нулевые.

### Тесты

- `xor_data`: формы и значения;
- `new`: формы, детерминированность, не все нули;
- `forward`: формы, диапазон output, `hidden >= 0`;
- `backward`: формы, конечные значения;
- `train`: loss убывает, accuracy > 0.99, `epochs=0`, неверные формы, `lr<=0`;
- `predict`/`accuracy`: известные случаи, пустые данные.

---

## Пошаговый план

1. Создай проект, добавь `ndarray`.
2. Реализуй `xor_data`.
3. Реализуй `Mlp::new` с LCG:
   ```rust
   fn next_weight(seed: &mut u64) -> f64 {
       *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
       // Берём старшие 32 бита, чтобы получить равномерное число в [0, 1).
       let unit = ((*seed >> 32) as f64) / (u32::MAX as f64);
       unit - 0.5
   }
   ```
4. Реализуй `forward` и проверь формы на бумаге.
5. Реализуй `backward` по формулам. Сначала проверь на одном примере, потом на всех.
6. Реализуй `train`, печатай loss каждые 1000 эпох (или возвращай историю и смотри в тесте).
7. Реализуй `predict` и `accuracy`.
8. Запусти тесты. Если accuracy низкая — отлаживай по одному шагу.
9. Запусти:

   ```bash
   cargo test -p lab09-mlp-scratch
   cargo clippy -p lab09-mlp-scratch -- -D warnings
   cargo fmt
   ```

---

## Дополнительная часть

Выбери одно или несколько:

- сгенерируй датасет «две луны» детерминированно и обучи MLP на нём;
- добавь mini-batch SGD;
- реализуй Adam вместо SGD;
- добавь L2-регуляризацию;
- сделай сохранение/загрузку весов в текстовый файл без `serde`;
- добавь второй скрытый слой и сравни;
- замени ReLU на tanh и сравни сходимость;
- реализуй многоклассовую классификацию с softmax + cross-entropy;
- построй ASCII-график loss.

---

## Критерии приемки

- [ ] `cargo test -p lab09-mlp-scratch` проходит;
- [ ] forward/backward реализованы вручную, без autograd;
- [ ] нет `unsafe` и ML-фреймворков;
- [ ] loss убывает, XOR accuracy >= 0.99;
- [ ] градиенты проверены хотя бы на одном ручном примере;
- [ ] `cargo clippy -p lab09-mlp-scratch -- -D warnings` без предупреждений;
- [ ] `cargo fmt --check` проходит.

---

## На защите

1. Нарисуй вычислительный граф MLP.
2. Почему `dz2 = p - y`? Выведи.
3. Почему в `dW2` стоит `dz2^T`, а не `dz2`?
4. Почему делим на `n`?
5. Как считается `dz1` и почему используется `hidden > 0`?
6. Что будет, если инициализировать все веса нулями?
7. Почему XOR не решается одним линейным слоем?
8. Что такое `NaN` в loss и откуда он может появиться?
9. Как выбрать `learning_rate`? Что происходит при слишком большом?
10. Чем твоя реализация отличается от Candle/Burn? Что autograd делает за тебя?

---

## Подсказки (открывать после попытки)

<details>
<summary>forward</summary>

```rust
pub fn forward(&self, x: &Array2<f64>) -> (Array2<f64>, Array2<f64>) {
    let z1 = x.dot(&self.w1.t()) + &self.b1;
    let hidden = z1.mapv(|v| v.max(0.0));
    let z2 = hidden.dot(&self.w2.t()) + &self.b2;
    let output = z2.mapv(|v| 1.0 / (1.0 + (-v).exp()));
    (hidden, output)
}
```

</details>

<details>
<summary>backward</summary>

```rust
pub fn backward(
    &self,
    x: &Array2<f64>,
    y: &Array2<f64>,
    hidden: &Array2<f64>,
    output: &Array2<f64>,
) -> Option<Gradients> {
    let n = x.nrows();
    if n == 0
        || x.ncols() != self.w1.ncols()
        || hidden.nrows() != n
        || hidden.ncols() != self.w1.nrows()
        || output.nrows() != n
        || output.ncols() != self.w2.nrows()
        || y.nrows() != n
        || y.ncols() != output.ncols()
        || self.w2.ncols() != hidden.ncols()
    {
        return None;
    }

    let scale = 1.0 / n as f64;

    let dz2 = output - y; // (n, 1)
    let dw2 = dz2.t().dot(hidden) * scale; // (1, h)
    let db2 = dz2.sum_axis(ndarray::Axis(0)) * scale; // (1,)

    let dh = dz2.dot(&self.w2); // (n, h)
    let relu_mask = hidden.mapv(|h| if h > 0.0 { 1.0 } else { 0.0 });
    let dz1 = dh * relu_mask; // (n, h)

    let dw1 = dz1.t().dot(x) * scale; // (h, d)
    let db1 = dz1.sum_axis(ndarray::Axis(0)) * scale; // (h,)

    Some(Gradients { dw1, db1, dw2, db2 })
}
```

</details>

<details>
<summary>train</summary>

```rust
pub fn train(
    &mut self,
    x: &Array2<f64>,
    y: &Array2<f64>,
    learning_rate: f64,
    epochs: usize,
) -> Option<Vec<f64>> {
    if !learning_rate.is_finite() || learning_rate <= 0.0 {
        return None;
    }
    if x.nrows() == 0
        || x.nrows() != y.nrows()
        || x.ncols() != self.w1.ncols()
        || y.ncols() != self.w2.nrows()
    {
        return None;
    }

    let mut losses = Vec::with_capacity(epochs);

    for _ in 0..epochs {
        let (hidden, output) = self.forward(x);

        let n = x.nrows() as f64;
        let clamped = output.mapv(|p| p.clamp(1e-7, 1.0 - 1e-7));
        let mut loss = 0.0;
        for (&target, &prob) in y.iter().zip(clamped.iter()) {
            loss += -(target * prob.ln() + (1.0 - target) * (1.0 - prob).ln());
        }
        losses.push(loss / n);

        let grads = self.backward(x, y, &hidden, &output)?;

        self.w1 = &self.w1 - &(&grads.dw1 * learning_rate);
        self.b1 = &self.b1 - &(&grads.db1 * learning_rate);
        self.w2 = &self.w2 - &(&grads.dw2 * learning_rate);
        self.b2 = &self.b2 - &(&grads.db2 * learning_rate);
    }

    Some(losses)
}
```

</details>
