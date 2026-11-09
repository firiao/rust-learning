# Лаба 24. async и tokio

**Время:** ~20 минут
**Тема:** `async fn`, `.await`, runtime, `#[tokio::test]`

> **Языковой бюджет:** только то, что объяснено в лекциях 1–24. Полная таблица — [BUDGET.md](../BUDGET.md).

## Цель

Почувствовать две вещи: async-функция ничего не делает без `.await`, и из обычной функции
async-код запускают через runtime.

## Крейт уже создан

Открывай `practice/l24-async` — три `todo!()` и три теста (два из них `#[tokio::test]`).

```bash
cargo test -p l24-async      # 3 красных
```

## Реализуй

**1. `async fn double_after(yield_ms, value) -> i64`** — подождать `yield_ms` миллисекунд
через `tokio::time::sleep(Duration::from_millis(yield_ms)).await` и вернуть `value * 2`.

**2. `async fn sum_all(values: Vec<i64>) -> i64`** — сумма.

**3. `fn run_blocking() -> i64`** — обычная функция, которая запускает async-код и возвращает
результат `double_after(1, 21)`.

Подсказка: `tokio::runtime::Runtime::new()` и `.block_on(...)`.

## Эксперимент

В тесте `doubles` убери `.await`:

```rust
assert_eq!(double_after(1, 21), 42);
```

Запиши в `NOTES.md` ошибку компилятора: она показывает, что `async fn` возвращает не `i64`,
а `Future`.

## Дополнительное задание — проектируешь сам

Напиши `async fn`-функцию, которая запускает две задачи через `tokio::join!` и складывает
результаты. Замерь время с `join!` и без — запиши в `NOTES.md`.

Запиши в `NOTES.md`, почему выбрал такую сигнатуру или такой тип.

## Критерии приёмки

- [ ] 3 теста зелёные;
- [ ] `#[tokio::test]` используется для async-тестов, обычный `#[test]` — для `run_blocking`;
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` записан вывод ошибки без `.await`.

## Подсказки

<details>
<summary>`double_after`</summary>

```rust
pub async fn double_after(yield_ms: u64, value: i64) -> i64 {
    tokio::time::sleep(Duration::from_millis(yield_ms)).await;
    value * 2
}
```

`sleep` — асинхронный: пока ждём, runtime может заняться другими задачами.

</details>

<details>
<summary>`sum_all`</summary>

```rust
pub async fn sum_all(values: Vec<i64>) -> i64 {
    values.iter().sum()
}
```

Функция `async` не обязана ничего ждать — это просто «может приостанавливаться».

</details>

<details>
<summary>`run_blocking`</summary>

```rust
pub fn run_blocking() -> i64 {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(double_after(1, 21))
}
```

`Runtime::new()` создаёт исполнителя, `block_on` доводит будущее до конца. Так async-код
вызывают из обычного `main` или из библиотеки, которая не хочет навязывать runtime.

</details>

<details>
<summary>Эксперимент</summary>

```text
error[E0308]: mismatched types
expected `i64`, found opaque type `impl Future<Output = i64>`
```

`async fn` — функция, возвращающая будущее. Без `.await` ты сравниваешь будущее с числом.

</details>

---

**Готово?** Дальше — [лекция 25: ndarray](../25-ndarray/lecture.md).
