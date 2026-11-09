# Лаба 11. Трейты

**Время:** ~20 минут
**Тема:** свой трейт, `impl` для нескольких типов, методы по умолчанию, `dyn`

> **Языковой бюджет:** только то, что объяснено в лекциях 1–11. Полная таблица — [BUDGET.md](../BUDGET.md).

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

**4. `describe_all(shapes: &[&dyn Shape]) -> String`** — описания каждой фигуры, по одному на строку (разделитель `\n`).

Обрати внимание на тип параметра: `&[&dyn Shape]`. Это срез **ссылок на трейт-объекты** —
поэтому в нём могут лежать и `Circle`, и `Square` одновременно. `Vec` появится в лекции 13,
поэтому здесь хватает массива/среза.

## Эксперимент

Попробуй поменять сигнатуру на `&[&Circle]` и вызвать её с массивом из круга и квадрата:
`[&c as &dyn Shape, &s as &dyn Shape]`.

Запиши в `NOTES.md`, почему не компилируется и что это говорит про `dyn`.

## Дополнительное задание — проектируешь сам

Напиши **свой** трейт с двумя методами (один — с реализацией по умолчанию) и реализуй
его для двух своих типов. Тест напиши сам.

**Реши сам:** какие методы обязательные, какой можно дать по умолчанию, что принимает `self`.

Запиши в `NOTES.md`, почему выбрал такую сигнатуру или такой тип.

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
    let mut total = 0.0;
    for shape in shapes {
        total += shape.area();
    }
    total
}

pub fn describe_all(shapes: &[&dyn Shape]) -> String {
    let mut all = String::new();
    for shape in shapes {
        if !all.is_empty() {
            all.push('\n');
        }
        all.push_str(&shape.describe());
    }
    all
}
```



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
