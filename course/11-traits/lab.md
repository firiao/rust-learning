# Лаба 11. Трейты

**Время:** ~20 минут
**Тема:** свой трейт, `impl` для нескольких типов, методы по умолчанию, `dyn`

## Цель

Один трейт, две реализации и один список, в котором лежат **разные типы**.

## Крейт уже создан

Открывай `practice/l11-traits` — трейт `Shape` уже описан, `todo!()` стоят в четырёх местах.

```bash
cargo test -p l11-traits     # 4 красных
```

## Реализуй

**1. `impl Shape for Circle`** — площадь круга: `PI * r * r`.
**2. `impl Shape for Square`** — площадь квадрата: `side * side`.

Метод `describe` **не трогай** — он уже есть в трейте как метод по умолчанию. Тест
`default_method_works` проверяет, что ты его не переписал.

**3. `total_area(shapes: &[&dyn Shape]) -> f64`** — сумма площадей.

**4. `describe_all(shapes: &[&dyn Shape]) -> Vec<String>`** — описания каждой фигуры.

Обрати внимание на тип параметра: `&[&dyn Shape]`. Это список **ссылок на трейт-объекты** —
поэтому в нём могут лежать и `Circle`, и `Square` одновременно.

## Эксперимент

Попробуй поменять сигнатуру на `&[&Circle]` и вызвать с `vec![&c, &s]` (круг и квадрат).

Запиши в `NOTES.md`, почему не компилируется и что это говорит про `dyn`.

## Критерии приёмки

- [ ] 4 теста зелёные;
- [ ] `describe` нигде не переопределён;
- [ ] `total_area` принимает именно `&[&dyn Shape]`;
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` объяснена разница между `dyn Shape` и `Shape` в сигнатуре.

## Подсказки

<details>
<summary>Реализации</summary>

```rust
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn name(&self) -> &'static str {
        "круг"
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn name(&self) -> &'static str {
        "квадрат"
    }
}

pub fn total_area(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

pub fn describe_all(shapes: &[&dyn Shape]) -> Vec<String> {
    shapes.iter().map(|s| s.describe()).collect()
}
```

В `map` замыкание получает `&&dyn Shape`, но метод вызывается через авторазыменование.

</details>

<details>
<summary>Почему `&'static str`</summary>

`name` возвращает литерал `"круг"`. Литералы вшиты в программу и живут всё её время; метка `'static`
означает именно это. Подробно про время жизни — в лекции 17.

</details>

<details>
<summary>Эксперимент</summary>

```text
error[E0308]: mismatched types
expected `&Square`, found `&Circle`
```

`&[&Circle]` — список **одного** типа. Круг и квадрат в него не помещаются, даже если оба умеют `Shape`.

`dyn Shape` — «какой-то тип, умеющий Shape». Именно поэтому в одном `Vec` могут лежать разные фигуры.

</details>

---

**Готово?** Дальше — [лекция 12: обобщения](../12-generics/lecture.md).
