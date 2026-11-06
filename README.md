# 🦀 Rust Learning — курс для ML-инженера

Учебная папка по Rust. Собрана под конкретный профиль: **Python знаешь хорошо, Java учишь недавно, направление — ML**. Поэтому здесь не будет объяснений «что такое цикл»; зато будет много про то, чем Rust отличается от Python и Java, как устроена память, почему компилятор ругается, и как на Rust писать ML-код.

Темп — медленный и вдумчивый. Один блок за раз, с кодом, тестами и вопросами на самопроверку.

---

## Быстрый старт

**Новичку с нуля — начни здесь: [`course/`](course/README.md).** Это линейный курс из 16 лекций: у каждой конспект, короткий тест и лаба на 10–20 минут.

```bash
cd ~/rust-learning

# проверить, что весь текущий код собирается и тесты проходят
cargo test --workspace

# запустить примеры по темам
./run_examples.sh

# сыграть в угадайку (первый маленький проект)
cargo run -p guess_game

# начать курс
# открой course/README.md
```

Остальные папки (`assignments/`, `labs/`, `docs/`, `ml/`) — справочники и вузовские лабораторные поглубже. Они дополняют курс, но проходить их подряд не обязательно.

> В системе уже стоят `rustc 1.99.1`, `cargo`, `clippy` и `rustfmt`. `rustup` не установлен, поэтому команды вида `rustup component add` здесь не используются. Офлайн-документация: `rustc --explain E0382`, `cargo doc --open`.

---

## Что такое Rust

Rust — компилируемый язык системного уровня. Начал разрабатываться в Mozilla в 2006 году (автор — Graydon Hoare), стабильная версия 1.0 вышла в 2015-м. Сейчас развивается под эгидой Rust Foundation; используется в Linux kernel, Windows, Android, Cloudflare, AWS, Microsoft, Dropbox и куче инфраструктурных проектов.

Главная идея: **скорость и контроль как в C/C++, но с безопасностью памяти, которую проверяет компилятор**. Без сборщика мусора и без ручного `free`.

Rust не «убивает» Python и не заменяет PyTorch в исследованиях. Его ниша — то, где важны скорость, предсказуемость, отсутствие GC и надёжность: CLI-утилиты, бэкенды, обработка данных, inference, WebAssembly, embedded, движки, криптография, инфраструктура для ML.

---

## 10 особенностей, которые определяют Rust

### 1. Ownership — владение

У каждого значения ровно один владелец. Когда владелец выходит из области видимости, значение уничтожается, память освобождается. Компилятор знает это статически, поэтому GC не нужен.

```rust
fn main() {
    let s1 = String::from("hello"); // s1 владеет строкой в куче
    let s2 = s1;                    // владение переместилось в s2 (move)
    // println!("{s1}");            // ошибка: s1 больше не владелец
    println!("{s2}");
}
```

### 2. Borrowing — заимствование

Вместо копирования можно дать ссылку. Правило на всю жизнь:

- либо **много** неизменяемых ссылок `&T`;
- либо **ровно одна** изменяемая `&mut T`;
- нельзя одновременно и то, и другое.

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];   // неизменяемое заимствование
println!("{first}");
v.push(4);           // можно: заимствование уже закончилось (NLL)
```

### 3. Нет GC, нет `null`, нет исключений

- Вместо `null` — `Option<T>` (`Some(value)` / `None`).
- Вместо исключений — `Result<T, E>` (`Ok(value)` / `Err(error)`).
- `panic!` — это не «исключение», а аварийное завершение; в обычной логике используют `Result`.

```rust
fn parse_age(text: &str) -> Result<u8, std::num::ParseIntError> {
    let age: u8 = text.parse()?; // ? вернёт Err наверх
    Ok(age)
}
```

### 4. Enums + pattern matching

`enum` в Rust — это алгебраический тип: вариант может нести данные. `match` заставляет обработать все варианты, иначе код не скомпилируется.

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
    }
}
```

Это очень удобно для ML: функции потерь, активации, метрики, конфиги моделей, состояния обучения.

### 5. Трейты вместо наследования

Классов и наследования в Rust нет. Есть структуры, перечисления и трейты — общие контракты поведения. Полиморфизм — через generics (статический) или `dyn Trait` (динамический).

```rust
trait Metric {
    fn name(&self) -> &str;
    fn score(&self, y_true: &[f64], y_pred: &[f64]) -> f64;
}

struct Mse;
impl Metric for Mse {
    fn name(&self) -> &str { "MSE" }
    fn score(&self, y_true: &[f64], y_pred: &[f64]) -> f64 {
        y_true.iter().zip(y_pred).map(|(a, b)| (a - b).powi(2)).sum::<f64>()
            / y_true.len() as f64
    }
}
```

### 6. Неизменяемость по умолчанию

`let x = 5;` — менять нельзя. Нужно `let mut x = 5;`. Это не мелочь: компилятор помогает не разносить случайные изменения по программе.

### 7. Zero-cost abstractions

