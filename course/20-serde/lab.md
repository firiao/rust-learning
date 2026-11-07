# Лаба 20. serde и JSON

**Время:** ~15 минут
**Тема:** `Serialize`/`Deserialize`, `default`, ошибки разбора

## Цель

Научиться превращать структуру в JSON и обратно — и понимать, что происходит с пропущенными полями.

## Крейт уже создан

Открывай `practice/l20-serde`. Структура `Config` уже описана с атрибутами:

```rust
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub hidden_dim: usize,
    #[serde(default = "default_epochs")]
    pub epochs: usize,
    #[serde(default)]
    pub verbose: bool,
}
```

Зависимости `serde` и `serde_json` уже в `Cargo.toml`.

```bash
cargo test -p l20-serde      # 5 красных
```

## Реализуй

Две функции:

**1. `to_json(&Config) -> String`** — сериализовать. Подсказка: `serde_json::to_string`.

**2. `from_json(&str) -> Result<Config, serde_json::Error>`** — разобрать. Подсказка: `serde_json::from_str`.

Обрати внимание: `to_json` возвращает `String`, а не `Result`. Реши, что делать с ошибкой
сериализации (подсказка: для структуры без «хитрых» типов она невозможна — но подумай, как
это отразить в коде, чтобы не было `unwrap` без объяснения).

## Эксперимент

Убери `#[serde(default = "default_epochs")]` у поля `epochs` и запусти тест `defaults_are_applied`.

Запиши в `NOTES.md`: что изменилось и почему.

## Критерии приёмки

- [ ] 5 тестов зелёные;
- [ ] `roundtrip` проходит: `Config` → JSON → `Config` даёт то же значение;
- [ ] `missing_required_field_is_error` проходит: без `hidden_dim` — ошибка;
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` разобран эксперимент с `default`.

## Подсказки

<details>
<summary>Реализации</summary>

```rust
pub fn to_json(config: &Config) -> String {
    serde_json::to_string(config).unwrap()
}

pub fn from_json(text: &str) -> Result<Config, serde_json::Error> {
    serde_json::from_str(text)
}
```

Почему `unwrap` в `to_json` допустим: сериализация структуры из `usize`/`bool` не может упасть —
там нет ни файлов, ни невалидных значений. Если бы в структуре был, например, `HashMap` с ключом
не-строкой, `unwrap` был бы плохой идеей.

</details>

<details>
<summary>Как выглядят ошибки</summary>

```text
invalid type: string "много", expected usize at line 1 column 28
missing field `hidden_dim` at line 1 column 15
```

С указанием места — это делает `serde_json` очень удобным для отладки входных данных.

</details>

<details>
<summary>Эксперимент</summary>

Без `#[serde(default = ...)]` поле `epochs` становится обязательным. Тест `defaults_are_applied`
начнёт падать: `from_json(r#"{ "hidden_dim": 8 }"#)` вернёт `Err(missing field 'epochs')`.

`#[serde(default)]` у `verbose` остаётся — `bool` получает `false`.

</details>

---

**Готово?** Дальше — [лекция 21: CLI с clap](../21-clap/lecture.md).
