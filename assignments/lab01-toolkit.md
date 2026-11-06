# Лабораторная 01. Функции, типы и тесты

## Цель

Освоить структуру Cargo-проекта, функции, примитивные типы, условия, циклы, `Option<T>` и базовые тесты. Это аналог первой лабораторной по Java, но на Rust.

## Что нужно знать до начала

- переменные, условия, циклы и функции в Python;
- как создать и запустить Cargo-проект;
- `cargo test`;
- ничего про ownership, трейты и потоки знать не нужно.

## Языковой бюджет

Разрешено: `fn`, `let`, `mut`, `if`, `while`, `for`, `&str`, `String`, `Vec`, `Option`, `match`/`if let` (по желанию), `#[test]`, `assert!`, `assert_eq!`.

Запрещено на этом этапе: трейты, generics, замыкания, итераторы сверх `for`, `unsafe`, внешние крейты.

## Файлы

- Условие: этот файл.
- Код: [`../labs/lab01-toolkit/src/lib.rs`](../labs/lab01-toolkit/src/lib.rs).
- Запуск: `cargo test -p lab01-toolkit` из корня `rust-learning`.

## Что уже есть

В стартовом проекте:

- реализована `is_even(i64) -> bool`;
- есть проходящие тесты для неё;
- объявлены `is_prime`, `is_palindrome`, `average` с `todo!()`;
- тесты для них помечены `#[ignore]`, чтобы стартовая сборка была зелёной.

`todo!()` — это макрос, который паникует с сообщением «not yet implemented». Он специально оставляет дырку в коде, которую видно компилятору и тебе.

---

## Подготовительная часть — выполнить до обязательной

1. Запусти:

   ```bash
   cd ~/rust-learning
   cargo test -p lab01-toolkit
   ```

   Убедись, что тесты проходят, а часть помечена `ignored`.

2. Открой `labs/lab01-toolkit/src/lib.rs` и найди в каждом тесте три части: подготовку, действие, проверку.

3. Добавь тест `even_zero_is_even` для `is_even(0)`.

4. Запусти тест отдельно:

   ```bash
   cargo test -p lab01-toolkit even_zero_is_even
   ```

5. Запусти все тесты пакета и убедись, что они зелёные.

---

## Обязательное задание

Дополни `labs/lab01-toolkit/src/lib.rs`, не меняя сигнатуры:

```rust
pub fn is_even(number: i64) -> bool;
pub fn is_prime(number: i64) -> bool;
pub fn is_palindrome(text: &str) -> bool;
pub fn average(values: &[f64]) -> Option<f64>;
```

### Контракт

1. `is_even` возвращает `true` для чётных чисел, включая отрицательные и ноль.
2. `is_prime` возвращает `false` для чисел меньше 2.
3. `is_prime(2)` → `true`; составные числа → `false`; квадраты простых (например, 49) → `false`; простые (3, 5, 97) → `true`.
4. `is_prime` не должен переполняться на больших `i64`. Проверяй делители через `divisor <= number / divisor`, а не `divisor * divisor <= number`.
5. `is_palindrome` сравнивает строку **в точности**: регистр и пробелы значимы. `"шалаш"` → `true`, `"Шалаш"` → `false`, `"а роза упала на лапу азора"` → `false` (пробелы значимы).
6. Пустая строка — палиндром (`true`).
7. `average` возвращает `None` для пустого среза и `Some(среднее)` иначе.
8. `average` не изменяет входной срез.
9. Функции ничего не печатают в консоль.
10. В библиотечном коде нет `unwrap()`/`expect()` на пользовательских данных.

> В Rust нет `null`, поэтому в отличие от Java здесь нет отдельной проверки на `null`. Отсутствие результата выражается через `Option<T>`.

### Тесты

Сними `#[ignore]` и добавь собственные тесты:

- для `is_prime`: 0, 1, 2, 3, 4, 49, 97, отрицательное;
- для `is_palindrome`: пустая строка, один символ, чётная/нечётная длина, регистр, пробелы, кириллица;
- для `average`: пустой срез, один элемент, положительные, отрицательные, смешанные.

Не удаляй существующие тесты. Имена тестов — на английском, в стиле `returns_false_for_one`.

---

## Пошаговый план

1. Прочитай `src/lib.rs` целиком. Найди, где заканчивается `is_even` и начинаются заглушки.
2. Реализуй `is_prime` простым перебором делителей от 2 до `number / divisor`.
3. Запусти только тесты простых чисел, сняв `#[ignore]`.
4. Реализуй `is_palindrome` через `chars()` и `eq` с обратным итератором. Проверь, что `chars()` работает с Unicode лучше, чем байтовые индексы.
5. Реализуй `average`: проверь пустоту, посчитай сумму, раздели на длину с приведением `as f64`.
6. Добавь тесты границ.
7. Запусти:

   ```bash
   cargo test -p lab01-toolkit
   cargo clippy -p lab01-toolkit -- -D warnings
   cargo fmt
   ```

8. Убедись, что ignored-тестов больше нет.

---

## Дополнительная часть

Выбери одно или несколько:

- реализуй `min(&[f64]) -> Option<f64>` и `max(&[f64]) -> Option<f64>`;
- сделай CLI `src/bin/toolkit.rs` с командами `prime N`, `palindrome TEXT`, `average 1 2 3`; используй `std::env::args`, без внешних крейтов;
- перепиши тесты простых чисел как таблицу случаев в одном тесте с циклом;
- добавь функцию `is_perfect_square(i64) -> bool` без `sqrt` и протестируй границы.

CLI не должен ломать библиотечные тесты. Запуск: `cargo run -p lab01-toolkit --bin toolkit -- prime 97`.

---

## Критерии приемки

- [ ] `cargo test -p lab01-toolkit` проходит;
- [ ] нет `#[ignore]` в тестах лабораторной;
- [ ] есть собственные тесты границ;
- [ ] `cargo clippy -p lab01-toolkit -- -D warnings` без предупреждений;
- [ ] `cargo fmt --check` проходит;
- [ ] сигнатуры не изменены;
- [ ] библиотечные функции не печатают и не паникуют на обычных данных;
- [ ] входной срез в `average` не изменяется.

---

## На защите

Ответь без подсказок:

1. Чем `i64` отличается от `usize`? Почему длина среза — `usize`?
2. Почему `average` возвращает `Option<f64>`, а не `f64`? Что было бы в Java?
3. Чем `&str` отличается от `String`? Почему параметр — `&str`?
4. Почему `number % 2 == 0` работает для отрицательных чисел?
5. Почему `divisor * divisor <= number` может переполниться и как это исправить?
6. Зачем `#[cfg(test)]` у модуля тестов?
7. Что делает `todo!()` и почему код с ним компилируется?
8. Как бы ты реализовал `is_palindrome` без `chars().rev()`?

---

## Подсказки (открывать после попытки)

<details>
<summary>is_prime</summary>

```rust
pub fn is_prime(number: i64) -> bool {
    if number < 2 {
        return false;
    }
    let mut divisor = 2;
    while divisor <= number / divisor {
        if number % divisor == 0 {
            return false;
        }
        divisor += 1;
    }
    true
}
```

</details>

<details>
<summary>is_palindrome</summary>

```rust
pub fn is_palindrome(text: &str) -> bool {
    text.chars().eq(text.chars().rev())
}
```

</details>

<details>
<summary>average</summary>

```rust
pub fn average(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let sum: f64 = values.iter().sum();
    Some(sum / values.len() as f64)
}
```

`values.iter().sum()` — итератор; если хочешь строго в рамках языка до lab05, напиши цикл `for &x in values`.

</details>
