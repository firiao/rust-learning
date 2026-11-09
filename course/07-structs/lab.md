# Лаба 7. Структуры и методы

**Время:** ~15 минут
**Тема:** `struct`, `impl`, `&self`/`&mut self`, `Self`, `derive`

> **Языковой бюджет:** только то, что объяснено в лекциях 1–7. Полная таблица — [BUDGET.md](../BUDGET.md).

## Цель

Две структуры с методами: точка (только чтение) и прямоугольник (чтение и изменение).

## Крейт уже создан

Открывай `practice/l07-structs` — копировать ничего не нужно:

- `src/lib.rs` — структуры с `todo!()` и тесты;
- пакет называется `l07-structs`.

```bash
cargo test -p l07-structs     # 5 красных: стоят todo!()
```

**Почему компилятор предупреждает `unused variable`** — параметры не используются из-за `todo!()`.
Исчезнут, как только реализуешь.

## Реализуй

Шесть методов:

| Метод | Что делает | Вид `self` |
|---|---|---|
| `Point::new` | создать точку | нет `self` (ассоциированная) |
| `Point::distance_to` | расстояние до другой точки | `&self` |
| `Point::translate` | сдвинуть на месте | `&mut self` |
| `Rectangle::new` | создать прямоугольник | нет `self` |
| `Rectangle::area`, `perimeter`, `is_square` | измерения | `&self` |
| `Rectangle::scale` | увеличить обе стороны | `&mut self` |

`distance_to` — теорема Пифагора: `sqrt(dx² + dy²)`, где `.sqrt()` — метод `f64`.

## Эксперимент

Временного убери `mut` у `let mut r` в тесте `rectangle_scale`:

```rust
let r = Rectangle::new(2.0, 3.0);
r.scale(2.0);
```

Запиши в `NOTES.md`, какой код ошибки и что советует компилятор.

## Дополнительное задание — тип и сигнатуру проектируешь сам

Опиши структуру `Money` с полями `rubles: u32` и `kopecks: u8` и напиши метод
`in_kopecks(&self) -> u32` (сколько всего копеек).

**Сигнатуру и тело выбери сам.** Подумай:
- метод должен читать или менять? От этого зависит `&self` или `&mut self`;
- нужна ли проверка, что `kopecks < 100`? Если да — где её место: в конструкторе или в методе?

Напиши тест сам. В `NOTES.md` объясни, почему выбрал такой `self`.

## Критерии приёмки

- [ ] 5 тестов зелёные;
- [ ] `clippy -p l07-structs --all-targets -- -D warnings` чистый **после** реализации;
- [ ] `is_square` сравнивает через `abs() < 1e-9`, а не `==` (сравнение `f64` на равенство — источник багов);
- [ ] в `NOTES.md` разобран эксперимент с `&mut self` без `mut`.

## Подсказки

<details>
<summary>Реализации</summary>

```rust
impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    pub fn is_square(&self) -> bool {
        (self.width - self.height).abs() < 1e-9
    }

    pub fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}
```

</details>

<details>
<summary>Почему `is_square` не через `==`</summary>

`f64` — двоичная дробь, и `0.1 + 0.2 != 0.3` в ней. Для «равенства» вещественных сравнивают
с допуском: `(a - b).abs() < 1e-9`. Для целых значений это даёт тот же результат, а для
вычисленных — не подведёт.

</details>

<details>
<summary>Эксперимент</summary>

```text
error[E0596]: cannot borrow `r` as mutable, as it is not declared as mutable
help: consider changing this to be mutable
  |
  |     let mut r = Rectangle::new(2.0, 3.0);
  |         +++
```

`&mut self` — это **просьба** о праве менять. Право выдаётся при объявлении переменной.

</details>

---

**Готово?** Дальше — [лекция 8: `match`](../08-match/lecture.md).
