# Лаба 16. Модули, тесты, документация

**Время:** ~20 минут
**Тема:** `mod`, `pub`, unit/интеграционные/doc-тесты, `clippy`

## Цель

Разложить код по модулям, спрятать внутреннее и проверить публичный API **снаружи**.

## Крейт уже создан

Открывай `practice/l16-modules` — это уже **несколько файлов**, как в настоящем проекте:

```text
practice/l16-modules/
├── src/
│   ├── lib.rs          ← pub mod geometry; pub mod text; + pub use
│   ├── geometry.rs     ← area() с todo!() + unit-тесты + doc-тест
│   └── text.rs         ← normalize() с todo!() + unit-тесты + doc-тест
└── tests/
    └── integration.rs  ← тест «снаружи», видит только pub
```

```bash
cargo test -p l16-modules    # красные: unit, doc и integration
```

## Реализуй

**1. `geometry::area(width, height)`** — площадь.

**2. `text::normalize(text)`** — обрезать края, привести к нижнему регистру, схлопнуть внутренние
пробелы в один. Подсказка: `split_whitespace()` делает и обрезку, и схлопывание.

Больше ничего писать не надо: тесты уже есть, включая doc-примеры в `///`.

## Эксперимент

В `src/lib.rs` замени

```rust
pub use geometry::area;
```

на

```rust
use geometry::area;
```

и запусти `cargo test -p l16-modules`. Запиши в `NOTES.md`, что сломалось и почему (подсказка:
сломался именно интеграционный тест).

## Критерии приёмки

- [ ] `cargo test -p l16-modules` — зелёные **все три группы**: unit, integration, doc;
- [ ] ни в одном файле нет `unwrap()` на пользовательских данных;
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` объяснено, зачем нужен `pub use`.

## Подсказки

<details>
<summary>`area`</summary>

```rust
width * height
```

</details>

<details>
<summary>`normalize`</summary>

```rust
text.split_whitespace()
    .map(|w| w.to_lowercase())
    .collect::<Vec<_>>()
    .join(" ")
```

`split_whitespace()` уже убирает пробелы по краям и схлопывает внутренние, поэтому отдельный
`trim()` не нужен.

</details>

<details>
<summary>Эксперимент</summary>

```text
error[E0432]: unresolved import `l16_modules::area`
```

`use` — приватный псевдоним **внутри** крейта. `pub use` — реэкспорт, он делает имя доступным
снаружи. Интеграционный тест — внешний пользователь, поэтому без `pub` он ничего не видит.

</details>

<details>
<summary>Как посмотреть, что тесты действительно три вида</summary>

```bash
cargo test -p l16-modules --lib          # unit
cargo test -p l16-modules --test integration   # интеграционный
cargo test -p l16-modules --doc          # doc-тесты
```

</details>

---

**Готово?** Дальше — [лекция 17: умные указатели](../17-smart-pointers/lecture.md).