Итераторы, замыкания, generics, `Option` — всё это компилируется в эффективный код, часто в обычный цикл. Абстракции не обязаны стоить производительности.

### 8. Fearless concurrency

Компилятор проверяет, что данные не разделяются между потоками небезопасно. Типы `Send` и `Sync` не дают случайно получить гонку данных. Это не «магия», но огромная помощь.

### 9. Cargo и инструменты

`cargo` — сборка, зависимости, тесты, примеры, бенчмарки, документация, публикация. Плюс `rustfmt` (форматирование), `clippy` (линтер), `rust-analyzer` (IDE). По качеству tooling Rust — один из лучших языков.

### 10. Macros и `unsafe`

Макросы (`println!`, `vec!`, `derive`) — это не C-препроцессор, а полноценная генерация кода. `unsafe` — явный opt-in для низкоуровневых операций; обычный код пишется в safe-подмножестве.

---

## Rust vs Python vs Java

| | Python | Java | Rust |
|---|---|---|---|
| Типизация | динамическая | статическая | статическая + вывод типов |
| Память | GC | GC (JVM) | ownership, без GC |
| Ошибки | исключения | исключения | `Result<T, E>`, `panic!` |
| Отсутствие значения | `None` | `null` | `Option<T>` |
| ООП | классы, multiple inheritance | классы, single inheritance | structs/enums + traits, без наследования |
| Полиморфизм | duck typing | интерфейсы/наследование | generics + `dyn Trait` |
| Потоки | GIL, async | threads, JMM | threads, `Send`/`Sync`, async |
| Сборка | pip/venv/poetry | Maven/Gradle | Cargo |
| Скорость | низкая | высокая (JIT) | очень высокая (AOT) |
| Кривая обучения | низкая | средняя | высокая |
| Типичные ниши | ML, скрипты, бэкенд | enterprise, бэкенд | systems, CLI, инфраструктура, WASM, ML-инференс |

### Что это значит на практике

- **Из Python** легко переносится мышление «данные + функции», но придётся привыкнуть к статическим типам и владению.
- **Из Java** легко переносится строгость и generics, но придётся отказаться от классов-иерархий и `null`.
- **Rust не прощает** неявных копий, случайных мутаций и «я потом разберусь». Зато после компиляции многие ошибки уже невозможны.

---

## Чего в Rust нет

- **Классов и наследования.** Есть структуры, перечисления, трейты и композиция.
- **Сборщика мусора.** Есть `Rc<T>`/`Arc<T>` для общего владения, но это не GC.
- **`null`.** Есть `Option<T>`.
- **Исключений.** Есть `Result<T, E>` и `panic!`.
- **Перегрузки функций.** Есть generics и трейты.
- **Аргументов по умолчанию.** Обычно builder pattern.
- **Стабильного ABI.** Для C-совместимости — `extern "C"`.

---

## ML в Rust: что есть из библиотек

Коротко: **делать нейронки можно**, но экосистема моложе Python. Для обучения с нуля и инференса — отлично; для SOTA-исследований PyTorch всё ещё удобнее.

| Библиотека | Что это | Когда брать |
|---|---|---|
| **Candle** (`candle-core`, `candle-nn`) | минималистичный DL-фреймворк от Hugging Face, PyTorch-подобный API, CPU/CUDA/Metal | первый фреймворк, инференс, небольшие модели |
| **Burn** | полноценный DL-фреймворк: autodiff, обучение, разные бэкенды, ONNX-импорт | обучение, переносимые модели, эксперименты |
| **tch** | биндинги к PyTorch C++ (libtorch) | если уже знаешь PyTorch и нужна его мощь |
| **ndarray** | N-мерные массивы, аналог NumPy | ручные вычисления, классический ML, понимание матчасти |
| **linfa** | классический ML в стиле scikit-learn | регрессии, SVM, деревья, кластеризация |
| **ort** | биндинги ONNX Runtime | production-инференс ONNX-моделей |
| **tract** | чисто Rust-инференс ONNX/TensorFlow | embedded, портативность, без C++ |
| **tokenizers**, **hf-hub** | токенизация и загрузка моделей с Hugging Face | LLM, NLP |
| **polars** | DataFrame-библиотека | подготовка данных, аналоги pandas |
| **pyo3**, **maturin** | Rust ↔ Python | писать быстрые расширения для Python/PyTorch |

Версии и подробности — в [`docs/05-ml-ecosystem.md`](docs/05-ml-ecosystem.md).

**Порядок для этого курса:**

1. Сначала язык: labs 01–07.
2. Потом `ndarray` и линейная регрессия: lab 08.
3. Потом MLP с ручным backprop: lab 09.
4. Потом тот же MLP на Candle: lab 10.
5. Дальше — Burn / tch / ort / tokenizers по интересу.

Не начинай с Candle или Burn «сразу». Без понимания ownership, срезов и итераторов API фреймворка будет выглядеть магией, а ошибки компилятора — необъяснимыми.

---

## Как устроена папка

