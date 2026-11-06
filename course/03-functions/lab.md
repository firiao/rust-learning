# Лаба 3. Функции и возврат

**Время:** ~15 минут
**Тема:** последнее выражение, `return`, вызов функций, `()`

## Цель

Три задачи, где главное — не алгоритм, а правильный возврат значения.

## Крейт уже создан

Открывай `practice/l03-functions` — копировать ничего не нужно:

- `src/lib.rs` — заготовка: сигнатуры функций с `todo!()` и тесты;
- `Cargo.toml` — пакет `l03-functions`.

`todo!()` — макрос-заглушка: компилируется, но паникует при вызове. Поэтому сейчас тесты **красные**. Это не поломка, а задача: пока `todo!()` на месте, тест падает с `not yet implemented`.

```bash
cargo test -p l03-functions        # сейчас: failures
```

**Почему компилятор предупреждает `unused variable`.** Параметры функций пока не используются — тело заменено на `todo!()`. Это нормально: предупреждения исчезнут сами, как только реализуешь. `cargo test` они не мешают, а `clippy -D warnings` станет чистым **после** реализации.

## Реализуй

Три функции и одна печатающая — все с `todo!()` в `src/lib.rs`.

```bash
cargo test -p l03-functions
```

## Эксперимент с `return`

Перепиши `fib` так, чтобы каждая ветка заканчивалась `return`:

```rust
fn fib(n: u32) -> u64 {
    if n < 2 {
        return n as u64;
    }
    // ...
}
```

Работает. А теперь попробуй убрать `;` в конце одной из веток — и объясни, почему это ломает функцию.

## Критерии приёмки

- [ ] 4 теста зелёные;
- [ ] `clippy` и `fmt --check` чистые;
- [ ] `print_and_count` **и печатает, и возвращает** (`println!` + `len()`);
- [ ] `fizzbuzz` не использует `return` — только выражение;
- [ ] в `NOTES.md` записано, что происходит без `;` в конце ветки.

## Подсказки

<details>
<summary>`fizzbuzz` через `if` как выражение</summary>

```rust
if n.is_multiple_of(15) {
    "FizzBuzz".to_string()
} else if n.is_multiple_of(3) {
    "Fizz".to_string()
} else if n.is_multiple_of(5) {
    "Buzz".to_string()
} else {
    n.to_string()
}
```

Никакого `let mut result;`. Порядок важен: 15 проверяем первым, иначе 3 перехватит.

**Про `is_multiple_of`.** Привычное `n % 15 == 0` тоже работает, но `clippy` предложит замену — и при `-D warnings` это станет ошибкой. `n.is_multiple_of(15)` читается как «n кратно 15» и делает ровно то же самое. Используй его, чтобы лаба проходила чистый `clippy`.

</details>

<details>
<summary>`collatz_steps` через `loop`</summary>

```rust
let mut n = start;
let mut steps = 0;
loop {
    if n == 1 {
        break;
    }
    n = if n.is_multiple_of(2) { n / 2 } else { 3 * n + 1 };
    steps += 1;
}
steps
```

Здесь `is_multiple_of(2)` — вместо `n % 2 == 0`, по той же причине: иначе `clippy` будет недоволен.

</details>

<details>
<summary>`fib` итеративно</summary>

```rust
if n < 2 {
    return n as u64;
}
let mut prev: u64 = 0;
let mut curr: u64 = 1;
for _ in 1..n {
    let next = prev + curr;
    prev = curr;
    curr = next;
}
curr
```

</details>

<details>
<summary>`print_and_count`</summary>

```rust
println!("{text}");
text.chars().count()
```

Здесь две роли функции: печать — это побочный эффект, возврат — значение. Обрати внимание: `println!` возвращает `()`, а последняя строка даёт `usize`.

</details>

<details>
<summary>Почему без `;` ломается</summary>

Последнее выражение в функции — её значение. Если поставить `;`, выражение превращается в «посчитать и выбросить», и функция начнёт возвращать `()` — компилятор скажет `E0308: mismatched types`.

Это особенно коварно внутри `if`-ветки: `return x;` — правильно (это оператор), а `x;` в ветке, которая должна дать значение, — ошибка.

</details>

---

**Готово?** Дальше — [лекция 4: условия и циклы](../04-conditions-loops/lecture.md).
