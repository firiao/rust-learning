# Лаба 5. Владение

**Время:** ~20 минут
**Тема:** move, `Copy`, `clone`, передача владения в функцию

> **Языковой бюджет:** лекции 1–5. `Vec` и ссылки `&`/`&mut` будут в лекциях 6 и 13 —
> здесь их нет, работаем с `String` по значению.

## Цель

Прочувствовать move руками: не прочитать про него, а увидеть, как компилятор отбирает переменную.

## Крейт уже создан

Открывай `practice/l05-ownership` — четыре `todo!()` и четыре теста плюс блок экспериментов.

```bash
cargo test -p l05-ownership   # 4 красных
```

## Реализуй

**1. `append_bang(text: String) -> String`** — дописать `!` и вернуть владение.
Параметр уже `mut`, потому что внутри будешь менять.

**2. `append_twice(text: String) -> String`** — дописать **два** `!`.

**3. `double(n: i32) -> i32`** — `i32` это `Copy`, поэтому вызывающий переменную не теряет.

**4. `longest_word_owned(text: &str) -> String`** — самое длинное слово. Здесь уже можно
`for` и `if` (лекция 4). Возвращать надо `String` — новую строку, а не срез (срезы будут
в следующей лекции).

## Эксперименты

Эксперименты лежат отдельными файлами в `practice/l05-ownership/examples/` и **не мешают тестам**:
cargo не собирает их без флага `--features experiments`.

| Файл | Команда | Что ожидать |
|---|---|---|
| `e1_move_assign.rs` | `cargo check -p l05-ownership --example e1_move_assign --features experiments` | `E0382` |
| `e2_move_into_fn.rs` | `cargo check -p l05-ownership --example e2_move_into_fn --features experiments` | `E0382` |
| `e3_copy.rs` | `cargo run -p l05-ownership --example e3_copy --features experiments` | ошибки нет |
| `e4_clone.rs` | `cargo run -p l05-ownership --example e4_clone --features experiments` | ошибки нет |

Править и комментировать ничего не надо: файлы уже готовы, ошибки в них намеренные.
Запиши в `NOTES.md`, что показал каждый.

## Критерии приёмки

- [ ] 4 теста зелёные;
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` разобраны все 4 эксперимента;
- [ ] можешь объяснить, почему `text = append_bang(text)` работает.

## Подсказки

<details>
<summary>`append_bang` и `append_twice`</summary>

```rust
pub fn append_bang(mut text: String) -> String {
    text.push('!');
    text
}

pub fn append_twice(mut text: String) -> String {
    text.push('!');
    text.push('!');
    text
}
```

`text.push('!')` — метод добавляет один символ. Он уже был в лекции 5.

</details>

<details>
<summary>`longest_word_owned`</summary>

```rust
let mut best = String::from("");
let mut best_len = 0;
for word in text.split_whitespace() {
    let len = word.chars().count();
    if len > best_len {
        best_len = len;
        best = String::from(word);
    }
}
best
```

`split_whitespace()` и `chars().count()` встречались в лекциях 2 и 5; `for` и `if` — в лекции 4.
`String::from(word)` превращает `&str` в собственную строку.

</details>

<details>
<summary>Эксперименты 1 и 2</summary>

**1:** `error[E0382]: borrow of moved value: 's1'` — `let s2 = s1;` переместил владение.

**2:** та же `E0382`, но помечено `value moved here` на вызове `take(s);`.
Передача в функцию — тоже перемещение.

</details>

<details>
<summary>Эксперименты 3 и 4</summary>

**3:** ошибки нет — `i32` реализует `Copy`, `let b = a` копирует число.

**4:** ошибки нет — `clone()` создаёт вторую `String` с собственной копией байтов в куче.

</details>

---

**Готово?** Дальше — [лекция 6: заимствование](../06-borrowing/lecture.md).
