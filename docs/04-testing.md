# 04. Тесты в Rust

Тесты в Rust встроены в язык и Cargo. Не нужен отдельный фреймворк вроде pytest или JUnit для базовых вещей.

---

## Unit-тесты

Живут в том же файле, что и код, в модуле `#[cfg(test)]`. На прод-сборку не влияют.

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn adds_negative_numbers() {
        assert_eq!(add(-2, -3), -5);
    }
}
```

`#[test]` помечает функцию. `cargo test` находит её автоматически.

### Анатомия теста

```rust
#[test]
fn average_of_empty_slice_is_none() {
    // 1. Подготовка (arrange)
    let values: &[f64] = &[];

    // 2. Действие (act)
    let result = average(values);

    // 3. Проверка (assert)
    assert_eq!(result, None);
}
```

### Утверждения

```rust
assert!(condition);
assert!(!condition);

assert_eq!(left, right);       // требует PartialEq + Debug
assert_ne!(left, right);

assert_eq!(left, right, "сообщение с контекстом: {left}");
```

Для float нельзя `assert_eq!` из-за погрешности:

```rust
let x = 0.1 + 0.2;
assert!((x - 0.3).abs() < 1e-9);
```

Или крейт `approx`:

```rust
use approx::assert_relative_eq;
assert_relative_eq!(0.1 + 0.2, 0.3, epsilon = 1e-9);
```

### Ожидаемая паника

```rust
#[test]
#[should_panic(expected = "деление на ноль")]
fn division_by_zero_panics() {
    divide(10, 0);
}
```

Используй редко: в нормальном коде лучше `Result`, а не паника.

### Тест, возвращающий `Result`

```rust
#[test]
fn parses_valid_number() -> Result<(), std::num::ParseIntError> {
    let n: i32 = "42".parse()?;
    assert_eq!(n, 42);
    Ok(())
}
```

Удобно, когда само действие возвращает `Result`.

### `#[ignore]`

```rust
#[test]
#[ignore = "реализуйте функцию и снимите ignore"]
fn prime_two() {
    assert!(is_prime(2));
}
```

Запуск только игнорируемых:

```bash
cargo test -- --ignored
```

В учебных лабах часть тестов помечена `#[ignore]`, чтобы стартовая сборка была зелёной. Снимай `#[ignore]`, когда реализуешь функцию.

---

## Интеграционные тесты

Лежат в `tests/` и используют **публичный** API крейта, как внешний пользователь.

```text
my-crate/
├── src/lib.rs
└── tests/
    └── integration.rs
```

```rust
// tests/integration.rs
use my_crate::add;

#[test]
fn public_api_works() {
    assert_eq!(add(1, 2), 3);
}
```

Каждый файл в `tests/` — отдельный крейт. Внутренние приватные функции отсюда недоступны — это хорошо: тестируется контракт, а не реализация.

---

## Doc-тесты

Примеры в документации компилируются и запускаются как тесты.

```rust
/// Складывает два числа.
///
/// ```
/// assert_eq!(my_crate::add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

`cargo test` запускает и их. Это отличный способ поддерживать документацию рабочей.

---

## Запуск

```bash
cargo test                      # все тесты
cargo test -p lab01-toolkit     # конкретный пакет workspace
cargo test adds_positive        # тесты, чьё имя содержит подстроку
cargo test -- --nocapture       # показать println! из тестов
cargo test -- --test-threads=1  # последовательно (если тесты делят состояние)
cargo test -- --ignored         # только ignored
cargo test --doc                # только doc-тесты
```

Тесты по умолчанию идут параллельно. Если тест меняет глобальное состояние/файлы — либо изолируй, либо запускай последовательно.

---

## Что и как тестировать

Хороший набор:

1. **Обычный случай** — типичный вход.
2. **Границы** — 0, 1, пустая коллекция, максимальное значение.
3. **Ошибочные входы** — неверный формат, `None`, пустая строка.
4. **Свойства** — например, «среднее всегда между min и max».
5. **Регрессии** — тест на каждый найденный баг.

Не тестируй приватные детали, если они могут измениться. Тестируй поведение.

### Плохой тест

```rust
#[test]
fn test1() {
    assert!(true);
}
```

### Хороший тест

```rust
#[test]
fn average_ignores_nothing_and_returns_none_for_empty() {
    assert_eq!(average(&[]), None);
    assert_eq!(average(&[2.0, 4.0]), Some(3.0));
}
```

---

## Тесты и ownership

Тесты — отличный способ почувствовать владение:

```rust
#[test]
fn consume_takes_ownership() {
    let s = String::from("hello");
    let len = consume_and_len(s);
    assert_eq!(len, 5);
    // println!("{s}"); // не скомпилируется: s перемещена
}
```

Если тест не компилируется из-за move — это не «ошибка теста», а полезная информация об API. Возможно, функция должна принимать `&str`.

---

## Property-based тесты (позже)

Крейт `proptest` генерирует случайные входы и проверяет свойства:

```toml
[dev-dependencies]
proptest = "1"
```

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn average_is_between_min_and_max(values in prop::collection::vec(-100.0..100.0f64, 1..100)) {
        let avg = average(&values).unwrap();
        let min = values.iter().copied().fold(f64::INFINITY, f64::min);
        let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        assert!(avg >= min && avg <= max);
    }
}
```

Это не первая тема, но полезная после lab05.

---

## Тесты в этом курсе

- В `labs/lab01-toolkit` и `labs/lab02-ownership` часть тестов помечена `#[ignore]` — снимай по мере реализации.
- В `labs/lab03-domain-model` аналогично.
- Для lab04+ тесты ты пишешь сам по контракту из задания.
- `cargo test --workspace` должен быть зелёным (ignored не считаются падениями).

---

## Частые ошибки

| Симптом | Причина |
|---|---|
| `no tests found` | тесты не в `#[cfg(test)] mod tests` / не `#[test]` |
| `function is never used` | тест не видит приватную функцию — добавь `use super::*;` |
| тест проходит, но `assert` не срабатывает | забыл `#[test]` |
| тесты мешают друг другу | общее глобальное состояние/файлы; изолируй |
| `assert_eq!` не компилируется | тип не реализует `PartialEq`/`Debug`; добавь `#[derive(...)]` |
| float-тест падает | сравнивай с epsilon |

---

## Мини-упражнение

Добавь в lab01 тесты:

- `is_prime(1)` → `false`;
- `is_prime(2)` → `true`;
- `is_prime(49)` → `false` (квадрат простого);
- `is_palindrome("шалаш")` → `true`;
- `is_palindrome("Шалаш")` → `false` (регистр значим);
- `average(&[])` → `None`;
- `average(&[2.0, 4.0])` → `Some(3.0)`.

Запусти `cargo test -p lab01-toolkit`.
