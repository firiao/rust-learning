# Лаба 21. CLI с clap

**Время:** ~15 минут
**Тема:** `derive(Parser)`, `try_parse_from`, обязательные и необязательные аргументы

> **Языковой бюджет:** только то, что объяснено в лекциях 1–21. Полная таблица — [BUDGET.md](../BUDGET.md).

## Цель

Научиться разбирать аргументы так, чтобы разбор можно было **протестировать**.

## Крейт уже создан

Открывай `practice/l21-clap`. Структура `Args` с атрибутами уже описана:

```rust
#[derive(Parser, Debug, PartialEq)]
pub struct Args {
    pub path: String,                                   // обязательный позиционный
    #[arg(short, long, default_value_t = 10)]
    pub top: usize,                                     // флаг -t / --top
}
```

```bash
cargo test -p l21-clap      # 5 красных
```

## Реализуй

**`parse_args(items: &[&str]) -> Result<Args, clap::Error>`** — один вызов.

Важно: `items` **не содержит** имени программы, а `clap` его ожидает первым элементом.
Значит, его надо подставить самому.

## Эксперимент

1. Убери подстановку имени программы и оставь `Args::try_parse_from(items)`. Посмотри на ошибку теста
   `parses_path_with_default_top` и объясни её в `NOTES.md`.
2. Запусти `cargo run -p l21-clap -- --help` — но сначала добавь `src/main.rs` с `Args::parse()`,
   чтобы увидеть сгенерированную справку.

## Критерии приёмки

- [ ] 5 тестов зелёные;
- [ ] `parse_args(&[])` и `parse_args(&["--top","5"])` дают `Err` (нет обязательного `path`);
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` объяснено, зачем имя программы и как выглядит `--help`.

## Подсказки

<details>
<summary>Реализация</summary>

```rust
pub fn parse_args(items: &[&str]) -> Result<Args, clap::Error> {
    let mut full = vec!["prog"];
    full.extend_from_slice(items);
    Args::try_parse_from(full)
}
```

`"prog"` — любое имя; оно попадёт в строку `Usage:` в сообщениях об ошибках.

</details>

<details>
<summary>Как выглядит `main.rs` для такой программы</summary>

```rust
use clap::Parser;
use l21_clap::Args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let text = std::fs::read_to_string(&args.path)?;
    println!("строк: {}", text.lines().count());
    let _ = args.top;
    Ok(())
}
```

`Args::parse()` — то же, что `try_parse_from`, но при ошибке печатает сообщение и завершает процесс
с кодом 2.

</details>

<details>
<summary>Почему у `clap` первым идёт имя программы</summary>

Это наследие C: `argv[0]` — имя исполняемого файла. `clap` использует его в `Usage:`.

В реальной программе `std::env::args()` уже включает `argv[0]`, поэтому там подставлять ничего
не нужно — сложность только в тестах, где список аргументов задаёшь ты сам.

</details>

---

**Готово?** Дальше — [лекция 22: ошибки в практике](../22-errors-in-practice/lecture.md).
