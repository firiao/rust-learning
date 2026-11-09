# Лаба 18. Потоки

**Время:** ~20 минут
**Тема:** `thread::scope`, `mpsc`, `Arc<Mutex>`

> **Языковой бюджет:** только то, что объяснено в лекциях 1–18. Полная таблица — [BUDGET.md](../BUDGET.md).

## Цель

Три сценария многопоточности: параллельные вычисления по срезу, передача результата через канал,
разделяемый счётчик.

## Крейт уже создан

Открывай `practice/l18-threads` — три `todo!()` и пять тестов.

```bash
cargo test -p l18-threads    # 5 красных
```

## Реализуй

**1. `parallel_sum(values, threads) -> i64`** — через `thread::scope` и `chunks`.

Граничные случаи обязательны: пустой срез, `threads = 0`, `threads = 1`. Обрати внимание,
что `chunks(0)` **паникует** — поэтому количество потоков надо ограничить.

**2. `sum_via_channel(n) -> u64`** — `thread::spawn` считает сумму `1..=n` и отправляет через `mpsc`.

**3. `increment_concurrently(counter, threads, times)`** — каждый из `threads` потоков делает
`times` инкрементов под `Mutex`.

## Эксперимент

В `increment_concurrently` вынеси `let mut guard = counter.lock().unwrap();` **за** внутренний цикл:

```rust
let mut guard = counter.lock().unwrap();
for _ in 0..times {
    *guard += 1;
}
```

Тест пройдёт, но станет медленнее. Запиши в `NOTES.md`, почему: что именно держит блокировку
и почему это плохо для параллелизма.

## Дополнительное задание — проектируешь сам

Напиши `parallel_count_char(&str, char) -> usize`: строка режется на куски, каждый считается
в своём потоке через `thread::scope`, результаты складываются.

**Реши сам:** как делить строку по **границам символов** (подсказка: байтовые индексы из лекции 6).

Запиши в `NOTES.md`, почему выбрал такую сигнатуру или такой тип.

## Критерии приёмки

- [ ] 5 тестов зелёные и стабильны при 3–5 повторных запусках;
- [ ] `parallel_sum` использует `thread::scope` и **не** копирует срез;
- [ ] `sum_via_channel` реально отправляет результат через канал;
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` разобран эксперимент с блокировкой в цикле.

## Подсказки

<details>
<summary>`parallel_sum`</summary>

```rust
if values.is_empty() || threads <= 1 {
    return values.iter().sum();
}
let threads = threads.min(values.len());
let chunk_size = values.len().div_ceil(threads);
thread::scope(|s| {
    let handles: Vec<_> = values
        .chunks(chunk_size)
        .map(|part| s.spawn(move || part.iter().sum::<i64>()))
        .collect();
    handles.into_iter().map(|h| h.join().unwrap()).sum()
})
```

`div_ceil` округляет вверх: 1000 элементов на 4 потока — по 250.

</details>

<details>
<summary>`sum_via_channel`</summary>

```rust
let (tx, rx) = mpsc::channel();
thread::spawn(move || {
    let total: u64 = (1..=n).sum();
    tx.send(total).unwrap();
});
rx.recv().unwrap()
```

`tx` перемещается в поток, поэтому внешний отправитель исчезает и `recv()` не повиснет.

</details>

<details>
<summary>`increment_concurrently`</summary>

```rust
thread::scope(|s| {
    for _ in 0..threads {
        let counter = Arc::clone(&counter);
        s.spawn(move || {
            for _ in 0..times {
                let mut guard = counter.lock().unwrap();
                *guard += 1;
            }
        });
    }
});
```

`Arc::clone` внутри цикла: каждый поток получает свою ручку. `guard` отпускается в конце каждой
итерации.

</details>

<details>
<summary>Эксперимент</summary>

Если держать `guard` на весь внутренний цикл, блокировка не отпускается между инкрементами.
Потоки выстраиваются в очередь и работают фактически последовательно — параллелизма нет,
а накладные расходы на синхронизацию остаются.

Правило: **держать блокировку как можно короче**.

</details>

---

**Готово?** Дальше — [лекция 19: Cargo глубже](../19-cargo-deep/lecture.md).
