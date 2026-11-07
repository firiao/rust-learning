# Лаба 27. Candle

**Время:** ~25 минут
**Тема:** `VarMap`, `Linear`, autograd, `AdamW`, логиты и BCE

> ⚠️ **Сборка тяжёлая.** `candle-core` тянет LLVM, tokenizers и gemm — первая сборка занимает
> несколько минут и грузит все ядра. Перед лабой закрой лишнее и собирай с ограничением:
>
> ```bash
> RAYON_NUM_THREADS=1 cargo test -j 1 -p l27-candle
> ```
>
> Если ноутбук нужен для другого — сделай лабу позже. Она не срочная.

## Крейт уже создан

Открывай `practice/l27-candle`. Уже написаны: `xor_data`, `train_xor` (цикл обучения), `accuracy`.
Реализовать надо три метода `Mlp` — ровно те, где раньше был ручной backprop.

```bash
RAYON_NUM_THREADS=1 cargo test -j 1 -p l27-candle   # 2 красных
```

## Реализуй

**1. `Mlp::new(input_dim, hidden_dim, vb, varmap)`** — два линейных слоя:

```rust
let l1 = linear(input_dim, hidden_dim, vb.pp("l1"))?;
let l2 = linear(hidden_dim, 1, vb.pp("l2"))?;
```

**2. `Mlp::forward(&self, x)`** — **логиты** (без sigmoid):

```rust
let h = self.l1.forward(x)?.relu()?;
self.l2.forward(&h)
```

**3. `Mlp::probabilities(&self, x)`** — `candle_nn::ops::sigmoid(&self.forward(x)?)`.

## Эксперимент

В `train_xor` замени

```rust
weight_decay: 0.0,
```

на

```rust
weight_decay: 0.01,
```

(это значение по умолчанию). Запусти тест `learns_xor` несколько раз.

Запиши в `NOTES.md`: меняется ли результат и насколько стабильно.

## Критерии приёмки

- [ ] 2 теста зелёные (сеть сходится, accuracy `1.0`);
- [ ] `forward` возвращает **логиты** — sigmoid только в `probabilities`;
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` записан эксперимент с `weight_decay = 0.01`;
- [ ] после лабы: `cargo clean -p l27-candle` (сборка Candle весит сотни мегабайт).

## Подсказки

<details>
<summary>Реализации</summary>

```rust
pub fn new(
    input_dim: usize,
    hidden_dim: usize,
    vb: VarBuilder,
    varmap: VarMap,
) -> Result<Self> {
    let l1 = linear(input_dim, hidden_dim, vb.pp("l1"))?;
    let l2 = linear(hidden_dim, 1, vb.pp("l2"))?;
    Ok(Self { l1, l2, varmap })
}

pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
    let h = self.l1.forward(x)?.relu()?;
    self.l2.forward(&h)
}

pub fn probabilities(&self, x: &Tensor) -> Result<Tensor> {
    candle_nn::ops::sigmoid(&self.forward(x)?)
}
```

`Linear::forward` — метод трейта `Module`, он уже импортирован в файле.

</details>

<details>
<summary>Если не компилируется</summary>

- `no method named forward found for Linear` → нужен `use candle_nn::Module;` (он уже есть).
- `no method named backward_step` → нужен `use candle_nn::Optimizer;` (тоже уже есть).
- `cannot find function binary_cross_entropy_with_logit` → имя именно такое, с `logit`
  в единственном числе.

</details>

<details>
<summary>Почему `hidden = 16`, а не 8</summary>

С `hidden = 8` примерно 2–3% запусков застревают: часть ReLU-нейронов «умирает», loss
останавливается около `0.3466`, две вероятности ровно `0.5`. С `hidden = 16` проверено 100/100
успешных запусков.

</details>

---

**Готово?** Дальше — [лекция 28: инференс](../28-inference-pyo3/lecture.md).
