# Лаба 26. MLP с нуля

**Время:** ~25 минут
**Тема:** forward и backprop руками, ReLU, sigmoid, BCE

> **Языковой бюджет:** только то, что объяснено в лекциях 1–26. Полная таблица — [BUDGET.md](../BUDGET.md).

## Цель

Написать те две функции, без которых нет нейросети: прямой проход и обратное распространение.

## В крейте только спека

`practice/l26-*` содержит **только тесты** — файл `tests/spec.rs`. Никаких структур, функций
и заготовок с `todo!()` там нет: всё пишешь ты.

**Тесты — это и есть ТЗ.** В них видно:

- как называются функции и типы;
- какие у них аргументы и что они возвращают;
- какое поведение считается правильным (это и есть требования).

Открывай `tests/spec.rs` и начинай с первого теста. Пока код не написан, `cargo test -p <пакет>`
не компилируется — это нормально, спека ещё не выполнена.

```bash
cargo test -p l26-mlp    # сначала ошибки компиляции, потом красные тесты
```

## Реализуй

**1. `forward(&self, x) -> (Vec<f64>, f64)`**

```text
h_i = ReLU(w1[2i]·x0 + w1[2i+1]·x1 + b1[i])
z   = Σ w2[i]·h_i + b2
p   = sigmoid(z)
вернуть (h, p)
```

**2. `step(&mut self, x, y, lr) -> f64`**

```text
(h, p) = forward(x)
dz  = p - y
для каждого i:
    dh  = dz · w2[i]
    dz1 = dh, если h[i] > 0, иначе 0
    w2[i] -= lr·dz·h[i]
    w1[2i] -= lr·dz1·x0
    w1[2i+1] -= lr·dz1·x1
    b1[i] -= lr·dz1
b2 -= lr·dz
вернуть BCE = -(y·ln p + (1-y)·ln(1-p))
```

Важно: `dz1` считается по **старым** весам, поэтому `dh` вычисляется до обновления `w2`.

## Дополнительное задание — проектируешь сам

Добавь в `Mlp` метод `accuracy` на произвольных данных (не только XOR) и метрику
`mean_loss(&self, data) -> f64`.

Тест напиши сам.

Запиши в `NOTES.md`, почему выбрал такую сигнатуру или такой тип.

## Критерии приёмки

- [ ] 2 теста зелёные: сеть сходится, accuracy `1.0`;
- [ ] loss падает ниже `0.01` за 10 000 эпох при `lr = 0.1`, `hidden = 8`, `seed = 1`;
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;

## Подсказки

<details>
<summary>`forward`</summary>

```rust
pub fn forward(&self, x: &[f64]) -> (Vec<f64>, f64) {
    let hidden = self.b1.len();
    let mut h = Vec::with_capacity(hidden);
    for i in 0..hidden {
        let z = self.w1[2 * i] * x[0] + self.w1[2 * i + 1] * x[1] + self.b1[i];
        h.push(z.max(0.0));
    }
    let z2: f64 = self.w2.iter().zip(&h).map(|(w, hi)| w * hi).sum::<f64>() + self.b2;
    (h, sigmoid(z2))
}
```

`Vec::with_capacity` + `push` — так `clippy` не ругается и не нужно `vec![0.0; n]` с последующей записью.

</details>

<details>
<summary>`step`</summary>

```rust
pub fn step(&mut self, x: &[f64], y: f64, lr: f64) -> f64 {
    let (h, p) = self.forward(x);
    let dz = p - y;

    for (i, hi) in h.iter().enumerate() {
        let dh = dz * self.w2[i];
        let dz1 = if *hi > 0.0 { dh } else { 0.0 };

        self.w2[i] -= lr * dz * hi;
        self.w1[2 * i] -= lr * dz1 * x[0];
        self.w1[2 * i + 1] -= lr * dz1 * x[1];
        self.b1[i] -= lr * dz1;
    }
    self.b2 -= lr * dz;

    let p = p.clamp(1e-9, 1.0 - 1e-9);
    -(y * p.ln() + (1.0 - y) * (1.0 - p).ln())
}
```

</details>

<details>
<summary>Если loss не падает</summary>

Проверь по порядку:

1. `forward` действительно зависит от `x` (напечатай `h` для разных входов);
2. `dz1` учитывает ReLU (`h > 0`);
3. `dh` считается до обновления `w2`;
4. `p` зажат перед логарифмом.

</details>

---

**Готово?** Дальше — [лекция 27: Candle](../27-candle/lecture.md).
