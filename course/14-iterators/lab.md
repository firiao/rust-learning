# Лаба 14. Итераторы и метрики

**Время:** ~20 минут
**Тема:** адаптеры, потребители, `&&` в `filter`, `fold`

> **Языковой бюджет:** только то, что объяснено в лекциях 1–14. Полная таблица — [BUDGET.md](../BUDGET.md).

## Цель

Пять функций, и все — цепочками итераторов, без ручных `for` (кроме случаев, где без него нельзя).

## Крейт уже создан

Открывай `practice/l14-iterators` — пять `todo!()` и пять тестов.

```bash
cargo test -p l14-iterators   # 5 красных
```

## Реализуй

**1. `mean(values) -> Option<f64>`** — `iter().sum::<f64>()`, пустой срез → `None`.

**2. `mse(y_true, y_pred) -> Option<f64>`** — `zip` + `map` + `sum`, делённое на длину.
Это формула MSE: `(1/n)·Σ(t-p)²`.

**3. `evens_squared(values) -> Vec<i32>`** — квадраты чётных чисел: `[1,2,3,4]` → `[4, 16]`.
`filter` + `map` + `collect`. Здесь пригодится `|&&n|` в `filter` и `|&n|` в `map`.

`|n| ...` — замыкание (функция без имени). Подробно они будут в лекции 15; пока читай это как
«вот тебе элемент — сделай с ним вот это».

**4. `first_big(values, limit) -> Option<i32>`** — `find`.

**5. `product(values) -> i64`** — потребитель `product()`. Пустой срез даёт 1 — это нейтральный
элемент умножения, а не ошибка.

## Эксперимент

Убери `.collect()` в `evens_squared`, оставив цепочку `iter().filter(...).map(...)`, и запусти тест.

Запиши в `NOTES.md`, какое предупреждение выдаёт компилятор и как это связано с ленивостью.

## Дополнительное задание — проектируешь сам

Перепиши `count_char` из лабы 6 в **одну цепочку итераторов** — без ручного `for` и `mut`.

Сравни в `NOTES.md` две версии: какая понятнее, какая быстрее (подумай, а не замеряй).

Запиши в `NOTES.md`, почему выбрал такую сигнатуру или такой тип.

## Критерии приёмки

- [ ] 5 тестов зелёные;
- [ ] в `evens_squared` и `first_big` нет ручного `for`;
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` объяснено, почему без `collect` ничего не происходит.

## Подсказки

<details>
<summary>`mean`</summary>

```rust
if values.is_empty() {
    return None;
}
Some(values.iter().sum::<f64>() / values.len() as f64)
```

`::<f64>` — «рыбка»: подсказка `sum`, в каком типе складывать.

</details>

<details>
<summary>`mse`</summary>

```rust
if y_true.is_empty() || y_true.len() != y_pred.len() {
    return None;
}
let sum: f64 = y_true
    .iter()
    .zip(y_pred.iter())
    .map(|(t, p)| (t - p) * (t - p))
    .sum();
Some(sum / y_true.len() as f64)
```

`zip` останавливается на коротком — поэтому длину надо проверить **до**.

</details>

<details>
<summary>`evens_squared` и `first_big`</summary>

```rust
pub fn evens_squared(values: &[i32]) -> Vec<i32> {
    values
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .collect()
}

pub fn first_big(values: &[i32], limit: i32) -> Option<i32> {
    values.iter().copied().find(|&n| n > limit)
}
```

`copied()` превращает `&i32` в `i32` — так проще сравнивать. `n % 2 == 0` — остаток от деления;
`clippy` предложит `is_multiple_of`, но этот метод в курсе не встречается, поэтому в заготовке
стоит `#![allow(clippy::manual_is_multiple_of)]`.

</details>

<details>
<summary>Эксперимент</summary>

```text
warning: unused `Map` that must be used
note: iterators are lazy and do nothing unless consumed
```

Цепочка адаптеров — это **описание** работы. Всё начинается, когда появляется потребитель
(`collect`, `sum`, `find`, `for`).

</details>

---

**Готово?** Дальше — [лекция 15: замыкания](../15-closures/lecture.md).
