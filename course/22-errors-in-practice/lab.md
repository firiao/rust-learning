# Лаба 22. Ошибки в практике

**Время:** ~20 минут
**Тема:** `thiserror` для библиотеки, `anyhow` для приложения, `context`

## Цель

Почувствовать разницу: точный тип ошибки внизу, контекст для человека — наверху.

## Крейт уже создан

Открывай `practice/l22-errors`. Тип ошибки уже описан через `thiserror`:

```rust
#[derive(Debug, Error, PartialEq)]
pub enum LoadError {
    #[error("пустой ввод")]
    Empty,
    #[error("не число: {0}")]
    NotANumber(String),
    #[error("{0} слишком велико")]
    TooBig(i64),
}
```

`#[derive(Error)]` + `#[error("...")]` заменяют ручные `impl Display` и `impl std::error::Error`
из лекции 10.

```bash
cargo test -p l22-errors     # 5 красных
```

## Реализуй

**1. `parse_small(text) -> Result<u32, LoadError>`**

1. `trim()`, пусто → `Empty`;
2. `parse::<i64>()`, ошибка → `NotANumber`;
3. `>= 1000` → `TooBig`;
4. иначе `Ok(value as u32)`.

**2. `load_and_double(text) -> anyhow::Result<u32>`**

```rust
let value = parse_small(text).context("не удалось разобрать число")?;
Ok(value * 2)
```

Здесь `?` конвертирует `LoadError` в `anyhow::Error` автоматически.

## Эксперимент

Поменяй `format!("{err:#}")` в тесте на `format!("{err}")` (без `#`) и запусти.

Запиши в `NOTES.md`: чем отличаются выводы и что показывает каждый.

## Критерии приёмки

- [ ] 5 тестов зелёные;
- [ ] `parse_small` возвращает **свой** тип, а `load_and_double` — `anyhow::Result`;
- [ ] `error_messages_are_human_readable` проходит без ручного `impl Display` (его генерирует `thiserror`);
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` разница между `{err}` и `{err:#}`.

## Подсказки

<details>
<summary>`parse_small`</summary>

```rust
let trimmed = text.trim();
if trimmed.is_empty() {
    return Err(LoadError::Empty);
}
let value: i64 = trimmed
    .parse()
    .map_err(|_| LoadError::NotANumber(trimmed.to_string()))?;
if value >= 1000 {
    return Err(LoadError::TooBig(value));
}
Ok(value as u32)
```

</details>

<details>
<summary>Что показывает `{err:#}`</summary>

```text
не удалось разобрать число

Caused by:
    не число: abc
```

Без `#` — только верхний уровень:

```text
не удалось разобрать число
```

`#` в `anyhow` означает «показать всю цепочку причин». Именно для этого `.context()` и нужен.

</details>

<details>
<summary>Когда `thiserror`, а когда `anyhow`</summary>

- **`thiserror`** — когда ошибку будут **различать**: `match` по вариантам, реакция на конкретную причину.
- **`anyhow`** — когда ошибку будут **показывать**: программа всё равно печатает сообщение и выходит.

В одном проекте обычно оба: библиотечные модули отдают точные типы, `main` собирает их в `anyhow`.

</details>

---

**Готово?** Дальше — [лекция 23: макросы и unsafe](../23-macros-unsafe/lecture.md).
