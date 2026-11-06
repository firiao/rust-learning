# Лабораторная 07. Потоки и конкурентность

## Цель

Научиться безопасно работать с потоками: передавать владение в `move`-замыкания, использовать scoped threads, каналы, `Arc<Mutex<T>>` и понимать, почему Rust не даёт получить гонку данных.

## Что нужно знать до начала

- lab01–lab06;
- ownership, `move`, замыкания;
- `Result`/`Option`;
- трейты `Send`/`Sync` — пока на уровне идеи.

## Языковой бюджет

Разрешено: `std::thread`, `thread::scope`, `move`-замыкания, `mpsc::channel`, `Arc`, `Mutex`, `AtomicUsize` (доп.), `JoinHandle`.

Запрещено: `unsafe`, async, внешние крейты (кроме `rayon` в дополнительной части).

## Проект

```bash
cd ~/rust-learning
cargo new --lib labs/lab07-concurrency
```

Добавь в `members` корневого `Cargo.toml`:

```toml
"labs/lab07-concurrency",
```

Запуск:

```bash
cargo test -p lab07-concurrency
```

---

## Подготовительная часть

1. Запусти поток, который возвращает число, и забери результат через `join`:

   ```rust
   use std::thread;

   let handle = thread::spawn(|| 21 * 2);
   let result = handle.join().unwrap();
   assert_eq!(result, 42);
   ```

2. Передай `String` в поток через `move`. Объясни, почему без `move` не компилируется.

3. Создай канал `mpsc`, отправь из потока три числа, прочитай их в главном потоке.

4. Создай `Arc<Mutex<usize>>`, увеличь счётчик из двух потоков по 1000 раз. Проверь результат.

5. Ответь: чем `Rc<RefCell<T>>` отличается от `Arc<Mutex<T>>`?

---

## Обязательное задание

### 1. Параллельная сумма

```rust
pub fn parallel_sum(values: &[i64], threads: usize) -> i64;
```

Контракт:

1. Пустой срез → `0`.
2. `threads == 0` трактуется как `1` (или верни `0` для пустого; главное — не паниковать).
3. Если `threads` больше длины, лишние потоки можно не создавать.
4. Используй `std::thread::scope`, чтобы не требовать `'static` и не копировать срез.
5. Разбей срез на чанки, каждый поток считает сумму своего чанка; результаты сложи.
6. Переполнение `i64` — ответственность вызывающего; в тестах используй безопасные значения.

### 2. Параллельный MSE

```rust
pub fn parallel_mse(y_true: &[f64], y_pred: &[f64], threads: usize) -> Option<f64>;
```

Контракт:

1. Пустые срезы или разная длина → `None`.
2. `threads == 0` → `1`.
3. Каждый поток считает `(sum_sq, count)` для своего чанка.
4. Итог: `sum_sq / count`.
5. Результат должен совпадать с последовательным MSE с точностью до `1e-12`.

### 3. Счётчик

```rust
pub fn increment_counter(threads: usize, increments: usize) -> usize;
```

Контракт:

1. Создай `Arc<Mutex<usize>>`.
2. Запусти `threads` потоков; каждый делает `increments` инкрементов.
3. Дождись всех, верни итоговое значение.
4. `threads == 0` или `increments == 0` → `0`.
5. Используй `Arc::clone`, `move`, `lock().unwrap()` (в тестах/учебном коде допустимо).

### 4. Канальная обработка

```rust
pub fn channel_sum(values: &[i64], workers: usize) -> i64;
```

Контракт:

1. Главный поток отправляет числа в канал.
2. `workers` потоков читают из общего `Receiver` (через `Arc<Mutex<Receiver>>`).
3. Каждый worker суммирует свои числа и отправляет частичную сумму обратно.
4. Главный поток собирает частичные суммы и возвращает общую.
5. `workers == 0` → `1`; пустой вход → `0`.
6. Не должно быть deadlock: закрой отправитель, когда закончишь.

---

## Пошаговый план

