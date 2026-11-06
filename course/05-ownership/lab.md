# Лаба 5. Владение

**Время:** ~20 минут
**Тема:** move, `Copy`, `clone`, передача владения в функцию

## Цель

Прочувствовать move руками: не прочитать про него, а увидеть, как компилятор отбирает переменную.

## Крейт уже создан

Открывай `practice/l05-ownership` — копировать ничего не нужно:

- `src/lib.rs` — заготовка: сигнатуры функций с `todo!()` и тесты;
- `Cargo.toml` — пакет `l05-ownership`.

`todo!()` — макрос-заглушка: компилируется, но паникует при вызове. Поэтому сейчас тесты **красные**. Это не поломка, а задача: пока `todo!()` на месте, тест падает с `not yet implemented`.

```bash
cargo test -p l05-ownership        # сейчас: failures
```

**Почему компилятор предупреждает `unused variable`.** Параметры функций пока не используются — тело заменено на `todo!()`. Это нормально: предупреждения исчезнут сами, как только реализуешь. `cargo test` они не мешают, а `clippy -D warnings` станет чистым **после** реализации.

## Реализуй

Все четыре функции — с `todo!()` в `src/lib.rs`. Обрати внимание: `append_bang` принимает `String` (забирает владение) и возвращает `String` (отдаёт обратно).

```bash
cargo test -p l05-ownership
```

## Эксперименты

Главная часть лабы. По одному блоку: раскомментировал → `cargo check` → прочитал → закомментировал.

1. **Блок 1** — move при присваивании: какой код ошибки?
2. **Блок 2** — move при передаче в функцию: чем сообщение отличается от блока 1?
3. **Блок 3** — `Copy`: ошибки **нет**. Объясни, почему.
4. **Блок 4** — `clone`: ошибки нет. Какая цена?

## Критерии приёмки

- [ ] 5 тестов зелёные;
- [ ] `clippy` и `fmt --check` чистые;
- [ ] в `NOTES.md` разобраны все 4 эксперимента;
- [ ] можешь объяснить, почему `text = append_bang(text)` внутри теста работает.

## Подсказки

<details>
<summary>Реализации</summary>

```rust
pub fn append_bang(mut text: String) -> String {
    text.push('!');
    text
}

pub fn total(values: Vec<i32>) -> i32 {
    values.iter().sum()
}

pub fn longest_word_owned(text: &str) -> String {
    let mut best = String::new();
    let mut best_len = 0;
    for word in text.split_whitespace() {
        let len = word.chars().count();
        if len > best_len {
            best_len = len;
            best = word.to_string();
        }
    }
    best
}

pub fn double(n: i32) -> i32 {
    n * 2
}
```

`longest_word_owned` принимает `&str` (не забирает владение), но возвращает `String` — потому что вернуть ссылку на своё временное значение нельзя.

</details>

<details>
<summary>Эксперимент 1: move при присваивании</summary>

```text
error[E0382]: borrow of moved value: `s1`
  |         - move occurs because `s1` has type `String`,
  |           which does not implement the `Copy` trait
  |     let s2 = s1;
  |             - value moved here
  |     println!("{s1}");
  |               ^ value borrowed here after move
help: consider cloning the value ...
```

</details>

<details>
<summary>Эксперимент 2: move в функцию</summary>

Та же ошибка `E0382`, но место перемещения — **вызов функции**: `value moved here` указывает на `take(s);`.

Вывод: передача в функцию — такое же перемещение, как присваивание.

</details>

<details>
<summary>Эксперимент 3: Copy</summary>

Ошибки нет, потому что `i32: Copy`. `let b = a` копирует 4 байта, обе переменные независимы.

То же с `f64`, `bool`, `char`, кортежами из них.

</details>

<details>
<summary>Эксперимент 4: clone</summary>

Ошибки нет: `clone()` создаёт **второй** `String` со своей копией байтов в куче. Цена — выделение памяти и копирование.

</details>

---

**Готово?** Дальше — [лекция 6: заимствование](../06-borrowing/lecture.md).
