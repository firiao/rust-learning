# 02. Rust после Python и Java

Ты уже знаешь два языка — это ускоряет учёбу и одновременно мешает: руки тянутся к привычным конструкциям. Ниже — карта переходов и главные ментальные сдвиги.

---

## Пять главных сдвигов

### 1. Типы проверяются до запуска

В Python ошибка `"2" + 2` вылезет в рантайме. В Rust — при компиляции. Это не «строгий Java», это ещё и вывод типов: часто писать типы не нужно.

```rust
let x = 42;          // i32 выведен
let y = 3.14;        // f64 выведен
let v = vec![1, 2];  // Vec<i32>
```

### 2. Владение вместо GC

В Python/Java объект живёт, пока на него есть ссылки; сборщик сам разберётся. В Rust **у значения один владелец**, и компилятор заранее знает, когда память освободится. Подробно — в [`03-ownership.md`](03-ownership.md).

### 3. Ошибки — это значения

Не `try/except` и не `throws`, а `Result<T, E>`:

```rust
fn read_number(text: &str) -> Result<i32, std::num::ParseIntError> {
    let n = text.parse()?;
    Ok(n)
}
```

Вызывающий обязан как-то обработать `Result` (или явно `unwrap`/`expect`, что видно в коде).

### 4. Вместо `null` — `Option<T>`

```rust
let maybe_name: Option<String> = None;
let name = maybe_name.unwrap_or_else(|| String::from("без имени"));
```

Компилятор не даст забыть про `None`: чтобы достать значение, нужен `match`, `if let`, `?` или комбинатор.

### 5. Композиция вместо наследования

В Java ты привык к `class Dog extends Animal`. В Rust — `struct Dog` + `impl Animal for Dog`. Наследования нет вообще. Это не «недоделанное ООП», это осознанный дизайн: трейты + generics + композиция.

---

## Таблица соответствий

### Базовое

| Python | Java | Rust |
|---|---|---|
| `x = 1` | `int x = 1;` | `let x = 1;` |
| `x = 1; x += 1` | `x += 1;` | `let mut x = 1; x += 1;` |
| `def f(a, b):` | `int f(int a, int b)` | `fn f(a: i32, b: i32) -> i32` |
| `None` | `null` | `Option<T>` (`None`/`Some`) |
| `True`/`False` | `true`/`false` | `true`/`false` |
| `str` | `String` | `String` (владеющая) / `&str` (срез) |
| `list` | `ArrayList<T>` / массив | `Vec<T>` / `[T; N]` |
| `dict` | `HashMap<K,V>` | `HashMap<K,V>` |
| `set` | `HashSet<T>` | `HashSet<T>` |
| `tuple` | нет прямого аналога | `(T1, T2)` |
| `for x in xs:` | for-each / Stream | `for x in &xs` |
| `[f(x) for x in xs]` | Stream API | `xs.iter().map(f).collect()` |
| `lambda x: x+1` | лямбда | `|x| x + 1` |
| `class` | `class` | `struct` + `impl` |
| `interface` | `interface` | `trait` |
| `raise` | `throw` | `return Err(...)` / `panic!` |
| `try/except` | `try/catch` | `match result { Ok/Err }` / `?` |
| `with` | try-with-resources | RAII / `Drop` |
| `import` | `import` | `use` + `mod` |
| `pip`/`venv` | Maven/Gradle | Cargo |
| `pytest` | JUnit | `#[test]` + `cargo test` |
| `dataclass` | record/POJO | `#[derive(Debug, Clone, PartialEq)] struct` |
| `@decorator` | аннотация | атрибут `#[...]` (не то же самое) |

### Типы

| Python | Java | Rust |
|---|---|---|
| `int` (произвольной длины) | `int`/`long`/`BigInteger` | `i8/i16/i32/i64/i128/isize`, `u8...` |
| `float` (double) | `double`/`float` | `f32`/`f64` |
| `bool` | `boolean` | `bool` |
| `str` | `String`/`char` | `String`, `&str`, `char` (4 байта, Unicode scalar) |
| `bytes` | `byte[]` | `Vec<u8>`, `&[u8]` |
| `Optional[T]` | `Optional<T>` | `Option<T>` |
| `Union[...]` | sealed interface | `enum` |
| `Any` | `Object` | generics / `dyn Trait` |

---

## Пример: одна задача на трёх языках

Посчитать среднее положительных чисел.

**Python**

```python
def average_positive(values):
    positives = [x for x in values if x > 0]
    if not positives:
        return None
    return sum(positives) / len(positives)
```

**Java**

```java
static Double averagePositive(List<Double> values) {
    double sum = 0;
    int count = 0;
    for (double x : values) {
        if (x > 0) { sum += x; count++; }
    }
    return count == 0 ? null : sum / count;
}
```

**Rust**

```rust
fn average_positive(values: &[f64]) -> Option<f64> {
    let mut sum = 0.0;
    let mut count = 0usize;

    for &x in values {
        if x > 0.0 {
            sum += x;
            count += 1;
        }
    }

    if count == 0 { None } else { Some(sum / count as f64) }
}
```

