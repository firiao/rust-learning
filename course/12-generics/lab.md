# Лаба 12. Обобщения

**Время:** ~20 минут
**Тема:** `<T>`, ограничения, обобщённая структура, `impl Trait`, `where`

> **Языковой бюджет:** только то, что объяснено в лекциях 1–12. Полная таблица — [BUDGET.md](../BUDGET.md).

## Цель

Написать код, который работает для `i32`, `f64`, `char` и любого типа с нужными свойствами.

## Крейт уже создан

Открывай `practice/l12-generics` — пять `todo!()` в `src/lib.rs` и пять тестов.

```bash
cargo test -p l12-generics   # 5 красных
```

## Реализуй

**1. `largest<T: PartialOrd + Copy>(values: &[T]) -> Option<T>`** — наибольший элемент.
Пустой срез должен давать `None` (а не панику): подсказка — `values.first()?`.

**2. `Pair::new(first, second)`** — создать пару. `impl<T>` без ограничений: сравнивать не нужно.

**3. `Pair::larger(&self) -> &T`** — **ссылка** на большее значение. `impl<T: PartialOrd>` —
ограничение нужно, потому что внутри сравнение.

**4. `print_all<T>(items: &[T]) where T: Debug`** — печатает элементы через `{:?}`.
Ограничение уже выписано через `where`, трогать его не надо.

**5. `make_pair<A, B>(first, second) -> (A, B)`** — два **разных** параметра типа.

## Эксперимент

В `largest` убери ограничение `Copy`, оставив только `PartialOrd`.

Запиши в `NOTES.md`: на какой строке компилятор ругается и почему `Copy` понадобился.

## Дополнительное задание — проектируешь сам

Напиши обобщённую функцию `min_max`, которая за один проход возвращает минимум и максимум
среза. **Сигнатуру выбери сам:** какие ограничения нужны `T`, что вернуть для пустого среза.

Тест напиши сам.

Запиши в `NOTES.md`, почему выбрал такую сигнатуру или такой тип.

## Критерии приёмки

- [ ] 5 тестов зелёные;
- [ ] `largest` работает для `i32`, `f64`, `char` без дублирования кода;
- [ ] `larger` возвращает `&T`, а не `T` (иначе не скомпилируется для `&str`);
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` разобран эксперимент без `Copy`.

## Подсказки

<details>
<summary>`largest`</summary>

```rust
let mut best = *values.first()?;
for &v in &values[1..] {
    if v > best {
        best = v;
    }
}
Some(best)
```

`values.first()?` — `Option<&T>`; `?` в функции, возвращающей `Option`, даёт ранний `None`.
`*` разыменовывает ссылку.

`for &v in &values[1..]` — идём по ссылкам, но `&v` в шаблоне копирует значение (для `Copy`-типов).

</details>

<details>
<summary>`Pair`</summary>

```rust
impl<T> Pair<T> {
    pub fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
}

impl<T: PartialOrd> Pair<T> {
    pub fn larger(&self) -> &T {
        if self.first > self.second {
            &self.first
        } else {
            &self.second
        }
    }
}
```

Два отдельных `impl`: `new` не требует сравнения, `larger` требует. Это нормальная практика —
ограничения ставят только там, где они нужны.

</details>

<details>
<summary>`print_all`</summary>

```rust
for item in items {
    println!("{item:?}");
}
```

`{item:?}` требует `Debug` — поэтому ограничение обязательно.

</details>

<details>
<summary>Эксперимент</summary>

Без `Copy` компилятор укажет на `let mut best = *values.first()?;`:

```text
error[E0507]: cannot move out of a shared reference
```

Разыменование `*` пытается **забрать** значение из-под ссылки, а срез чужой. `Copy` разрешает
скопировать вместо перемещения. Альтернатива без `Copy` — вернуть `Option<&T>`, а не `Option<T>`.

</details>

---

**Готово?** Дальше — [лекция 13: коллекции](../13-collections/lecture.md).
