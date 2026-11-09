# Лаба 10. Ошибки

**Время:** ~20 минут
**Тема:** свой `enum` ошибок, `Display`, `?`, `Result`

> **Языковой бюджет:** только то, что объяснено в лекциях 1–10. Полная таблица — [BUDGET.md](../BUDGET.md).

## Цель

Сделать полноценный тип ошибки и код, который «не падает, а сообщает».

## Крейт уже создан

Открывай `practice/l10-errors` — `src/lib.rs` с `todo!()` и восемью тестами.

```bash
cargo test -p l10-errors     # 6 красных
```

В заготовке уже есть:
- `enum ParseError` с тремя вариантами;
- `impl std::error::Error for ParseError {}` — пустая, но нужная (объяснение в лекции).

Реализовать надо: `Display` и три функции.

## Реализуй

**1. `impl Display for ParseError`** — `match` по вариантам, `write!(f, "...")`:

```text
Empty                          -> "пустая строка"
NotANumber(s)                  -> "не число: {s:?}"     ← кавычки от {:?}
OutOfRange { value, min, max } -> "{value} вне диапазона {min}..={max}"
DivisionByZero                 -> "деление на ноль"
```

**2. `parse_number(text) -> Result<i64, ParseError>`** — `trim()`, пусто → `Empty`,
`parse::<i64>()` с `map_err` → `NotANumber`.

**3. `parse_age(text) -> Result<u8, ParseError>`** — вызвать `parse_number(text)?`, проверить
диапазон `0..=150`, вернуть `Ok(value as u8)`. Разбор **не дублировать**.

**4. `divide(a: i64, b: i64) -> Result<f64, ParseError>`** — на ноль `DivisionByZero`,
иначе `Ok(a as f64 / b as f64)`.

**5. `average_age(first, second) -> Result<f64, ParseError>`** — два `?` и среднее.

## Эксперимент

Поменяй тип возврата `average_age` на `f64` и оставь внутри `let a = parse_age(first)?;`.

Запиши в `NOTES.md` текст ошибки: это ровно тот случай, про который говорит лекция.

## Дополнительное задание — свой тип ошибки проектируешь сам

Опиши перечисление ошибок для функции «разделить одно число на другое»
(минимум: деление на ноль; подумай, нужны ли ещё варианты) и функцию, которая возвращает
`Result<f64, ТвояОшибка>`.

**Реши сам:** сколько вариантов, какие поля, что печатает `Display`.
Напиши тест сам. В `NOTES.md` — почему именно такие варианты.

## Критерии приёмки

- [ ] 6 тестов зелёные;
- [ ] нигде нет `unwrap()` и `panic!()` в реализациях;
- [ ] `parse_age` переиспользует `parse_number` через `?`, а не копирует разбор;
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` разобран эксперимент с `?` в функции без `Result`.

## Подсказки

<details>
<summary>`Display`</summary>

```rust
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "пустая строка"),
            ParseError::NotANumber(s) => write!(f, "не число: {s:?}"),
            ParseError::OutOfRange { value, min, max } => {
                write!(f, "{value} вне диапазона {min}..={max}")
            }
        }
    }
}
```

`write!` — как `println!`, но пишет в `f` и возвращает `Result`.

</details>

<details>
<summary>`parse_age`</summary>

```rust
let trimmed = text.trim();
if trimmed.is_empty() {
    return Err(ParseError::Empty);
}
let value: i64 = trimmed
    .parse()
    .map_err(|_| ParseError::NotANumber(trimmed.to_string()))?;
if !(0..=150).contains(&value) {
    return Err(ParseError::OutOfRange { value, min: 0, max: 150 });
}
Ok(value as u8)
```

`map_err` заменяет ошибку разбора (`ParseIntError`) на свою; `?` возвращает её наверх.

</details>

<details>
<summary>`parse_number` и `parse_age`</summary>

```rust
pub fn parse_number(text: &str) -> Result<i64, ParseError> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(ParseError::Empty);
    }
    trimmed
        .parse()
        .map_err(|_| ParseError::NotANumber(trimmed.to_string()))
}

pub fn parse_age(text: &str) -> Result<u8, ParseError> {
    let value = parse_number(text)?;
    if !(0..=150).contains(&value) {
        return Err(ParseError::OutOfRange { value, min: 0, max: 150 });
    }
    Ok(value as u8)
}
```

`?` здесь делает то же, что `match` из лекции, только короче: ошибка `parse_number` уходит наверх
как есть.

</details>

<details>
<summary>`divide` и `average_age`</summary>

```rust
pub fn divide(a: i64, b: i64) -> Result<f64, ParseError> {
    if b == 0 {
        return Err(ParseError::DivisionByZero);
    }
    Ok(a as f64 / b as f64)
}

pub fn average_age(first: &str, second: &str) -> Result<f64, ParseError> {
    let a = parse_age(first)?;
    let b = parse_age(second)?;
    Ok((a as f64 + b as f64) / 2.0)
}
```

</details>

<details>
<summary>Эксперимент</summary>

```text
error[E0277]: the `?` operator can only be used in a function that returns
              `Result` or `Option`
```

`?` при ошибке должен что-то вернуть. Функция, возвращающая `f64`, вернуть ошибку не может.

</details>

---

**Готово?** Дальше — [лекция 11: трейты](../11-traits/lecture.md).
