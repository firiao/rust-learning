# Лаба 25. ndarray и линейная регрессия

**Время:** ~20 минут
**Тема:** `Array1`/`Array2`, `dot`, `t()`, градиентный спуск

## Цель

Обучить линейную регрессию на матрицах — ту же, что в лекции, но самому.

## Крейт уже создан

Открывай `practice/l25-ndarray`. `dataset()` уже написан (данные подобраны так, что
истинное решение `w = [3, 2]`, `b = 1`). Остальные четыре функции — с `todo!()`.

```bash
cargo test -p l25-ndarray    # 4 красных
```

## Реализуй

**1. `predict(x, w, b) -> Array1<f64>`** — `x.dot(w) + b`.

**2. `mse(pred, y) -> f64`** — `mean((pred - y)²)`.

**3. `step(x, y, w, b, lr) -> (Array1<f64>, f64, f64)`** — один шаг:

```text
err = pred - y
dw  = (2/n) · Xᵀ·err
db  = (2/n) · Σ err
w'  = w - lr·dw
b'  = b - lr·db
```

Вернуть `(w', b', loss_после_шага)`.

**4. `train(x, y, lr, epochs) -> (Array1<f64>, f64)`** — цикл из `step`.

## Эксперимент

Поставь `lr = 0.5` вместо `0.01` в тесте `trains_line` (не забудь вернуть обратно).

Запиши в `NOTES.md`: что происходит с loss и почему (подсказка: шаг больше, чем нужно).

## Критерии приёмки

- [ ] 4 теста зелёные;
- [ ] `trains_line` даёт `w ≈ [3, 2]`, `b ≈ 1` (допуск `0.05`);
- [ ] нигде нет ручных циклов по элементам матрицы — только операции `ndarray`;
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` разобран эксперимент с `lr = 0.5`.

## Подсказки

<details>
<summary>Реализации</summary>

```rust
pub fn predict(x: &Array2<f64>, w: &Array1<f64>, b: f64) -> Array1<f64> {
    x.dot(w) + b
}

pub fn mse(pred: &Array1<f64>, y: &Array1<f64>) -> f64 {
    let diff = pred - y;
    diff.mapv(|v| v * v).mean().unwrap_or(0.0)
}

pub fn step(x: &Array2<f64>, y: &Array1<f64>, w: &Array1<f64>, b: f64, lr: f64)
    -> (Array1<f64>, f64, f64)
{
    let n = y.len() as f64;
    let pred = predict(x, w, b);
    let err = &pred - y;
    let dw = x.t().dot(&err) * (2.0 / n);
    let db = err.sum() * (2.0 / n);
    let new_w = w - &(&dw * lr);
    let new_b = b - lr * db;
    let loss = mse(&predict(x, &new_w, new_b), y);
    (new_w, new_b, loss)
}

pub fn train(x: &Array2<f64>, y: &Array1<f64>, lr: f64, epochs: usize) -> (Array1<f64>, f64) {
    let mut w = Array1::zeros(2);
    let mut b = 0.0;
    for _ in 0..epochs {
        let (nw, nb, _loss) = step(x, y, &w, b, lr);
        w = nw;
        b = nb;
    }
    (w, b)
}
```

`mapv` — «применить функцию к каждому элементу» (аналог `np.vectorize`).

</details>

<details>
<summary>Эксперимент с `lr = 0.5`</summary>

Слишком большой шаг: параметры перескакивают минимум, loss колеблется или растёт.
`trains_line` упадёт с сообщением вида `w[0] = -1.8` — далеко от 3.

Правило: loss расходится → уменьшай `lr` на порядок.

</details>

---

**Готово?** Дальше — [лекция 26: MLP с нуля](../26-mlp-scratch/lecture.md).
