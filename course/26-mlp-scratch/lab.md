# Лаба 26. MLP с нуля

**Время:** ~25 минут
**Тема:** forward и backprop руками, ReLU, sigmoid, BCE

## Цель

Написать те две функции, без которых нет нейросети: прямой проход и обратное распространение.

## Крейт уже создан

Открывай `practice/l26-mlp`. Всё, кроме двух методов, уже написано:

- `Rng` — генератор с seed (чтобы результат был воспроизводим);
- `sigmoid(x)`;
- `Mlp::new(hidden, seed)` — инициализация весов;
- `xor_data()`, `train_xor(...)` (цикл обучения), `accuracy(...)`.

Реализовать надо **`Mlp::forward`** и **`Mlp::step`**.

```bash
cargo test -p l26-mlp        # 2 красных
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

## Эксперимент

Замени в `forward` `h.push(z.max(0.0))` на `h.push(z)`. Запусти тест `trains_xor`.

Запиши в `NOTES.md`: что стало с loss и почему (подсказка: без нелинейности сеть снова
превращается в линейную модель).

## Критерии приёмки

- [ ] 2 теста зелёные: сеть сходится, accuracy `1.0`;
- [ ] loss падает ниже `0.01` за 10 000 эпох при `lr = 0.1`, `hidden = 8`, `seed = 1`;
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` разобран эксперимент без ReLU.

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
