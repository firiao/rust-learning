# 01. Cargo — шпаргалка

Cargo — это одновременно сборщик, менеджер зависимостей, тест-раннер, генератор документации и публикатор. Аналог `pip + venv + pytest + sphinx + twine` сразу, но с нормальной воспроизводимостью.

---

## Команды

| Команда | Что делает |
|---|---|
| `cargo new name` | создать binary-проект |
| `cargo new name --lib` | создать library-проект |
| `cargo init` | инициализировать Cargo в текущей папке |
| `cargo build` | собрать debug |
| `cargo build --release` | собрать с оптимизацией |
| `cargo run` | собрать и запустить |
| `cargo run --release` | запустить release |
| `cargo run --bin name` | запустить конкретный bin |
| `cargo check` | проверить без кодогенерации (быстро) |
| `cargo test` | запустить тесты |
| `cargo test name` | тесты, чьё имя содержит `name` |
| `cargo test -- --nocapture` | показать `println!` |
| `cargo test -- --ignored` | запустить `#[ignore]`-тесты |
| `cargo bench` | бенчмарки |
| `cargo fmt` | форматировать |
| `cargo fmt --check` | проверить форматирование |
| `cargo clippy` | линтер |
| `cargo clippy -- -D warnings` | предупреждения как ошибки |
| `cargo doc --open` | собрать и открыть документацию |
| `cargo add crate` | добавить зависимость |
| `cargo remove crate` | удалить зависимость |
| `cargo tree` | дерево зависимостей |
| `cargo update` | обновить версии в пределах semver |
| `cargo clean` | удалить `target/` |
| `cargo metadata` | метаданные проекта в JSON |

---

## `Cargo.toml`: минимум

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2024"
authors = ["Имя <email>"]
description = "Учебный проект"
license = "MIT OR Apache-2.0"

[dependencies]
ndarray = "0.17"

[dev-dependencies]
approx = "0.5"

[profile.release]
opt-level = 3
lto = "thin"
```

- `[dependencies]` — обычные зависимости.
- `[dev-dependencies]` — только для тестов, примеров и бенчмарков.
- `[build-dependencies]` — для `build.rs`.
- `version = "0.17"` означает `^0.17`: `>=0.17.0, <0.18.0` (для версий `0.x` semver особый).
- Точная версия: `version = "=0.17.2"`.
- Git-зависимость: `{ git = "https://...", branch = "main" }`.
- Локальная: `{ path = "../my-lib" }`.

---

## Features

Фичи — это условная компиляция возможностей:

```toml
[dependencies]
candle-core = { version = "0.11", features = ["cuda"] }
```

Свои фичи:

```toml
[features]
default = ["std"]
std = []
gpu = ["dep:some-gpu-crate"]
```

В коде:

```rust
#[cfg(feature = "gpu")]
fn use_gpu() { /* ... */ }
```

Не включай тяжёлые фичи «на всякий случай»: они увеличивают время сборки.

---

## Workspace

Workspace позволяет держать несколько крейтов в одном репозитории с общим `target/` и `Cargo.lock`.

Корневой `Cargo.toml`:

```toml
[workspace]
resolver = "3"
members = [
    "labs/lab01-toolkit",
    "labs/lab02-ownership",
    "projects/guess_game",
]
```

Общие зависимости:

```toml
[workspace.dependencies]
ndarray = "0.17"
```

В крейте:

```toml
[dependencies]
ndarray.workspace = true
```

Команды:

```bash
cargo test --workspace
cargo build -p lab01-toolkit
cargo run -p guess_game
```

---

## Тесты и примеры

```text
my-crate/
├── src/
│   ├── lib.rs
│   └── bin/
│       └── tool.rs
├── tests/
│   └── integration.rs     ← интеграционные тесты
├── examples/
│   └── demo.rs            ← cargo run --example demo
└── benches/
    └── speed.rs
```

- Unit-тесты — в `src/*.rs` в `#[cfg(test)] mod tests`.
- Integration-тесты — в `tests/`, используют публичный API.
- Примеры — в `examples/`, запускаются `cargo run --example demo`.
- Doc-тесты — примеры в `///`-комментариях, запускаются `cargo test`.

---

## Публикация

```bash
cargo login
cargo publish --dry-run
cargo publish
```

Для учебных проектов публикация не нужна.

---

## Частые ошибки

| Ошибка | Причина |
|---|---|
| `no bin target named ...` | неверное имя пакета/бина |
| `failed to select a version` | конфликт версий зависимостей |
| `unresolved import` | забыл `use` или зависимость |
| `the trait bound ... is not satisfied` | не хватает impl/feature/типа |
| `linker not found` | не установлены build tools (на Arch: `base-devel`) |
| `error: could not compile ...` | смотри первую ошибку, а не последнюю |

---

## Полезные крейты для учебы

| Крейт | Зачем |
|---|---|
| `approx` | сравнение float в тестах |
| `proptest` | property-based тесты |
| `thiserror` | удобные ошибки в библиотеках |
| `anyhow` | удобные ошибки в приложениях |
| `rand` | случайные числа |
| `serde` / `serde_json` | сериализация |
| `clap` | CLI-аргументы |
| `tracing` | логирование |
| `criterion` | бенчмарки |

Начинать с них не нужно: сначала стандартная библиотека, потом крейты.
