# Лабораторная 02. Ownership, borrowing, slices

## Цель

Понять, как Rust управляет памятью без сборщика мусора: move, `Copy`, `Clone`, заимствование, изменяемые ссылки, срезы и время жизни ссылок.

Это **самая важная** лабораторная курса. Если она не понята, дальше будет больно.

## Что нужно знать до начала

- lab01;
- `String`, `&str`, `Vec<T>`, срезы;
- функции и тесты;
- желательно прочитать [`docs/03-ownership.md`](../docs/03-ownership.md).

## Языковой бюджет

Разрешено: `&T`, `&mut T`, `String`, `&str`, `Vec<T>`, `&[T]`, `Option`, `match`/`if let`, lifetimes `<'a>`.

Запрещено: `Rc`, `RefCell`, `Arc`, `Mutex`, `unsafe`, внешние крейты. Если хочется `clone()` — сначала объясни, почему без него нельзя.

## Файлы

- Условие: этот файл.
- Код: [`../labs/lab02-ownership/src/lib.rs`](../labs/lab02-ownership/src/lib.rs).
- Запуск: `cargo test -p lab02-ownership`.

## Что уже есть

Реализованы:

- `count_words(&str) -> usize`;
- `shout_in_place(&mut String)`;
- `sum(&[i32]) -> i32`;
- `consume_and_len(String) -> usize`;
- `greet(&str) -> String`.

Нужно реализовать:

- `first_word(&str) -> &str`;
- `longest<'a>(&'a str, &'a str) -> &'a str`;
- `split_even_odd(&[i32]) -> (Vec<i32>, Vec<i32>)`.

Часть тестов помечена `#[ignore]`.

---

## Подготовительная часть

1. Прочитай [`docs/03-ownership.md`](../docs/03-ownership.md), разделы «Move», «Borrowing», «`String` vs `&str`», «Lifetimes».

2. Запусти:

   ```bash
   cargo test -p lab02-ownership
   ```

3. Объясни себе (письменно в `NOTES.md` или вслух), почему этот код не компилируется:

   ```rust
   let s1 = String::from("hello");
   let s2 = s1;
   println!("{s1}");
   ```

4. Почему этот компилируется?

   ```rust
   let x = 5;
   let y = x;
   println!("{x} {y}");
   ```

5. Почему этот не компилируется и как исправить?

   ```rust
   let mut v = vec![1, 2, 3];
   let first = &v[0];
   v.push(4);
   println!("{first}");
   ```

6. Попробуй в отдельном файле/проекте закомментировать `println!("{first}")` и посмотри, что изменится. Объясни через NLL.

---

## Обязательное задание

Реализуй функции, не меняя сигнатуры:

```rust
pub fn first_word(text: &str) -> &str;
pub fn longest<'a>(left: &'a str, right: &'a str) -> &'a str;
pub fn split_even_odd(values: &[i32]) -> (Vec<i32>, Vec<i32>);
```

### Контракт

1. `first_word` возвращает первое слово, разделённое пробельными символами.
2. Ведущие пробелы игнорируются. Если слов нет — возвращается пустая строка `""`.
3. `first_word` **не выделяет новую память** и возвращает срез входной строки. Результат живёт столько же, сколько вход.
4. `longest` возвращает более длинную строку. Если длины равны — левую.
5. `longest` не выделяет память; результат — срез одного из входов.
6. `split_even_odd` возвращает два вектора: чётные и нечётные числа, сохраняя исходный порядок.
7. `split_even_odd` не изменяет входной срез. Пустой вход → два пустых вектора.
8. `count_words`: пустая строка → 0; несколько пробелов подряд не создают пустые слова.
9. `shout_in_place` переводит строку в верхний регистр **на месте** (меняя переданный `String`), поддерживает кириллицу.
10. `consume_and_len` забирает владение строкой и возвращает её длину в байтах. После вызова исходная переменная недоступна — это ожидаемо.
11. `greet` не забирает владение именем, возвращает новую строку.

### Тесты

Сними `#[ignore]` и добавь тесты:

- `first_word`: обычная строка, ведущие пробелы, только пробелы, пустая строка, один символ, кириллица;
- `longest`: левая длиннее, правая длиннее, равные длины, пустые строки, Unicode;
- `split_even_odd`: пустой срез, все чётные, все нечётные, смешанные, отрицательные, порядок;
- `shout_in_place`: латиница, кириллица, пустая строка, уже верхний регистр;
- `consume_and_len`: длина в байтах (помни, что кириллица в UTF-8 занимает 2 байта на символ);
- `greet`: пустое имя, обычное имя.

