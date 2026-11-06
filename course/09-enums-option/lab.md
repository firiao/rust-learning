# Лаба 9. Перечисления и `Option`

**Время:** ~20 минут
**Тема:** `enum` с данными, `match` по вариантам, `Option`, `if let`

## Цель

Свой `enum` с разнородными вариантами и четыре функции, которые возвращают `Option` вместо паники.

## Крейт уже создан

Открывай `practice/l09-enums` — `src/lib.rs` с `todo!()` и шестью тестами.

```bash
cargo test -p l09-enums      # 6 красных
```

## Реализуй

**1. `Shape::area(&self)`** — `match` по вариантам. Все три варианта перечисли явно, **без `_`**:
именно в этом смысл `enum`.

**2. `Shape::has_area(&self)`** — через `self.area() > 0.0`. Не дублируй формулу: если завтра
площадь круга посчитают иначе, `has_area` обновится сам.

**3. `safe_divide(a, b) -> Option<f64>`** — на ноль `None`.

**4. `first_even_index(values) -> Option<usize>`** — индекс первого чётного.

**5. `longest_word(text) -> Option<&str>`** — самое длинное слово или `None`, если текста нет.
Функция возвращает **ссылку** внутрь входной строки, без `to_string()`.

**6. `describe(value: Option<i32>) -> String`** — четыре случая через `match`.

## Эксперимент

Замени в `describe` первый рукав на `Some(n) => ...` и убери `None`:

```rust
match value {
    Some(n) => format!("есть число {n}"),
}
```

Запиши в `NOTES.md` текст ошибки. Это та самая исчерпывающесть.

## Критерии приёмки

- [ ] 6 тестов зелёные;
- [ ] нигде нет `unwrap()` и `panic!()`;
- [ ] `longest_word` не вызывает `to_string()`;
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` записана ошибка про непокрытый `None`.

## Подсказки

<details>
<summary>`area` и `has_area`</summary>

```rust
pub fn area(&self) -> f64 {
    match self {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
        Shape::Empty => 0.0,
    }
}

pub fn has_area(&self) -> bool {
    self.area() > 0.0
}
```

`match self`, где `self` — `&Shape`: паттерны автоматически работают по ссылке, `radius` внутри — `&f64`.

</details>

<details>
<summary>`longest_word`</summary>

```rust
let mut best: Option<&str> = None;
for word in text.split_whitespace() {
    if best.is_none_or(|b| word.chars().count() > b.chars().count()) {
        best = Some(word);
    }
}
best
```

`is_none_or` — «лучшего ещё нет **или** новый длиннее». Если твой компилятор её не знает, замени на
`best.map_or(true, |b| ...)`.

</details>

<details>
<summary>Про чётность и `clippy`</summary>

Привычное `*v % 2 == 0` работает, но `clippy` предложит `is_multiple_of`. Беда в том, что у `&i32`
метод `is_multiple_of` не находится (он для беззнаковых), а разыменовывать в цикле неудобно.

Поэтому в эталоне используется `v.rem_euclid(2) == 0` — «остаток по модулю, всегда неотрицательный».
Для отрицательных чисел это ещё и корректнее, чем `%`.

</details>

---

**Готово?** Дальше — [лекция 10: ошибки и `Result`](../10-errors/lecture.md).
