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

В `top_words` убери вторую половину условия сравнения:

```rust
let better = right.1 > left.1;
```

Запусти тест `ties_are_alphabetical` несколько раз. Он начнёт падать через раз — почему?
Запиши в `NOTES.md`.

## Дополнительное задание — проектируешь сам

Напиши функцию, которая группирует слова по первой букве: `HashMap<char, Vec<String>>`.
**Реши сам:** что делать со словами, начинающимися не с буквы.

Тест напиши сам.

Запиши в `NOTES.md`, почему выбрал такую сигнатуру или такой тип.

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
let mut current = String::new();
for ch in text.chars() {
    if ch.is_alphabetic() {
        current.push(ch);
    } else if !current.is_empty() {
        *freq.entry(current.to_lowercase()).or_insert(0) += 1;
        current.clear();
    }
}
if !current.is_empty() {
    *freq.entry(current.to_lowercase()).or_insert(0) += 1;
}
freq
```

Собираем слово по буквам в `current`, а на любом не-буквенном символе сбрасываем его в словарь.
`to_lowercase()` у `String` возвращает новую строку в нижнем регистре.

</details>

<details>
<summary>`top_words`</summary>

```rust
let mut all: Vec<(String, usize)> = Vec::new();
for (word, count) in freq {
    all.push((word.clone(), *count));
}

for i in 1..all.len() {
    let mut j = i;
    while j > 0 {
        let left = &all[j - 1];
        let right = &all[j];
        let better = right.1 > left.1 || (right.1 == left.1 && right.0 < left.0);
        if better {
            all.swap(j - 1, j);
            j -= 1;
        } else {
            break;
        }
    }
}

let mut top = Vec::new();
for pair in &all {
    if top.len() == n {
        break;
    }
    top.push(pair.clone());
}
top
```

`for (word, count) in freq` по `HashMap` даёт пары `(&String, &usize)` — поэтому `clone()` и `*count`.

</details>

<details>
<summary>`median`</summary>

```rust
if values.is_empty() {
    return None;
}
let mut sorted = values.to_vec();
for i in 1..sorted.len() {
    let mut j = i;
    while j > 0 && sorted[j - 1] > sorted[j] {
        sorted.swap(j - 1, j);
        j -= 1;
    }
}
let mid = sorted.len() / 2;
if sorted.len() % 2 == 0 {
    Some((sorted[mid - 1] + sorted[mid]) / 2.0)
} else {
    Some(sorted[mid])
}
```

`values.to_vec()` — копия среза в `Vec` (срез нельзя менять).

</details>

<details>
<summary>CSV</summary>

```rust
pub fn parse_line(line: &str) -> Vec<f64> {
    let mut numbers = Vec::new();
    for part in line.split(',') {
        if let Ok(value) = part.trim().parse::<f64>() {
            numbers.push(value);
        }
    }
    numbers
}

pub fn parse_csv(text: &str) -> Vec<Vec<f64>> {
    let mut table = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        table.push(parse_line(line));
    }
    table
}
```

</details>

<details>
<summary>Почему без второго критерия тест «мигает»</summary>

Порядок обхода `HashMap` не определён и меняется между запусками. Если два слова встречаются
одинаково часто, а критерий только «по частоте», результат зависит от того, в каком порядке
они лежали в таблице.

Правило: **если результат должен быть воспроизводимым — задай полный порядок**, даже когда
кажется, что «второй критерий не важен».

</details>

---

**Готово?** Дальше — [лекция 14: итераторы](../14-iterators/lecture.md).
