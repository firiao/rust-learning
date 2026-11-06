# Лекция 20. serde: JSON и не только

**Время:** ~25 минут

**Нужно знать заранее:** структуры, `derive`, `Result`, `?`.

**Новые слова в этой лекции:**
- **сериализация** — превратить значение в текст (или байты);
- **десериализация** — превратить текст обратно в значение;
- **`Serialize` / `Deserialize`** — трейты «умею в текст» / «умею из текста»;
- **`serde_json`** — конкретный формат: JSON;
- **атрибуты serde** — `#[serde(rename = "...")]` и другие настройки.

---

## 1. Задача

Программа хранит настройки в файле, отправляет данные по сети, читает ответ API. Всё это — превращение структур в текст и обратно.

Руками это писать мучительно: у структуры 10 полей, нужно разобрать строку, проверить типы, обработать пропуски. `serde` делает это за тебя.

---

## 2. Подключение

```bash
cargo add serde --features derive
cargo add serde_json
```

`Cargo.toml`:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

Почему два крейта: `serde` — общий механизм (трейты + `derive`), `serde_json` — конкретный формат. Есть ещё `serde_yaml`, `toml`, `bincode` — они используют тот же `serde`.

---

## 3. Структура ↔ JSON

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User { name: "Аня".to_string(), age: 20 };

    // структура -> строка JSON
    let json = serde_json::to_string(&user)?;
    println!("{json}");                       // {"name":"Аня","age":20}

    // строка JSON -> структура
    let parsed: User = serde_json::from_str(&json)?;
    println!("{parsed:?}");

    Ok(())
}
```

Разберём:

- `#[derive(Serialize, Deserialize)]` — попросить компилятор сгенерировать `impl` для обоих трейтов (лекция 11: `derive` = автогенерация);
- `serde_json::to_string(&user)` — сериализовать; принимает **ссылку**, не забирает владение; возвращает `Result<String, Error>`;
- `serde_json::from_str(&json)` — десериализовать; тип результата указан аннотацией `: User`;
- `?` — если JSON битый, ошибка уйдёт наверх (лекция 10).

### Красивый JSON

```rust
let pretty = serde_json::to_string_pretty(&user)?;
println!("{pretty}");
```

```json
{
  "name": "Аня",
  "age": 20
}
```

---

## 4. Атрибуты serde

Имена полей в Rust и в JSON часто не совпадают (например, `snake_case` против `camelCase`):

```rust
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(rename = "maxTokens")]
    max_tokens: usize,

    #[serde(default)]
    verbose: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
}
```

Что делают:

- `rename = "..."` — в JSON поле называется иначе;
- `default` — если поля в JSON нет, взять значение по умолчанию (`Default`, лекция 11);
- `skip_serializing_if` — не писать поле, если оно `None`.

Ещё полезное:

```rust
#[serde(deny_unknown_fields)]   // ошибка, если в JSON есть лишние поля
#[serde(rename_all = "camelCase")]  // применить rename ко всем полям структуры
```

---

## 5. ML-нить: файл конфигурации

Типичная задача: параметры обучения лежат в JSON.

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TrainConfig {
    #[serde(default = "default_epochs")]
    epochs: usize,
    #[serde(default = "default_lr")]
    learning_rate: f64,
    hidden_dim: usize,
}

fn default_epochs() -> usize { 2000 }
fn default_lr() -> f64 { 0.01 }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = r#"{ "hidden_dim": 16 }"#;
    let cfg: TrainConfig = serde_json::from_str(text)?;

    println!("{cfg:?}");
    // TrainConfig { epochs: 2000, learning_rate: 0.01, hidden_dim: 16 }
    Ok(())
}
```

Разберём `r#"..."#` — «сырая строка»: внутри можно писать кавычки без экранирования. Удобно для JSON-литералов.

Здесь `hidden_dim` обязателен, а `epochs` и `learning_rate` получили значения по умолчанию из функций.

---

## 6. Работа с ошибками

Ошибки `serde_json` информативны:

```rust
let bad = r#"{ "hidden_dim": "много" }"#;
let cfg: TrainConfig = serde_json::from_str(bad)?;
```

```text
invalid type: string "много", expected usize at line 1 column 28
```

`invalid type: string, expected usize` — «ожидал число, получил строку», с указанием строки и колонки.

Если поля нет:

```text
missing field `hidden_dim` at line 1 column 15
```

`missing field` — «не хватает поля». Лечится либо `#[serde(default)]`, либо проверкой входных данных.

---

## 7. Осторожности

**1. `derive` требует, чтобы все поля тоже были сериализуемы.** Если в структуре поле типа, который не умеет `Serialize`, — ошибка компиляции.

```text
error[E0277]: the trait bound `MyType: Serialize` is not satisfied
```

**2. `Deserialize` и владение.** Строки обычно становятся `String`. Если хочешь избежать копирования — есть `&str` с временем жизни, но это уже тонкая настройка.

**3. Не доверяй входным данным.** JSON от пользователя может содержать гигантские числа, вложенность и лишние поля. `deny_unknown_fields` и проверки диапазонов — твоя ответственность.

---

## 8. Что ты теперь знаешь

- `serde` — трейты `Serialize`/`Deserialize`, `serde_json` — формат.
- `#[derive(Serialize, Deserialize)]` генерирует превращение структуры в текст и обратно.
- `to_string(&value)` — в JSON, `from_str::<T>(&text)` — из JSON.
- `rename`, `default`, `skip_serializing_if`, `deny_unknown_fields` — основные атрибуты.
- Ошибки десериализации содержат поле, строку и колонку.

---

## Проверь себя

1. Зачем нужны два крейта: `serde` и `serde_json`?
2. Что делает `#[derive(Serialize, Deserialize)]`?
3. Чем `to_string` отличается от `to_string_pretty`?
4. Что делает `#[serde(default)]`?
5. Почему нельзя десериализовать структуру, у которой поле не умеет `Deserialize`?
6. Что означает ошибка `missing field`?

---

**Дальше:** [лекция 21 — CLI с clap](../21-clap/lecture.md).
