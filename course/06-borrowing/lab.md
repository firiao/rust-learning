# Лаба 6. Заимствование

**Время:** ~20 минут
**Тема:** `&`, `&mut`, `*`, срезы, правила заимствования

> **Языковой бюджет:** лекции 1–6. Без `Vec`, без итераторов (`iter`, `map`, `filter`) —
> только циклы и индексы.

## Цель

Функции, которые **не забирают** данные: только смотрят, меняют на месте или возвращают срез.

## Крейт уже создан

Открывай `practice/l06-borrowing` — четыре `todo!()`, пять тестов и три эксперимента.

```bash
cargo test -p l06-borrowing   # 5 красных
```

## Реализуй

**1. `first_word(text: &str) -> &str`** — вернуть **срез** входной строки. Подсказка:
`char_indices()` из лекции 6 даёт пары `(байтовый индекс, символ)`.

**2. `push_bang(text: &mut String)`** — дописать `!`, не забирая владение.

**3. `sum_slice(values: &[i32]) -> i32`** — сумма среза. `for v in values` даёт `&i32`,
поэтому нужен `*v` (разыменование из лекции 6).

**4. `count_vowels(text: &str) -> usize`** — гласные. Работаем циклом по `chars()` и сравнениями
через `||`; `match` будет только в лекции 8.

## Эксперименты

По одному блоку: раскомментировал → `cargo check -p l06-borrowing` → прочитал → вернул.

1. `&` и `&mut` одновременно;
2. два `&mut`;
3. `&mut` у переменной без `mut`.

## Критерии приёмки

- [ ] 5 тестов зелёные;
- [ ] `first_word` возвращает `&str` — нигде нет `String::from` и `to_string`;
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` разобраны три эксперимента.

## Подсказки

<details>
<summary>`first_word`</summary>

```rust
for (i, ch) in text.char_indices() {
    if ch == ' ' {
        return &text[..i];
    }
}
text
```

`char_indices()` важен потому, что `&text[..i]` режет **байты**. Для кириллицы индекс в символах
дал бы неверный срез.

</details>

<details>
<summary>`sum_slice`</summary>

```rust
let mut total = 0;
for v in values {
    total += *v;
}
total
```

`values` — срез, элементы идут по ссылке. `*v` — разыменование: «достать значение по ссылке».

</details>

<details>
<summary>`count_vowels`</summary>

```rust
let mut count = 0;
for ch in text.chars() {
    if ch == 'а' || ch == 'е' || ch == 'ё' || ch == 'и' || ch == 'о' || ch == 'у'
        || ch == 'ы' || ch == 'э' || ch == 'ю' || ch == 'я'
        || ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' || ch == 'y'
    {
        count += 1;
    }
}
count
```

</details>

<details>
<summary>Эксперименты</summary>

**1:** `error[E0502]: cannot borrow 's' as mutable because it is also borrowed as immutable`.

**2:** `error[E0499]: cannot borrow 's' as mutable more than once at a time`.

**3:** `error[E0596]: cannot borrow 's' as mutable, as it is not declared as mutable`
+ `help: consider changing this to be mutable` → нужно `let mut s`.

</details>

---

**Готово?** Дальше — [лекция 7: структуры](../07-structs/lecture.md).