1. Реализуй `parallel_sum` сначала последовательно, затем распараллель.
2. Разберись с нарезкой: `chunk_size = (len + threads - 1) / threads`, `values.chunks(chunk_size)`.
3. Используй `thread::scope(|scope| { ... })` и `scope.spawn(move || ...)`.
4. Собери `JoinHandle` в `Vec`, затем `join().unwrap()`.
5. Реализуй `parallel_mse` по той же схеме, но с двумя срезами и парой `(sum, count)`.
6. Реализуй `increment_counter` через `Arc<Mutex<usize>>`.
7. Реализуй `channel_sum`. Следи, чтобы `tx` был закрыт до чтения.
8. Напиши тесты, сравни параллельные результаты с последовательными.
9. Запусти:

   ```bash
   cargo test -p lab07-concurrency
   cargo clippy -p lab07-concurrency -- -D warnings
   cargo fmt
   ```

---

## Дополнительная часть

Выбери одно или несколько:

- замени `Mutex<usize>` на `AtomicUsize` и сравни код/скорость;
- добавь `rayon` и перепиши `parallel_sum` в одну строку; сравни с ручными потоками;
- сделай bounded-канал (`sync_channel`) и сравни поведение;
- реализуй пул воркеров, которые живут дольше одной задачи;
- добавь бенчмарк (вручную через `Instant`) для 1, 2, 4, 8 потоков;
- объясни, почему `Rc<RefCell<T>>` не подходит для потоков (подсказка: `Send`/`Sync`).

---

## Критерии приемки

- [ ] `cargo test -p lab07-concurrency` проходит;
- [ ] нет `unsafe`;
- [ ] `parallel_sum` и `parallel_mse` используют scoped threads;
- [ ] `increment_counter` не теряет инкременты;
- [ ] `channel_sum` не зависает;
- [ ] результаты совпадают с последовательными;
- [ ] `cargo clippy -p lab07-concurrency -- -D warnings` без предупреждений;
- [ ] `cargo fmt --check` проходит.

---

## На защите

1. Чем гонка данных (data race) отличается от гонки условий (race condition)?
2. Что означают `Send` и `Sync`? Приведи примеры.
3. Почему `Rc<T>` нельзя отправить в другой поток, а `Arc<T>` можно?
4. Зачем нужен `Mutex`? Что такое poisoning?
5. Чем `thread::scope` лучше `thread::spawn` для заимствованных данных?
6. Почему `move`-замыкание обязательно при `spawn`?
7. Как канал передаёт владение? Что происходит, когда отправитель закрыт?
8. Почему `Arc<Mutex<Receiver>>` нужен для нескольких consumer'ов?
9. Может ли Rust предотвратить deadlock? Почему нет?
10. Как бы ты распараллелил обучение нескольких моделей на разных потоках?

---

## Подсказки (открывать после попытки)

<details>
<summary>parallel_sum</summary>

```rust
use std::thread;

pub fn parallel_sum(values: &[i64], threads: usize) -> i64 {
    if values.is_empty() {
        return 0;
    }

    let threads = threads.max(1).min(values.len());
    let chunk_size = values.len().div_ceil(threads);

    thread::scope(|scope| {
        let mut handles = Vec::new();

        for chunk in values.chunks(chunk_size) {
            handles.push(scope.spawn(move || chunk.iter().sum::<i64>()));
        }

        handles.into_iter().map(|h| h.join().unwrap()).sum()
    })
}
```

`usize::div_ceil` стабилен с Rust 1.73.

</details>

<details>
<summary>increment_counter</summary>

```rust
use std::sync::{Arc, Mutex};
use std::thread;

pub fn increment_counter(threads: usize, increments: usize) -> usize {
    let counter = Arc::new(Mutex::new(0usize));
    let mut handles = Vec::new();

    for _ in 0..threads {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..increments {
                let mut value = counter.lock().unwrap();
                *value += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    *counter.lock().unwrap()
}
```

</details>
