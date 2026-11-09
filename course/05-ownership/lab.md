# Лаба 5. Владение

**Время:** ~20 минут
**Тема:** move, `Copy`, `clone`, передача владения в функцию

> **Языковой бюджет:** лекции 1–5. `Vec` и ссылки `&`/`&mut` будут в лекциях 6 и 13 —
> здесь их нет, работаем с `String` по значению.

## Цель

Прочувствовать move руками: не прочитать про него, а увидеть, как компилятор отбирает переменную.

## Крейт уже создан

Открывай `practice/l05-ownership` — четыре `todo!()` и четыре теста.

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

## Что важно понять без запуска ошибок

Лекция 5 показывает три ошибки компилятора (`E0382` при присваивании, `E0382` при передаче
в функцию, `E0596`). Воспроизводить их руками не нужно — они уже разобраны в тексте.

Вместо этого объясни **своими словами** в `NOTES.md`:

1. почему `let s2 = s1;` после себя оставляет `s1` недоступной;
2. почему передача `String` в функцию — тоже перемещение;
3. почему для `i32` ничего не ломается (что такое `Copy`);
4. зачем нужен `clone()`, если он дорогой.

## Критерии приёмки

- [ ] 4 теста зелёные;
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` есть ответы на 4 вопроса выше;
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

---

**Готово?** Дальше — [лекция 6: заимствование](../06-borrowing/lecture.md).
