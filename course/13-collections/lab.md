# Лаба 13. Коллекции и первая ML-задача

**Время:** ~20 минут
**Тема:** `Vec`, `HashMap`, `entry`, разбор CSV

> **Языковой бюджет:** только то, что объяснено в лекциях 1–13. Полная таблица — [BUDGET.md](../BUDGET.md).

## Цель

Пять функций: частотный словарь, топ-N, медиана и **первая задача по профилю** — превратить текст
в таблицу чисел.

## Крейт уже создан

Открывай `practice/l13-collections` — пять `todo!()` и шесть тестов.

```bash
cargo test -p l13-collections   # 6 красных
```

## Реализуй

**1. `word_frequencies(text) -> HashMap<String, usize>`** — разбить на слова по любому не-буквенному
символу, привести к нижнему регистру, посчитать через `entry`.

**2. `top_words(freq, n)`** — топ-N: по частоте убывание, при равенстве — по алфавиту.

**3. `median(values) -> Option<f64>`** — отсортировать копию и взять середину.

**4. `parse_line(line) -> Vec<f64>`** — одна строка CSV в числа; что не разобралось — пропустить.

**5. `parse_csv(text) -> Vec<Vec<f64>>`** — все строки в таблицу, пустые строки пропустить.

## Эксперимент

Закомментируй в `top_words` вторую часть сортировки:

```rust
pairs.sort_by(|a, b| b.1.cmp(&a.1));   // без then_with
```

Запусти тест `ties_are_alphabetical` несколько раз. Он будет падать через раз — почему? Запиши
в `NOTES.md`.

## Критерии приёмки

- [ ] 6 тестов зелёные и стабильны при повторных запусках;
- [ ] `word_frequencies` использует `entry`;
- [ ] `top_words` не зависит от порядка `HashMap`;
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` объяснено, почему без `then_with` тест «мигает».

## Подсказки

<details>
<summary>`word_frequencies`</summary>

```rust
let mut freq = HashMap::new();
for word in text.split(|c: char| !c.is_alphabetic()) {
    if word.is_empty() {
        continue;
    }
    *freq.entry(word.to_lowercase()).or_insert(0) += 1;
}
freq
```

`split` с замыканием-предикатом: разделитель — любой символ, для которого условие истинно.

</details>

<details>
<summary>`top_words`</summary>

```rust
let mut pairs: Vec<(String, usize)> = freq.iter().map(|(w, c)| (w.clone(), *c)).collect();
pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
pairs.truncate(n);
pairs
```

`b.1.cmp(&a.1)` — по частоте по убыванию. `then_with` — второй критерий при равенстве.

</details>

<details>
<summary>`median`</summary>

```rust
if values.is_empty() {
    return None;
}
let mut sorted = values.to_vec();
sorted.sort_by(|a, b| a.total_cmp(b));
let mid = sorted.len() / 2;
if sorted.len().is_multiple_of(2) {
    Some((sorted[mid - 1] + sorted[mid]) / 2.0)
} else {
    Some(sorted[mid])
}
```

`total_cmp` — полное сравнение `f64`, корректно обрабатывает `NaN`.

</details>

<details>
<summary>CSV</summary>

```rust
pub fn parse_line(line: &str) -> Vec<f64> {
    line.split(',')
        .filter_map(|part| part.trim().parse().ok())
        .collect()
}

pub fn parse_csv(text: &str) -> Vec<Vec<f64>> {
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_line)
        .collect()
}
```

</details>

<details>
<summary>Почему без `then_with` тест «мигает»</summary>

Если критерий один, а значения равны, `sort_by` не обязан сохранять исходный порядок (`sort` —
нестабильная сортировка). Исходный порядок берётся из `HashMap`, а он у каждого запуска разный.

Отсюда правило: **если результат должен быть воспроизводимым — задай полный порядок**, даже когда
кажется, что «второй критерий не важен».

</details>

---

**Готово?** Дальше — [лекция 14: итераторы](../14-iterators/lecture.md).
