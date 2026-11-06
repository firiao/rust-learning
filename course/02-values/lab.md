# Лаба 2. Текст и числа

**Время:** ~15 минут
**Тема:** `String` vs `&str`, байты и символы, `char`

## Цель

Четыре функции обработки текста. Все тесты — на кириллице, чтобы сразу было видно, кто считает байты, а кто символы.

## Крейт уже создан

Открывай `practice/l02-text` — копировать ничего не нужно:

- `src/lib.rs` — заготовка: сигнатуры функций с `todo!()` и тесты;
- `Cargo.toml` — пакет `l02-text`.

`todo!()` — макрос-заглушка: компилируется, но паникует при вызове. Поэтому сейчас тесты **красные**. Это не поломка, а задача: пока `todo!()` на месте, тест падает с `not yet implemented`.

```bash
cargo test -p l02-text        # сейчас: failures
```

**Почему компилятор предупреждает `unused variable`.** Параметры функций пока не используются — тело заменено на `todo!()`. Это нормально: предупреждения исчезнут сами, как только реализуешь. `cargo test` они не мешают, а `clippy -D warnings` станет чистым **после** реализации.

## Реализуй

Все четыре функции — в `src/lib.rs`. Начни с `char_count`: она самая короткая и сразу показывает разницу между байтами и символами.

```bash
cargo test -p l02-text
```

## Эксперимент

Добавь в конец файла:

```rust
// #[test]
// fn experiment() {
//     let text = "привет";
//     let s = &text[0..3];
//     println!("{s}");
// }
```

Раскомментируй и запусти. Что произошло и почему? Запиши вывод в `NOTES.md`.

## Критерии приёмки

- [ ] 5 тестов зелёные;
- [ ] `clippy` и `fmt --check` чистые;
- [ ] `initials("")` возвращает пустую строку, а не `"."`;
- [ ] в `NOTES.md` объяснено, почему `char_count("привет") != "привет".len()`;
- [ ] в `NOTES.md` записан результат эксперимента со срезом `&text[0..3]`.

## Подсказки

<details>
<summary>`char_count`</summary>

```rust
text.chars().count()
```

Один вызов, цикл не нужен. `chars()` — итератор по символам, `count()` — количество.

</details>

<details>
<summary>`initials` — как не получить лишнюю точку</summary>

```rust
let mut parts: Vec<String> = Vec::new();
for word in full_name.split_whitespace() {
    if let Some(first) = word.chars().next() {
        parts.push(first.to_uppercase().to_string());
    }
}

if parts.is_empty() {
    String::new()
} else {
    format!("{}.", parts.join(". "))
}
```

`split_whitespace()` уже схлопывает пробелы, поэтому `"  мария   ивановна "` даст два слова. `to_uppercase()` у `char` возвращает итератор (не `char`), поэтому `.to_string()`.

</details>

<details>
<summary>`mask_digits`</summary>

```rust
let mut out = String::with_capacity(text.len());
for ch in text.chars() {
    if ch.is_ascii_digit() {
        out.push('*');
    } else {
        out.push(ch);
    }
}
out
```

</details>

<details>
<summary>`shout`</summary>

```rust
format!("{}!", text.to_uppercase())
```

`to_uppercase()` у `&str` возвращает `String` и корректно работает с кириллицей.

</details>

<details>
<summary>Эксперимент: что будет с `&text[0..3]`</summary>

Паника:

```text
thread 'main' panicked at: end byte index 3 is not a char boundary;
it is inside 'р' (bytes 2..4 of string)
```

Индекс 3 попадает внутрь буквы «р» (она занимает байты 2 и 3). Срез по байтам, который режет символ пополам, недопустим.

</details>

---

**Готово?** Дальше — [лекция 3: функции](../03-functions/lecture.md).
