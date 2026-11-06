# 00. Первый запуск

## Что уже установлено

```bash
rustc --version   # rustc 1.99.1
cargo --version   # cargo 1.99.1
cargo clippy --version
cargo fmt --version
```

`rustup` в системе нет — Rust поставлен пакетом Arch Linux. Это нормально: для учёбы достаточно `rustc` + `cargo`. Если позже понадобится несколько toolchain'ов, можно поставить `rustup`, но сейчас это не нужно.

`rust-analyzer` отдельно не установлен. Если работаешь в VS Code:

```bash
# по желанию, один раз
sudo pacman -S rust-analyzer
```

И установи расширение **rust-analyzer** в VS Code. Оно даст автодополнение, переходы, подсветку ошибок и inline-подсказки типов.

Другие полезные расширения:

- **CodeLLDB** — отладка;
- **Even Better TOML** — подсветка `Cargo.toml`;
- **Error Lens** — показывает ошибки компилятора прямо в строке.

---

## Структура Cargo-проекта

```text
hello-rust/
├── Cargo.toml      ← манифест: имя, версия, edition, зависимости
├── Cargo.lock      ← точные версии зависимостей (для приложений коммитится)
└── src/
    ├── main.rs     ← точка входа для binary-крейта
    └── lib.rs      ← корень библиотечного крейта
```

Создать проект:

```bash
cargo new hello-rust          # binary (main.rs)
cargo new my-lib --lib        # library (lib.rs)
cd hello-rust
cargo run
```

`Cargo.toml` минимального проекта:

```toml
[package]
name = "hello-rust"
version = "0.1.0"
edition = "2024"

[dependencies]
```

`edition` — это не версия компилятора, а набор правил языка (2015, 2018, 2021, 2024). В курсе используется **2024**.

---

## Первый проект руками

```bash
cd ~
cargo new --bin hello-rust
cd hello-rust
```

`src/main.rs`:

```rust
fn main() {
    let name = "студент";
    println!("Привет, {name}!");
}
```

```bash
cargo run
cargo check
cargo test
cargo clippy
cargo fmt
```

- `cargo run` — собрать и запустить.
- `cargo check` — быстро проверить типы и заимствования **без** генерации бинарника. Используй чаще всего.
- `cargo test` — собрать тесты и запустить.
- `cargo clippy` — линтер, идиоматичные замечания.
- `cargo fmt` — отформатировать код.

---

## Как запускать материалы курса

Все лабораторные — члены Cargo-workspace в корне `rust-learning`:

```bash
cd ~/rust-learning

cargo test --workspace          # все тесты
cargo test -p lab01-toolkit     # тесты конкретной лабы
cargo test -p lab01-toolkit -- --nocapture   # показать println! из тестов
cargo run -p guess_game         # бинарник-игра
cargo run -p perceptron         # перцептрон
```

Примеры в `examples/` — самостоятельные `.rs`-файлы, не входят в workspace:

```bash
./run_examples.sh
# или вручную:
rustc --edition 2024 examples/01_hello.rs -o /tmp/hello
/tmp/hello
```

---

## Отладка

```rust
fn main() {
    let x = 42;
    dbg!(x);              // печатает x = 42 и файл/строку
    println!("{x:?}");    // Debug-вывод
    println!("{x:#?}");   // красивый Debug-вывод
}
```

Паника:

```bash
RUST_BACKTRACE=1 cargo run
```

`dbg!` возвращает значение, поэтому его можно вставлять в выражение:

```rust
let y = dbg!(x * 2) + 1;
```

---

## Частые вопросы

**Почему `cargo build` такой медленный?**
Rust компилирует много кода и оптимизирует. `cargo check` намного быстрее; используй его во время написания. Первая сборка с зависимостями (Candle, Burn) может занять минуты.

**Где лежат бинарники?**
`target/debug/имя` или `target/release/имя`.

**Как собрать с оптимизацией?**

```bash
cargo run --release
cargo build --release
```

**Как удалить артефакты?**

```bash
cargo clean
```

**Что такое `Cargo.lock`?**
Фиксация точных версий зависимостей. Для приложений и бинарников его коммитят, для библиотек — обычно нет.

**Можно ли писать в одном файле несколько `fn main`?**
Нет, точка входа одна. Для экспериментов создавай отдельные bin-файлы в `src/bin/` или отдельные проекты.

**Как добавить зависимость?**

```bash
cargo add ndarray
cargo add candle-core candle-nn
```

Или вручную в `Cargo.toml`:

```toml
[dependencies]
ndarray = "0.17"
```

После изменения `Cargo.toml` выполни `cargo check`.

---

## Полезные команды на каждый день

```bash
cargo check
cargo test
cargo clippy -- -D warnings
cargo fmt --check
cargo doc --open
cargo tree
cargo update
```

`cargo clippy -- -D warnings` — считать любые предупреждения ошибкой. Хорошая привычка перед сдачей лабы.

---

## Что дальше

- [`01-cargo-cheatsheet.md`](01-cargo-cheatsheet.md) — шпаргалка по Cargo.
- [`02-rust-after-python-java.md`](02-rust-after-python-java.md) — переход с Python/Java.
- [`03-ownership.md`](03-ownership.md) — главная тема Rust.