```text
rust-learning/
├── README.md                 ← ты здесь
├── ROADMAP.md                ← весь маршрут по шагам
├── NOTES.md                  ← твой журнал обучения, заполняешь сам
├── Cargo.toml                ← workspace: все лабораторные и проекты
├── docs/                     ← справочники и глубокие разборы
│   ├── 00-getting-started.md
│   ├── 01-cargo-cheatsheet.md
│   ├── 02-rust-after-python-java.md
│   ├── 03-ownership.md
│   ├── 04-testing.md
│   └── 05-ml-ecosystem.md
├── assignments/              ← условия лабораторных (как в ITIP)
│   ├── lab00-getting-started.md   ← начни с него
│   ├── lab01-toolkit.md
│   ├── lab02-ownership.md
│   ├── lab03-domain-model.md
│   ├── lab04-traits-generics.md
│   ├── lab05-errors-iterators.md
│   ├── lab06-modules-testing.md
│   ├── lab07-concurrency.md
│   ├── lab08-ndarray.md
│   ├── lab09-mlp-scratch.md
│   └── lab10-candle.md
├── labs/                     ← код лабораторных
│   ├── lab01-toolkit/        ← готовый каркас, можно начинать
│   ├── lab02-ownership/      ← готовый каркас
│   └── lab03-domain-model/   ← готовый каркас
├── examples/                 ← 9 самостоятельных примеров по темам
├── ml/
│   ├── README.md             ← ML-трек и библиотеки
│   └── 01-perceptron/        ← перцептрон с нуля, без зависимостей
└── projects/
    └── guess_game/           ← первая законченная программа
```

Лабораторные устроены по тому же принципу, что и в `~/MTUCI/ITIP`:

- условие лежит отдельно в `assignments/`;
- в `labs/labNN-...` — код, тесты и README;
- в каждом задании есть **подготовительная**, **обязательная** и **дополнительная** части;
- есть «языковой бюджет»: не нужно тащить в lab01 traits и потоки;
- в конце — вопросы «на защите» для самопроверки.

---

## Как заниматься

1. **Читай мало, пиши много.** 30–60 минут в день лучше, чем 6 часов раз в неделю.
2. **Сначала `cargo check`, потом `cargo test`, потом `cargo clippy`.** Компилятор здесь — главный преподаватель.
3. **Не копируй код из примеров.** Набери руками, сломай, почини.
4. **Не воюй с borrow checker.** Если он ругается — сначала объясни себе, какое правило владения нарушено.
5. **После каждой лабы объясни вслух** (или в `NOTES.md`), что делает код и почему он скомпилировался.
6. **Не забегай вперёд.** Async, macros, unsafe и Candle — только после базы.
7. **Ошибки — это нормально.** Первые недели 80% времени уйдёт на понимание сообщений компилятора. Это и есть обучение.

### Мини-правила

- Идентификаторы — на английском, комментарии можно на русском.
- Функция возвращает результат, если её смысл не в печати.
- Тесты — часть задания, а не «потом».
- `unwrap()` допустим в учебных тестах и прототипах; в нормальной логике — `?` и обработка ошибок.
- Если код «непонятный, но работает» — это плохой код. Упрости.

---

## Полезные ресурсы

**База**

- The Rust Programming Language (официальная книга): <https://doc.rust-lang.org/book/>
- То же на русском: <https://doc.rust-lang.ru/book/>
- Rust by Example: <https://doc.rust-lang.org/rust-by-example/>
- Rust by Example на русском: <https://doc.rust-lang.ru/rust-by-example/>
- Rustlings (упражнения): <https://github.com/rust-lang/rustlings>
- 100 Exercises to Learn Rust: <https://github.com/mainmatter/100-exercises-to-learn-rust>
- Cheatsheet: <https://cheats.rs/>
- Standard library: <https://doc.rust-lang.org/std/>

**ML**

- Candle: <https://github.com/huggingface/candle>
- Burn: <https://burn.dev/>
- tch-rs: <https://github.com/LaurentMazare/tch-rs>
- linfa: <https://github.com/rust-ml/linfa>
- ndarray: <https://docs.rs/ndarray/>
- ort: <https://github.com/pykeio/ort>
- tokenizers: <https://github.com/huggingface/tokenizers>
- Каталог ML-крейтов: <https://www.arewelearningyet.com/>

**Новости**

- This Week in Rust: <https://this-week-in-rust.org/>
- Rust Blog: <https://blog.rust-lang.org/>

---

## Первое задание на сегодня

1. Запусти `cargo test --workspace` — убедись, что всё зелёное (часть тестов будет `ignored`, это нормально).
2. Запусти `cargo run -p guess_game` и сыграй один раз.
3. Открой `examples/01_hello.rs`, прочитай и запусти `./run_examples.sh`.
4. Прочитай `docs/02-rust-after-python-java.md` — там карта переходов с Python/Java на Rust.
5. Пройди [`assignments/lab00-getting-started.md`](assignments/lab00-getting-started.md) — входной практикум: окружение, Cargo, первый цикл «сломал — починил».
6. Открой `assignments/lab01-toolkit.md` и начни lab01.

Дальше — [`ROADMAP.md`](ROADMAP.md).