Или идиоматичнее через итераторы:

```rust
fn average_positive(values: &[f64]) -> Option<f64> {
    let positives: Vec<f64> = values.iter().copied().filter(|&x| x > 0.0).collect();
    if positives.is_empty() {
        None
    } else {
        Some(positives.iter().sum::<f64>() / positives.len() as f64)
    }
}
```

Обрати внимание:

- `Option<f64>` вместо `None`/`null` — тип явно говорит, что результата может не быть.
- `&[f64]` — срез: работает и для массива, и для `Vec`.
- `count as f64` — явное приведение, неявных нет.
- `for &x in values` — итерируемся по ссылкам и разыменовываем каждый элемент.

---

## Что переносится из Python

✅ Быстрое прототипирование, выражение через итераторы/comprehensions → `map`/`filter`/`collect`.
✅ Функции как значения → замыкания.
✅ Работа со строками и срезами.
✅ «Утиная типизация» → трейты (но статически).
✅ `with` → `Drop`/RAII.
✅ Генераторы → итераторы (но писать их дольше).

## Что переносится из Java

✅ Строгая типизация, generics, интерфейсы → трейты.
✅ Пакеты → модули/крейты.
✅ Maven/Gradle → Cargo.
✅ JUnit → встроенные тесты.
✅ `try-with-resources` → RAII.
✅ Многопоточность → `std::thread`, но с проверкой на гонки.

## Что не надо переносить

❌ **Наследование.** В Rust его нет. Не пытайся эмулировать `class A extends B`; используй композицию и трейты.
❌ **`null`.** Не нужен; `Option<T>`.
❌ **Исключения.** Не нужны; `Result<T, E>`.
❌ **Общие изменяемые ссылки.** В Java объект можно менять из любого места; в Rust — только через `&mut` и только одну.
❌ **`Object`/`Any`.** Лучше generics или `dyn Trait`.
❌ **Привычку «сначала написать, потом разберёмся с типами».** Компилятор не даст.
❌ **`staticmethod`-архитектуру.** В Rust свободные функции — норма; методы — в `impl`.
❌ **Глубокие иерархии классов.** Плоская модель + enum + трейты обычно лучше.

---

## Типичные ошибки после Python

### «Я просто изменю строку в функции»

```rust
fn add_exclamation(s: String) {   // забирает владение
    println!("{s}!");
}

let s = String::from("hi");
add_exclamation(s);
// println!("{s}"); // ошибка: s перемещена
```

Решение: `fn add_exclamation(s: &str)` или вернуть `String`.

### «Сделаю список и буду менять его в цикле»

```rust
let mut v = vec![1, 2, 3];
for x in &v {
    // v.push(4); // ошибка: нельзя менять, пока итерируемся по ссылкам
    println!("{x}");
}
```

Решение: собрать изменения в отдельный `Vec` или использовать индексы, если действительно нужно.

### «Верну ссылку на локальную строку»

```rust
// fn bad() -> &str {
//     let s = String::from("hello");
//     &s // ошибка: s будет уничтожена в конце функции
// }
```

Решение: вернуть `String` или принимать ссылку снаружи.

### «Сделаю `clone()` на всё»

Это компилируется, но часто означает, что ты борешься с ownership, а не решаешь задачу. `clone()` — нормальный инструмент, но не рефлекс. Сначала спроси: можно ли взять `&T`?

---

## Типичные ошибки после Java

### «Где `extends`?»

Нет. `trait` описывает поведение, `struct` — данные, `impl` соединяет их. Общее поведение — через generics/trait objects.

### «Где `null`?»

Нет. `Option<T>`. Если поле может отсутствовать — `Option<T>`, а не «магическое» значение вроде `-1`.

### «Где конструктор?»

Есть `fn new(...) -> Self` — соглашение, а не ключевое слово. Можно несколько разных конструкторов с разными именами.

### «Где `toString()`?»

`impl fmt::Display for T` для пользовательского вывода и `#[derive(Debug)]` для отладочного `{:?}`.

### «Где `equals`/`hashCode`?»

`#[derive(PartialEq, Eq, Hash)]` или ручные `impl PartialEq`/`Hash`. Для `HashMap` ключ должен реализовывать `Eq + Hash`.

### «Где геттеры/сеттеры?»

Поля по умолчанию приватны для других модулей. Доступ — через методы `impl`, если нужно. Внутри одного модуля поля видны.

---

## Упражнение на переход

Перепиши на Rust знакомый Python-код:

```python
def normalize(values):
    if not values:
        raise ValueError("empty")
    total = sum(values)
    if total == 0:
        raise ValueError("zero sum")
    return [x / total for x in values]
```

Подсказка: в Rust не нужен `raise`; верни `Result<Vec<f64>, &'static str>`. Сделай это после lab05, когда пройдёшь `Result`.

---

## Что читать дальше

- [`03-ownership.md`](03-ownership.md) — главное отличие Rust.
- [`04-testing.md`](04-testing.md) — как проверять код.
- [`05-ml-ecosystem.md`](05-ml-ecosystem.md) — ML-библиотеки.
