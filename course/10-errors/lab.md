# Лаба 10. Ошибки

**Время:** ~20 минут
**Тема:** свой `enum` ошибок, `Display`, `?`, `Result`

> **Языковой бюджет:** только то, что объяснено в лекциях 1–10. Полная таблица — [BUDGET.md](../BUDGET.md).

## Цель

Сделать полноценный тип ошибки и код, который «не падает, а сообщает».

## Крейт уже создан

Открывай `practice/l10-errors` — `src/lib.rs` с `todo!()` и восемью тестами.

```bash
cargo test -p l10-errors     # 8 красных
```

В заготовке уже есть:
- `enum ParseError` с тремя вариантами;
- `impl std::error::Error for ParseError {}` — пустая, но нужная (объяснение в лекции).

Реализовать надо: `Display` и три функции.

## Реализуй

**1. `impl Display for ParseError`** — `match` по вариантам, `write!(f, "...")`:

```text
Empty                      -> "пустая строка"
NotANumber(s)              -> "не число: {s:?}"     ← кавычки от {:?}
OutOfRange { value, min, max } -> "{value} вне диапазона {min}..={max}"
```

**2. `parse_age(text) -> Result<u8, ParseError>`** — порядок важен:

1. `trim()`, пусто → `Empty`;
2. `parse::<i64>()`, ошибка → `NotANumber` (через `map_err`);
3. вне `0..=150` → `OutOfRange`;
4. `Ok(value as u8)`.

**3. `sum_csv(text) -> Result<i64, ParseError>`** — разбить по `,`, пустые куски пропустить,
если чисел не было → `Empty`.

**4. `average_csv(text) -> Result<f64, ParseError>`** — вызвать `sum_csv(text)?` и разделить
на количество, **не повторяя** разбор.

## Эксперимент

Поменяй тип возврата `average_csv` на `f64` и оставь внутри `let total = sum_csv(text)?;`.

Запиши в `NOTES.md` текст ошибки: это ровно тот случай, про который говорит лекция.

## Дополнительное задание — свой тип ошибки проектируешь сам

Опиши перечисление ошибок для функции «разделить одно число на другое»
(минимум: деление на ноль; подумай, нужны ли ещё варианты) и функцию, которая возвращает
`Result<f64, ТвояОшибка>`.

**Реши сам:** сколько вариантов, какие поля, что печатает `Display`.
Напиши тест сам. В `NOTES.md` — почему именно такие варианты.

## Критерии приёмки

- [ ] 8 тестов зелёные;
- [ ] нигде нет `unwrap()` и `panic!()`;
- [ ] `average_csv` переиспользует `sum_csv` через `?`, а не копирует разбор;
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
<summary>`sum_csv` и `average_csv`</summary>

```rust
pub fn sum_csv(text: &str) -> Result<i64, ParseError> {
    let mut total: i64 = 0;
    let mut count = 0;
    for part in text.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let value: i64 = part
            .parse()
            .map_err(|_| ParseError::NotANumber(part.to_string()))?;
        total += value;
        count += 1;
    }
    if count == 0 {
        return Err(ParseError::Empty);
    }
    Ok(total)
}

pub fn average_csv(text: &str) -> Result<f64, ParseError> {
    let total = sum_csv(text)?;
    let count = text.split(',').filter(|p| !p.trim().is_empty()).count();
    Ok(total as f64 / count as f64)
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