Не меняй существующие тесты без причины.

---

## Пошаговый план

1. Реализуй `first_word` через `split_whitespace().next().unwrap_or("")`. Проверь, что возвращается именно срез, а не новый `String`.
2. Реализуй `longest` с явным lifetime `'a`. Сравни `left.len()` и `right.len()`.
3. Реализуй `split_even_odd` двумя векторами и циклом `for &x in values`.
4. Сними `#[ignore]`, добавь тесты границ.
5. Запусти:

   ```bash
   cargo test -p lab02-ownership
   cargo clippy -p lab02-ownership -- -D warnings
   cargo fmt
   ```

6. Проверь себя: в коде нет `clone()` в реализациях (в тестах можно).

---

## Эксперименты с компилятором

Это часть лабораторной. Создай временный файл (не коммить) и по очереди проверь:

1. Верни ссылку на локальную `String`:

   ```rust
   fn bad() -> &str {
       let s = String::from("hello");
       &s
   }
   ```

   Прочитай ошибку E0515/E0597. Как исправить?

2. Две `&mut` одновременно:

   ```rust
   let mut s = String::from("hi");
   let a = &mut s;
   let b = &mut s;
   a.push('!');
   b.push('?');
   ```

   Прочитай E0499.

3. Чтение и запись одновременно:

   ```rust
   let mut v = vec![1, 2, 3];
   let first = &v[0];
   v.push(4);
   println!("{first}");
   ```

   Прочитай E0502. Переставь `println!` до `push` — компилируется? Почему?

4. Move в функцию:

   ```rust
   fn take(s: String) {}
   let s = String::from("hi");
   take(s);
   println!("{s}");
   ```

   Прочитай E0382. Исправь через `&str`.

Запиши в `NOTES.md` по одному предложению на каждую ошибку: какое правило нарушено.

---

## Дополнительная часть

Выбери одно или несколько:

- `initials(&str) -> String` — первые буквы слов в верхнем регистре (`"anna maria"` → `"AM"`);
- `longest_word(&str) -> &str` — самое длинное слово, при равенстве — первое;
- `truncate_to_first_word(&mut String) -> usize` — оставить в строке только первое слово, вернуть число удалённых байт;
- `split_at_word(&str) -> (&str, &str)` — вернуть первое слово и остаток строки;
- напиши функцию, которая принимает `&mut Vec<i32>` и удаляет все чётные числа **без** создания нового вектора; объясни, почему это сложнее, чем вернуть новый `Vec`.

---

## Критерии приемки

- [ ] `cargo test -p lab02-ownership` проходит;
- [ ] нет `#[ignore]` в тестах лабораторной;
- [ ] `first_word` и `longest` не выделяют память и возвращают срезы;
- [ ] `split_even_odd` не изменяет вход;
- [ ] нет `clone()` в реализациях;
- [ ] `cargo clippy -p lab02-ownership -- -D warnings` без предупреждений;
- [ ] `cargo fmt --check` проходит;
- [ ] в `NOTES.md` объяснены четыре эксперимента с компилятором.

---

## На защите

1. Чем move отличается от `Copy`? Приведи пример типа `Copy` и типа не-`Copy`.
2. Зачем нужен `Clone`, если есть `Copy`?
3. Сколько `&T` и `&mut T` может существовать одновременно? Почему?
4. Что такое NLL и как он помогает в обычном коде?
5. Почему `first_word` компилируется без явного lifetime, а `longest` — нет?
6. Что означает `'a` в `longest<'a>`?
7. Почему `&str` — хороший параметр функции, а `String` — хороший возвращаемый тип?
8. Что произойдёт, если вернуть ссылку на локальную `String`?
9. Почему `consume_and_len` после вызова делает исходную строку недоступной?
10. Как бы ты объяснил borrow checker человеку, который знает только Python?

---

## Подсказки (открывать после попытки)

<details>
<summary>first_word</summary>

```rust
pub fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}
```

</details>

<details>
<summary>longest</summary>

```rust
pub fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() { left } else { right }
}
```

</details>

<details>
<summary>split_even_odd</summary>

```rust
pub fn split_even_odd(values: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut evens = Vec::new();
    let mut odds = Vec::new();

    for &value in values {
        if value % 2 == 0 {
            evens.push(value);
        } else {
            odds.push(value);
        }
    }

    (evens, odds)
}
```

</details>
