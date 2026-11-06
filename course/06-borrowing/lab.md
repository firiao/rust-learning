# Лаба 6. Заимствование

**Время:** ~20 минут
**Тема:** `&str`, `&mut`, срезы, правила заимствования

## Цель

Написать функции, которые **не забирают** данные: только смотрят или меняют на месте.

## Крейт уже создан

Открывай `practice/l06-borrowing` — копировать ничего не нужно:

- `src/lib.rs` — заготовка: сигнатуры функций с `todo!()` и тесты;
- `Cargo.toml` — пакет `l06-borrowing`.

`todo!()` — макрос-заглушка: компилируется, но паникует при вызове. Поэтому сейчас тесты **красные**. Это не поломка, а задача: пока `todo!()` на месте, тест падает с `not yet implemented`.

```bash
cargo test -p l06-borrowing        # сейчас: failures
```

**Почему компилятор предупреждает `unused variable`.** Параметры функций пока не используются — тело заменено на `todo!()`. Это нормально: предупреждения исчезнут сами, как только реализуешь. `cargo test` они не мешают, а `clippy -D warnings` станет чистым **после** реализации.

## Реализуй

Пять функций с `todo!()` в `src/lib.rs`.

```bash
cargo test -p l06-borrowing
```

Подсказка по `first_word`: нужен **байтовый** индекс пробела, чтобы взять `&text[..i]`. Его даёт `char_indices()`.

## Эксперименты

Три блока, по одному. Для каждого запиши в `NOTES.md`:

- код ошибки;
- какие два заимствования встретились (для блока 1 и 2);
- как починить.

## Критерии приёмки

- [ ] 6 тестов зелёные;
- [ ] `clippy` и `fmt --check` чистые;
- [ ] `first_word` возвращает `&str` — нигде нет `.to_string()` или `String::from`;
- [ ] в `NOTES.md` разобраны три эксперимента.

## Подсказки

<details>
<summary>Реализации</summary>

```rust
pub fn first_word(text: &str) -> &str {
    for (i, ch) in text.char_indices() {
        if ch == ' ' {
            return &text[..i];
        }
    }
    text
}

pub fn count_vowels(text: &str) -> usize {
    const VOWELS: &str = "аеёиоуыэюяАЕЁИОУЫЭЮЯaeiouyAEIOUY";
    text.chars().filter(|c| VOWELS.contains(*c)).count()
}

pub fn push_bang(text: &mut String) {
    text.push('!');
}

pub fn sum_slice(values: &[i32]) -> i32 {
    values.iter().sum()
}

pub fn swap(a: &mut i32, b: &mut i32) {
    std::mem::swap(a, b);
}
```

`char_indices()` важен потому, что `&text[..i]` режет **байты**. `chars().enumerate()` дал бы индексы в символах — на кириллице это ошибка или паника.

</details>

<details>
<summary>`swap` без `std::mem::swap`</summary>

```rust
pub fn swap(a: &mut i32, b: &mut i32) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}
```

`*a = *b` — «запиши по адресу, на который смотрит `a`».

</details>

<details>
<summary>Эксперимент 1</summary>

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
  |
  |     let first = &v[0];
  |                  -- immutable borrow occurs here
  |     v.push(4);
  |       ^^^^^^ mutable borrow occurs here
  |     println!("{first}");
  |                - immutable borrow later used here
```

Починка: напечатать `first` до `push` или скопировать значение (`let first = v[0];`).

</details>

<details>
<summary>Эксперимент 2</summary>

```text
error[E0499]: cannot borrow `s` as mutable more than once at a time
```

Два `&mut` одновременно запрещены.

</details>

<details>
<summary>Эксперимент 3</summary>

```text
error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable
```

`&mut` не делает переменную изменяемой — нужен `let mut s`.

</details>

---

**Готово?** Дальше — [лекция 7: структуры](../07-structs/lecture.md).
