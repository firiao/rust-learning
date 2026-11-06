# Лекция 21. CLI с clap

**Время:** ~25 минут

**Нужно знать заранее:** структуры, `derive`, `Result`, `Option`.

**Новые слова в этой лекции:**
- **CLI** — программа с командной строкой (`program --flag value`);
- **аргумент** — значение после имени программы;
- **флаг** — аргумент вида `--name` или `-n`;
- **`clap`** — библиотека разбора аргументов;
- **`Parser`** — трейт clap, который превращает структуру в описание аргументов;
- **`#[command]` / `#[arg]`** — атрибуты с настройками.

---

## 1. Задача

В лекции 15 мы разбирали аргументы руками: цикл, `match`, проверки. Работает, но на 20 флагов превращается в кашу, а справка `--help` пишется вручную.

`clap` делает всё это из описания структуры.

```bash
cargo add clap --features derive
```

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

Feature `derive` нужна, чтобы можно было писать `#[derive(Parser)]`.

---

## 2. Минимальная программа

```rust
use clap::Parser;

/// Считает количество слов в файле.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Путь к файлу
    path: String,

    /// Сколько слов показать
    #[arg(short, long, default_value_t = 10)]
    top: usize,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
```

Разберём:

- `#[derive(Parser)]` — сгенерировать разбор аргументов (это процедурный макрос, лекция 11);
- `#[command(version, about, ...)]` — настройки программы: версия, описание;
- комментарии `///` над структурой и полями — **это и есть текст справки**. `clap` берёт их для `--help`;
- `path: String` — обязательный позиционный аргумент (просто значение после имени программы);
- `#[arg(short, long, default_value_t = 10)]` — сделать `top` флагом с короткой и длинной формой и значением по умолчанию;
- `Args::parse()` — разобрать `std::env::args()` и вернуть структуру. При ошибке напечатает сообщение и завершит программу.

Запуск:

```bash
$ cargo run -- README.md
Args { path: "README.md", top: 10 }

$ cargo run -- README.md --top 5
Args { path: "README.md", top: 5 }

$ cargo run -- --top 5
error: the following required arguments were not provided:
  <PATH>
```

Справка генерируется сама:

```bash
$ cargo run -- --help
Считает количество слов в файле

Usage: my-app [OPTIONS] <PATH>

Arguments:
  <PATH>  Путь к файлу

Options:
  -t, --top <TOP>
          Сколько слов показать [default: 10]
  -h, --help
          Print help
  -V, --version
          Print version
```

(в реальном выводе отступы шире; здесь они поджаты для читаемости)

---

## 3. Виды аргументов

```rust
#[derive(Parser, Debug)]
struct Args {
    /// обязательный позиционный
    input: String,

    /// необязательный позиционный
    output: Option<String>,

    /// флаг-переключатель (true, если указан)
    #[arg(short, long)]
    verbose: bool,

    /// флаг со значением и умолчанием
    #[arg(short, long, default_value_t = 1)]
    repeat: u32,

    /// несколько значений: --tag a --tag b
    #[arg(long = "tag")]
    tags: Vec<String>,
}
```

| Вид | Как в коде | Пример вызова |
|---|---|---|
| позиционный | `input: String` | `prog file.txt` |
| необязательный позиционный | `output: Option<String>` | `prog in.txt out.txt` |
| переключатель | `verbose: bool` | `prog --verbose` |
| флаг со значением | `repeat: u32` + `#[arg(short, long)]` | `prog -r 3` |
| повторяемый | `tags: Vec<String>` | `prog --tag a --tag b` |

---

## 4. Значения по умолчанию и перечисления

```rust
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = 2000)]
    epochs: usize,

    #[arg(long, default_value_t = 0.01)]
    learning_rate: f64,

    #[arg(long, value_enum, default_value_t = Optimizer::AdamW)]
    optimizer: Optimizer,
}

#[derive(Debug, Clone, ValueEnum)]
enum Optimizer {
    Sgd,
    AdamW,
}
```

`ValueEnum` позволяет принимать значения по имени: `--optimizer sgd` или `--optimizer adam-w`. Обрати внимание: `AdamW` превращается в `adam-w` (kebab-case).

Это прямой прообраз конфигурации обучения: `epochs`, `learning_rate`, `optimizer` — ровно те поля, что ты будешь передавать в модель.

---

## 5. Разбор и обработка ошибок

`Args::parse()` — самый простой вариант: при ошибке печатает и **завершает программу** с кодом 2.

Если ошибку нужно обработать самому (например, в тестах) — есть `Args::try_parse()`:

```rust
use clap::Parser;

let result = Args::try_parse_from(["prog", "file.txt", "--top", "abc"]);
match result {
    Ok(args) => println!("{args:?}"),
    Err(e) => eprintln!("{e}"),
}
```

`try_parse_from` принимает готовый список аргументов — это удобно для тестов: можно проверить разбор без запуска процесса (ровно та идея из лекции 15: вынеси разбор в тестируемую функцию).

---

## 6. Структура программы

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Путь к файлу
    path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let text = std::fs::read_to_string(&args.path)?;
    println!("символов: {}", text.chars().count());
    Ok(())
}
```

Логика — в отдельных функциях, `main` — тонкая обёртка (лекция 16). Тогда `clap` отвечает только за аргументы, а не за всё сразу.

---

## 7. Типичные ошибки

### Забыл feature `derive`

```text
error[E0433]: failed to resolve: use of undeclared crate or module `clap`
```

Или `#[derive(Parser)]` не компилируется: `cannot find derive macro Parser`. Проверь `features = ["derive"]`.

### Обязательный аргумент забыт

```text
error: the following required arguments were not provided:
  <PATH>
```

### `--` в `cargo run`

```bash
cargo run --top 5          # cargo попытается съесть --top
cargo run -- --top 5       # правильно: всё после -- идёт программе
```

---

## 8. Что ты теперь знаешь

- `clap` разбирает аргументы по описанию структуры.
- `#[derive(Parser)]` + `Args::parse()` — минимум для рабочего CLI.
- Комментарии `///` становятся текстом `--help`.
- `Option<T>` — необязательный аргумент, `bool` — переключатель, `Vec<T>` — повторяемый.
- `default_value_t` — значение по умолчанию, `ValueEnum` — выбор из вариантов.
- `try_parse_from` — для тестов без запуска процесса.

---

## Проверь себя

1. Зачем `clap`, если `std::env::args()` уже есть?
2. Что делает `#[derive(Parser)]`?
3. Откуда берётся текст в `--help`?
4. Чем `bool`-поле отличается от `Option<String>` в структуре аргументов?
5. Зачем нужен `try_parse_from`?
6. Почему в `cargo run` нужен `--`?

---

**Дальше:** [лекция 22 — ошибки в практике](../22-errors-in-practice/lecture.md).
