# Лаба 19. Cargo глубже

**Время:** ~15 минут
**Тема:** features, `dbg!`, `cargo tree`, профили

> **Языковой бюджет:** только то, что объяснено в лекциях 1–19. Полная таблица — [BUDGET.md](../BUDGET.md).

## Цель

Ощутить, как feature меняет **что вообще компилируется** в крейте.

## Крейт уже создан

Открывай `practice/l19-cargo`. В `Cargo.toml` уже объявлена feature:

```toml
[features]
default = []
extra = []
```

А в `lib.rs` — функция, которая видна **только** при включённой feature:

```rust
#[cfg(feature = "extra")]
pub fn extra() -> u32 { todo!() }
```

```bash
cargo test -p l19-cargo                  # 1 тест (base)
cargo test -p l19-cargo --features extra # 2 теста
```

## Реализуй

**1. `base()`** — вернуть `42`.
**2. `extra()`** — вернуть `100` (видна только с feature).

Второй тест `extra_works_only_with_feature` тоже помечен `#[cfg(feature = "extra")]` — он просто
не существует в обычной сборке.

## Эксперимент

1. Запусти без feature и посмотри, сколько тестов нашлось.
2. Запусти с feature.
3. Запусти `cargo tree -p l19-cargo` и `cargo tree -p l11-traits` — сравни.
4. Запусти `cargo doc -p l19-cargo --no-deps` и открой `target/doc/l19_cargo/index.html`.

Запиши в `NOTES.md`: сколько тестов в каждом режиме и что видно в документации про `extra`.

## Критерии приёмки

- [ ] `cargo test -p l19-cargo` — 1 passed;
- [ ] `cargo test -p l19-cargo --features extra` — 2 passed;
- [ ] `cargo clippy -p l19-cargo --all-targets -- -D warnings` чистый в обоих режимах;
- [ ] в `NOTES.md` записано, чем отличаются сборки с feature и без.

## Подсказки

<details>
<summary>Реализации</summary>

```rust
pub fn base() -> u32 {
    42
}

#[cfg(feature = "extra")]
pub fn extra() -> u32 {
    100
}
```

</details>

<details>
<summary>Почему `--features` работает, а после `--` — нет</summary>

```bash
cargo clippy --features extra --all-targets -- -D warnings   # правильно
cargo clippy --all-targets -- -D warnings --features extra   # неправильно
```

Всё, что стоит **после** `--`, уходит инструменту `clippy-driver`, а не `cargo`. `--features` —
флаг `cargo`, он должен быть до `--`. Та же история с `cargo test -- --nocapture`.

</details>

<details>
<summary>Что делает `#[cfg(...)]`</summary>

`cfg` — «условная компиляция»: если условие ложно, элемент **не попадает** в код вообще.
Это не `if` в рантайме — в бинарнике его просто нет.

`#[cfg(test)]` (лекция 16) — тот же механизм, только условие «мы собираем тесты».

</details>

---

**Готово?** Дальше — [лекция 20: serde](../20-serde/lecture.md).
