# Лаба 28. Инференс: softmax, argmax, разбор выхода

**Время:** ~20 минут
**Тема:** что делать с выходом модели — без фреймворков и без скачивания тяжёлых крейтов

## Цель

Собрать «постпроцессинг» модели: из чисел получить класс и вероятность. Это ровно та часть,
которую пишут на Rust в реальном пайплайне — и она ломается чаще, чем сам инференс.

> В лекции 28 упоминались `ort` и `pyo3`. Они **не входят** в эту лабу: они тянут за собой
> ONNX Runtime / Python headers, а это минуты сборки и гигабайты. Смысл лабы — математика
> и разбор данных, а не сборка фреймворка. Кто хочет `ort` — это уже проект, а не лаба.

## Крейт уже создан

Открывай `practice/l28-inference` — четыре `todo!()` и пять тестов. Зависимость только
`serde_json`.

```bash
cargo test -p l28-inference   # 5 красных
```

## Реализуй

**1. `softmax(scores) -> Vec<f64>`** — `exp(x_i - max) / Σ exp(x_j - max)`.
Вычитание максимума — защита от переполнения (проверяется тестом с числами 1000 и 1001).

**2. `argmax(scores) -> Option<usize>`** — индекс максимума, на пустом срезе — `None`.

**3. `parse_scores(json) -> Result<Vec<f64>, serde_json::Error>`** — разобрать `[0.1, 2.0, 0.7]`.

**4. `top_class(json) -> Result<(usize, f64), serde_json::Error>`** — end-to-end:
разобрать, посчитать softmax, вернуть `(индекс, вероятность)`.

## Эксперимент

Убери вычитание максимума в `softmax` и запусти `softmax_survives_big_numbers`.

Запиши в `NOTES.md`: что стало с результатом и почему (подсказка: `exp(1001)`).

## Критерии приёмки

- [ ] 5 тестов зелёные;
- [ ] `softmax` не возвращает `NaN`/`inf` даже на больших числах;
- [ ] `argmax(&[])` — `None`, а не паника;
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` разобран эксперимент без вычитания максимума.

## Подсказки

<details>
<summary>`softmax`</summary>

```rust
if scores.is_empty() {
    return Vec::new();
}
let max = scores.iter().copied().fold(f64::NEG_INFINITY, f64::max);
let exps: Vec<f64> = scores.iter().map(|s| (s - max).exp()).collect();
let sum: f64 = exps.iter().sum();
exps.iter().map(|e| e / sum).collect()
```

`fold(f64::NEG_INFINITY, f64::max)` — максимум без `unwrap`.

</details>

<details>
<summary>`argmax`</summary>

```rust
scores
    .iter()
    .enumerate()
    .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
    .map(|(i, _)| i)
```

`partial_cmp` возвращает `Option` — на `NaN` даёт `None`, поэтому `unwrap_or(Equal)`.

</details>

<details>
<summary>`top_class`</summary>

```rust
let scores = parse_scores(json)?;
let probs = softmax(&scores);
let idx = argmax(&probs).unwrap_or(0);
Ok((idx, probs.get(idx).copied().unwrap_or(0.0)))
```

</details>

<details>
<summary>Эксперимент</summary>

`exp(1001)` в `f64` даёт `inf`; деление `inf / inf` — `NaN`. Тест `is_finite` падает.

Вычитание максимума делает аргументы экспоненты неположительными: `exp(≤0) ≤ 1`, переполнения нет,
а результат тот же — потому что `exp(a-c)/Σexp(b-c)` не меняется при вычитании `c` из всех чисел.

</details>

---

## Курс пройден

Дальше — [CHECKLIST.md](../CHECKLIST.md): 40 пунктов самопроверки и 30 вопросов мок-собеседования.
