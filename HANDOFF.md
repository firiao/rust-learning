# Handoff: курс Rust для ML-инженера

**Дата:** 6 ноября 2026  
**Рабочая папка курса:** `/home/firiao/rust-learning`  
**Файл handoff:** `/home/firiao/.opencode/plan/rust-learning-handoff.md`  
**Статус:** база курса создана и проверена; lab10 (Candle) и lab05 (нумерация строк) закрыты — см. раздел 18.

---

## 0. Как продолжить в новом чате

Скопируй в новый чат примерно это:

```text
Прочитай /home/firiao/.opencode/plan/rust-learning-handoff.md и продолжи работу.
Рабочая папка: /home/firiao/rust-learning.
Сначала проверь cargo test --workspace, cargo clippy --workspace --all-targets -- -D warnings
и cargo fmt --all -- --check. Не переписывай существующие файлы без необходимости.
Главный незакрытый вопрос — lab10 Candle: нужно проверить сходимость XOR,
поправить assignment под реальный API Candle и разобраться с save/load.
После этого спроси пользователя, хочет ли он начать lab01 или сначала добить lab10.
```

---

## 1. Контекст пользователя

- Студент 2 курса российского ИТ-вуза.
- Направление/профиль — **ML-инженер** (инженерия систем искусственного интеллекта).
- Хорошо знает **Python**.
- Около 2–3 месяцев учит **Java**.
- Стало скучно, захотел выучить **Rust**.
- Попросил:
  - создать отдельную папку для обучения Rust;
  - рассказать про язык, его особенности;
  - проверить, есть ли библиотеки для собственных нейросетей;
  - если есть — добавить их в общий трек-путь;
  - идти **медленно, но углублённо**, «всё по чуть-чуть, понемногу, шаг за шагом»;
  - посмотреть архитектуру лабораторных в `~/MTUCI/ITIP` — она нравится пользователю.

Ключевые предпочтения:

- Русский язык общения.
- Не гнать вперёд, объяснять причины и внутреннее устройство.
- Формат лабораторных как в вузе: условие отдельно, код отдельно, подготовительная/обязательная/дополнительная части, критерии, вопросы «на защите».
- ML — не абстрактное будущее, а часть курса: сначала язык, потом `ndarray`, потом MLP с нуля, потом Candle.
- Не перегружать теорией без кода.
- Пользователь пока **не начал** проходить лабораторные; ему создана стартовая инфраструктура.

---

## 2. Окружение

- ОС: **Arch Linux**, x86_64.
- Домашняя папка: `/home/firiao`.
- `rustc 1.99.1 (58cf8f9c3 2026-10-13) (Arch Linux rust 1:1.99.1-1)`.
- `cargo 1.99.1 (5f94df478 2026-08-27)`.
- `cargo clippy 0.1.99`.
- `cargo fmt` / `rustfmt 1.10.0`.
- `rustup` **не установлен**.
- `rust-analyzer` **не установлен** (можно поставить `sudo pacman -S rust-analyzer` и расширение VS Code, но без согласия пользователя не устанавливать).
- Используется **edition 2024**, workspace resolver `3`.
- `git` в `/home/firiao/rust-learning` **не инициализирован**, коммитов нет.
- В папке есть `target/` (около 24 МБ), он в `.gitignore`.

---

## 3. Изученная архитектура вуза

Смотрели:

```text
/home/firiao/MTUCI/ITIP/ITIP/
├── README.md
├── assignments/lab00-getting-started.md ... lab08-concurrency.md
├── docs/00-getting-started.md ... 08-existing-repositories.md
├── lab01/ ... lab08/
│   ├── README.md
│   ├── pom.xml
│   └── src/main/java, src/test/java
├── pom.xml
└── mvnw
```

Что понравилось и было перенесено в Rust-курс:

- центральный `README.md` с картой курса;
- отдельные `assignments/` с условиями;
- отдельные `docs/` со справочниками;
- отдельные папки лабораторных с кодом;
- три уровня: подготовительная, обязательная, дополнительная части;
- «языковой бюджет» — какие конструкции разрешены на данном этапе;
- критерии приёмки;
- вопросы «на защите» для самопроверки;
- акцент на тесты и объяснение кода, а не на «лишь бы работало».

---

## 4. Созданная структура

```text
/home/firiao/rust-learning/
├── .gitignore
├── Cargo.lock
├── Cargo.toml                  # workspace
├── README.md                   # главный обзор курса
├── ROADMAP.md                  # маршрут на 16–20 недель
├── run_examples.sh             # сборка/запуск всех examples
├── docs/
│   ├── 00-getting-started.md
│   ├── 01-cargo-cheatsheet.md
│   ├── 02-rust-after-python-java.md
│   ├── 03-ownership.md
│   ├── 04-testing.md
│   └── 05-ml-ecosystem.md
├── assignments/
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
├── labs/
│   ├── lab01-toolkit/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/lib.rs
│   ├── lab02-ownership/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/lib.rs
│   └── lab03-domain-model/
│       ├── Cargo.toml
│       ├── README.md
│       └── src/lib.rs
├── examples/
│   ├── README.md
│   ├── 01_hello.rs
│   ├── 02_ownership.rs
│   ├── 03_borrow.rs
│   ├── 04_structs_enums.rs
│   ├── 05_match.rs
│   ├── 06_errors.rs
│   ├── 07_traits.rs
│   ├── 08_collections.rs
│   └── 09_threads.rs
├── ml/
│   ├── README.md
│   └── 01-perceptron/
│       ├── Cargo.toml
│       ├── README.md
│       └── src/main.rs
└── projects/
    └── guess_game/
        ├── Cargo.toml
        ├── README.md
        └── src/main.rs
```

`Cargo.toml` workspace:

```toml
[workspace]
resolver = "3"
members = [
    "labs/lab01-toolkit",
    "labs/lab02-ownership",
    "labs/lab03-domain-model",
    "ml/01-perceptron",
    "projects/guess_game",
]
```

Лабораторные 04–10 пока **не созданы как Cargo-проекты** — для них есть только подробные assignment-файлы. Пользователь должен создать их сам по инструкции, когда дойдёт.

---

## 5. Учебный маршрут

### Часть 0. Вход

Cargo, примеры, `guess_game`, первый проект.

### Часть I. Язык (labs 01–07)

| Lab | Тема | Статус |
|---|---|---|
| 01 | Функции, типы, тесты, `Option` | каркас готов, 3 функции `todo!()` |
| 02 | Ownership, borrowing, slices | каркас готов, 3 функции `todo!()` |
| 03 | Structs, enums, методы, match | каркас готов, методы `todo!()` |
| 04 | Traits, generics, trait objects | только assignment |
| 05 | Errors, iterators, closures | только assignment |
| 06 | Modules, testing, docs | только assignment |
| 07 | Threads, channels, `Arc<Mutex>` | только assignment |

### Часть II. ML (labs 08–10)

| Lab | Тема | Статус |
|---|---|---|
| 08 | `ndarray`, линейная регрессия, GD | только assignment, математика проверена |
| 09 | MLP с нуля, forward/backprop | только assignment, формулы проверены |
| 10 | Тот же MLP на Candle | только assignment, **требует доработки** |

### Часть III. По интересам

Burn, tch-rs, ort/tract, tokenizers/hf-hub, polars, pyo3, ONNX, WASM.

---

## 6. Что уже проверено и работает

### 6.1. `cargo test --workspace`

Проходит:

- `guess_game`: 5 passed.
- `lab01-toolkit`: 4 passed, 11 ignored (ignored — это TODO-тесты).
- `lab02-ownership`: 9 passed, 11 ignored.
- `lab03-domain-model`: 5 passed, 17 ignored.
- `perceptron`: 5 passed.
- doc-tests: 0.

Итог: **0 failed**, ignored-тесты ожидаемы.

### 6.2. Clippy и fmt

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

Оба проходят без ошибок (после `cargo fmt --all`).

### 6.3. Примеры

`./run_examples.sh` собирает и запускает все 9 примеров. Все компилируются и работают.

Исправленные по ходу проблемы:

- `examples/04_structs_enums.rs`: поле `active` не читалось → добавлен `println!`.
- `examples/05_match.rs`: неполный `match` по срезу → добавлен вариант `[only]`.
- `examples/03_borrow.rs`: неверный байтовый срез `..6` для кириллицы → исправлено на `..12`.
- `examples/08_collections.rs`: `fold` с `&i32` → `acc * *value`.

### 6.4. `guess_game`

Игра «угадай число» без внешних зависимостей: xorshift RNG, `read_line`, `Option`, `Ordering`, тесты. Запуск:

```bash
cargo run -p guess_game
```

### 6.5. `ml/01-perceptron`

Перцептрон с нуля на OR/AND. Запуск:

```bash
cargo run -p perceptron
```

Фактический вывод:

```text
=== Перцептрон: OR ===
До обучения: accuracy = 0.75
После обучения: accuracy = 1.00
weights = [0.1, 0.1], bias = -0.1000
...
=== Перцептрон: AND ===
accuracy = 1.00
```

Это первый ML-проект, доступный уже после lab03.

### 6.6. Проверка `ndarray` (во временной папке)

Проверены:

- `x.dot(&w) + bias`;
- `x.t().dot(&residual) * scale`;
- `&prediction - y`;
- `dz2.t().dot(hidden) * scale`;
- `sum_axis(Axis(0))`;
- `&w - &(&grad * lr)`;
- пустые `Array2::zeros((0, 2))` и `Array1::zeros(0)`.

Всё компилируется.

Линейная регрессия на синтетике (`n=200`, `lr=0.1`, `5000` эпох) дала:

```text
w ≈ [3.0, -1.9996], b ≈ 5.0001, loss 31.94 -> 0.00005
```

То есть lab08 с указанными гиперпараметрами сходится.

### 6.7. Проверка MLP с нуля (во временной папке)

После исправления LCG (`>> 33` → `>> 32`) MLP `2 → 8 → 1`, XOR, `lr=0.5`, `10000` эпох:

```text
seed 1: loss 0.699724 -> 0.000309, acc 1.00
seed 42: loss 0.711887 -> 0.000086, acc 1.00
```

Формулы backprop в lab09 корректны.

### 6.8. Candle

Минимальный пример работает:

```rust
let a = Tensor::new(&[[1f32, 2.], [3., 4.]], &Device::Cpu)?;
let b = a.matmul(&a)?;
```

Вывод:

```text
[[ 7., 10.],
 [15., 22.]]
```

---

## 7. ML-библиотеки: исследование (ноябрь 2026)

Проверялись через `cargo info` и crates.io API.

| Библиотека | Версия | Роль |
|---|---:|---|
| `candle-core` / `candle-nn` / `candle-transformers` | 0.11.1 | минималистичный DL от Hugging Face, PyTorch-подобный API, CPU/CUDA/Metal |
| `burn` | 0.22.1 stable (0.23.0-pre.1) | полноценный DL-фреймворк, абстракция бэкендов, обучение, ONNX-импорт |
| `tch` | 0.26.0 | биндинги к PyTorch/libtorch |
| `ndarray` | 0.17.2 | N-мерные массивы, NumPy-подобный API, без autograd |
| `linfa` | 0.8.1 | классический ML в стиле scikit-learn |
| `ort` | 2.0.0-rc.15 | ONNX Runtime, инференс |
| `tract` | 0.23.10 | чистый Rust-инференс ONNX/TensorFlow |
| `tokenizers` | 0.23.2 stable (1.0.0-rc.3) | быстрые токенизаторы HF |
| `hf-hub` | 1.0.0 | клиент Hugging Face Hub |
| `polars` | актуальная | DataFrame, аналог pandas |
| `pyo3` / `maturin` | актуальные | Rust ↔ Python |

Не рекомендуется как первый выбор:

- `dfdx` 0.13 — последнее обновление 2023;
- `autograd` 1.1.1 — 2021;
- `luminal` 0.2 — 2024, нишевый.

Ключевой вывод для пользователя:

- Rust **можно** использовать для нейросетей;
- для research Python/PyTorch всё ещё удобнее;
- Rust силён в inference, деплое, preprocessing, CLI, WASM, embedded, Python-расширениях;
- учебный порядок: `ndarray` → MLP с нуля → Candle → Burn/tch/ort/tokenizers.

---

## 8. Candle: важные API-факты и незакрытый вопрос

### 8.1. Реальный API Candle 0.11.1

Проверено по исходникам:

- `candle_nn::linear(in_dim, out_dim, vb)` → `Linear`;
- `Linear::forward(&Tensor)` через трейт `Module`;
- `Tensor::relu()`;
- `candle_nn::ops::sigmoid(&Tensor)`;
- loss: **`candle_nn::loss::binary_cross_entropy_with_logit`** (слово `logit` в единственном числе);
- `candle_nn::AdamW::new(vars, ParamsAdamW { lr, weight_decay, ..Default::default() })` требует `use candle_nn::Optimizer`;
- `ParamsAdamW` поля: `lr`, `beta1`, `beta2`, `eps`, `weight_decay` (default `weight_decay = 0.01`);
- `candle_nn::SGD::new(vars, lr)`;
- `VarMap::new()`, `VarMap::all_vars()`, `VarMap::save(path)`, `VarMap::load(path)`;
- `VarBuilder::from_varmap(&varmap, DType::F32, &device)`;
- `vb.pp("l1")`;
- `optimizer.backward_step(&loss)?`.

### 8.2. Важный нюанс save/load

`VarMap::load` **не создаёт переменные**, а обновляет уже существующие по именам. Поэтому порядок такой:

```rust
let mut varmap = VarMap::new();
let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);
let model = Mlp::new(2, 8, vb)?; // переменные уже созданы
varmap.load(path)?;              // теперь load имеет куда загружать
```

Нельзя сделать пустой `VarMap`, вызвать `load`, а потом создать модель — загрузка ничего не сделает.

### 8.3. Что уже проверено с Candle XOR

Первый тест:

- hidden = 8;
- `AdamW`, `lr = 0.1`, `5000` эпох;
- `weight_decay` по умолчанию `0.01`;
- forward возвращал logits, loss — `binary_cross_entropy_with_logit`.

Результат:

```text
epoch 0: loss = 0.9843393
epoch 1000: loss = 0.3477012
epoch 2000: loss = 0.34667975
epoch 3000: loss = 0.34663662
epoch 4000: loss = 0.34660503
probs = [[0.49998], [0.49998], [0.99997], [0.0000679]]
```

То есть accuracy ~3/4, XOR не выучен. Модель застряла. Возможные причины:

- слишком маленький hidden;
- `weight_decay = 0.01` мешает;
- `lr`/инициализация;
- ReLU-юниты умерли;
- нужно больше эпох или другой оптимизатор.

Была попытка прогнать варианты (AdamW с `wd=0.0`, `lr=0.5`, hidden 16/32, SGD), но команда была прервана переходом в Plan mode. Файл эксперимента: `/tmp/opencode/candle-mlp` (может сохраниться).

### 8.4. Что нужно сделать с lab10

1. Обновить `assignments/lab10-candle.md`:
   - `forward` должен возвращать **logits**, а не вероятности;
   - loss — `binary_cross_entropy_with_logit`;
   - `predict` применяет sigmoid;
   - убрать/исправить упоминание `candle_nn::loss::bce`;
   - исправить пример save/load (сначала модель, потом `varmap.load`).
2. Подобрать гиперпараметры, при которых XOR стабильно даёт accuracy >= 0.99:
   - попробовать `AdamW` с `weight_decay = 0.0`, `lr = 0.1–0.5`, hidden 8/16/32, 10k–20k эпох;
   - попробовать `SGD` с `lr = 0.5–1.0`;
   - при необходимости tanh вместо ReLU;
   - проверить несколько seed/инициализаций.
3. Проверить save/load в правильном порядке и добавить тест на совпадение предсказаний.
4. После этого можно считать lab10 готовой.

---

## 9. Известные проблемы и TODO

### P0 — обязательно

1. **lab10 Candle** — не сходится XOR на текущих гиперпараметрах; assignment использует неточный API. См. раздел 8.
2. **lab05 line numbering** — в assignment/hint номер строки фактически считается по непустым строкам, а не по физическим строкам исходного ввода. Нужно либо уточнить контракт («номер непустой строки»), либо переписать hint на физические номера.

### P1 — желательно

3. Создать `NOTES.md` — на него ссылаются lab02 и lab10, но файла пока нет. Можно сделать шаблон с разделами по лабораторным.
4. Добавить `assignments/lab00-getting-started.md` в стиле ITIP (сейчас вход описан в `docs/00-getting-started.md` и README).
5. По мере прохождения создать каркасы lab04–lab07 (сейчас только условия). Не создавать все сразу без запроса — пользователь идёт медленно.
6. Проверить все относительные ссылки в markdown (в основном они корректны).

### P2 — опционально

7. Инициализировать git в `rust-learning` и сделать первый commit (пользователь не просил; спросить).
8. Поставить `rust-analyzer` (пакет + расширение VS Code) — только с согласия.
9. Добавить отдельный пример/проект на Burn или tch после lab10.
10. Добавить `examples/10_smart_pointers.rs` (`Box`, `Rc`, `RefCell`, `Arc`, `Mutex`) — сейчас тема разбросана по docs/lab07.

---

## 10. Важные содержательные решения

- Курс оформлен как **10 лабораторных**, а не как набор разрозненных уроков.
- Labs 01–03 — рабочие каркасы с `todo!()` и `#[ignore]`-тестами, чтобы `cargo test` был зелёным на старте.
- Labs 04–10 — только условия; пользователь создаёт проекты сам, это часть обучения.
- В labs 01–07 **нет внешних зависимостей**.
- В lab08–09 используется только `ndarray`.
- В lab10 — `candle-core` + `candle-nn`.
- ML-трек начинается **после** lab07, кроме `ml/01-perceptron`, который доступен после lab03.
- Стиль документации: русский, с сравнениями Python/Java/Rust, без снисходительности, но и без перегруза.
- Все лабораторные имеют:
  - цель;
  - что нужно знать;
  - языковой бюджет;
  - подготовительную часть;
  - обязательное задание с контрактом;
  - пошаговый план;
  - дополнительную часть;
  - критерии приёмки;
  - вопросы «на защите»;
  - подсказки под `<details>`.

---

## 11. Хронология действий (что было сделано)

1. Проверено окружение: `rustc`, `cargo`, `clippy`, `rustfmt`, отсутствие `rustup`/`rust-analyzer`.
2. Изучена структура `/home/firiao/MTUCI/ITIP/ITIP`: README, assignments, docs, lab01–08, pom.xml.
3. Создана папка `/home/firiao/rust-learning` и подпапки `examples`, `projects/guess_game`, затем `docs`, `assignments`, `labs`, `ml`.
4. Создан корневой `Cargo.toml` workspace, `.gitignore`.
5. Написан `README.md` — обзор Rust, особенности, сравнение с Python/Java, ML-библиотеки, карта папки, ресурсы.
6. Написан `ROADMAP.md` — маршрут на 16–20 недель, labs 01–10, контрольные точки, проекты.
7. Написаны `docs/00`–`docs/05`:
   - первый запуск;
   - Cargo;
   - переход с Python/Java;
   - ownership/lifetimes;
   - тесты;
   - ML-экосистема.
8. Написаны 10 assignment-файлов lab01–lab10.
9. Созданы каркасы lab01, lab02, lab03 с тестами и `todo!()`.
10. Создан `ml/01-perceptron` — рабочий перцептрон без зависимостей.
11. Создан `projects/guess_game` — рабочая игра.
12. Созданы 9 примеров `examples/01`–`09` и `run_examples.sh`.
13. Прогнаны `cargo test --workspace`, `cargo clippy`, `cargo fmt`.
14. Прогнаны все примеры; исправлены найденные ошибки.
15. Проверены версии ML-крейтов через `cargo info`/crates.io.
16. Во временных папках проверены:
    - `ndarray`-операции;
    - линейная регрессия;
    - MLP с нуля (после исправления LCG);
    - минимальный Candle;
    - Candle XOR (обнаружена проблема сходимости).
17. Начата проверка вариантов Candle XOR — прервана переходом в Plan mode.

---

## 12. Команды, которые точно работают

```bash
cd /home/firiao/rust-learning

cargo test --workspace
cargo test -p lab01-toolkit
cargo test -p lab02-ownership
cargo test -p lab03-domain-model
cargo test -p perceptron
cargo test -p guess_game

cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
cargo fmt --all

./run_examples.sh

cargo run -p guess_game
cargo run -p perceptron
```

Ожидаемое поведение:

- все тесты проходят;
- часть тестов lab01–lab03 — `ignored`, пока пользователь не реализует `todo!()`;
- `cargo clippy` и `cargo fmt --check` чистые;
- примеры компилируются и запускаются.

---

## 13. Что НЕ сделано

- `lab04`–`lab10` не имеют Cargo-каркасов (только assignments).
- `NOTES.md` не создан.
- `git init`/commit не сделаны.
- `rust-analyzer` не установлен.
- lab10 Candle не доведена до рабочего состояния.
- lab05 line numbering не уточнён.
- Нет отдельного проекта на Burn/tch/ort.
- Пользователь ещё не проходил лабораторные; прогресса по lab01 нет.

---

## 14. Рекомендуемый следующий шаг

Сначала **добить lab10 Candle** (P0), потому что это единственное место, где материалы могут ввести пользователя в заблуждение. После этого:

1. Спросить пользователя, хочет ли он:
   - начать lab01 (если хочет учиться «с нуля»);
   - сначала посмотреть/добить lab10;
   - получить каркас lab04.
2. Если пользователь начинает lab01:
   - открыть `README.md`, `ROADMAP.md`, `assignments/lab01-toolkit.md`;
   - запустить `cargo test -p lab01-toolkit`;
   - объяснить ignored-тесты и `todo!()`;
   - не давать готовое решение сразу, а вести по подсказкам.
3. Если пользователь хочет ML сразу:
   - предупредить, что lab08–10 требуют базы;
   - можно дать `ml/01-perceptron` как развлечение, но не замену lab01–07.

---

## 15. Тон и стиль для нового чата

- Обращаться на «ты», по-русски.
- Не читать лекцию на 10 страниц без кода.
- Объяснять через сравнения с Python/Java.
- Не захваливать и не ругать; спокойно объяснять ошибки компилятора.
- Не использовать `unwrap()` в примерах без пояснения.
- Не предлагать `unsafe`, async, macros, Candle/Burn раньше времени.
- Подчёркивать: **сначала язык, потом ML-фреймворки**.
- Если пользователь застрял — вернуться к ownership/borrowing, а не пихать `clone()`.
- Сохранять стиль ITIP: контракт → тесты → критерии → самопроверка.

---

## 16. Быстрая карта ключевых файлов

| Файл | Что внутри |
|---|---|
| `README.md` | обзор Rust, 10 особенностей, сравнение с Python/Java, ML-таблица, карта курса, ресурсы |
| `ROADMAP.md` | маршрут по неделям, labs 01–10, контрольные точки, проекты |
| `docs/00-getting-started.md` | установка/проверка окружения, Cargo-проект, отладка |
| `docs/01-cargo-cheatsheet.md` | команды Cargo, workspace, features, тесты |
| `docs/02-rust-after-python-java.md` | таблицы соответствий, ментальные сдвиги, типичные ошибки |
| `docs/03-ownership.md` | глубокий разбор ownership, borrowing, lifetimes, ошибок E0382/E0502/E0499/E0515 |
| `docs/04-testing.md` | unit/integration/doc-тесты, assert, ignored, property-based |
| `docs/05-ml-ecosystem.md` | Candle, Burn, tch, ndarray, linfa, ort, tract, tokenizers, PyO3, реальность Rust в ML |
| `assignments/lab01-toolkit.md` | функции, тесты, `Option`, контракт `is_prime`/`is_palindrome`/`average` |
| `assignments/lab02-ownership.md` | move/borrow/slices, эксперименты с компилятором |
| `assignments/lab03-domain-model.md` | ML-доменная модель на struct/enum |
| `assignments/lab04-traits-generics.md` | метрики, `dyn Trait`, `Vector2` |
| `assignments/lab05-errors-iterators.md` | CSV-парсер, свои ошибки, итераторы |
| `assignments/lab06-modules-testing.md` | модули, публичный API, integration/doc-тесты |
| `assignments/lab07-concurrency.md` | scoped threads, mpsc, Arc<Mutex> |
| `assignments/lab08-ndarray.md` | ndarray, линейная регрессия, MSE, GD |
| `assignments/lab09-mlp-scratch.md` | MLP 2→8→1, ReLU, sigmoid, BCE, ручной backprop |
| `assignments/lab10-candle.md` | Candle, VarMap, autograd, XOR, save/load |
| `labs/lab01-toolkit/src/lib.rs` | `is_even` реализована; `is_prime`, `is_palindrome`, `average` — TODO |
| `labs/lab02-ownership/src/lib.rs` | 5 функций реализованы; 3 — TODO |
| `labs/lab03-domain-model/src/lib.rs` | типы и часть методов; остальные — TODO |
| `examples/*.rs` | 9 самостоятельных примеров по темам |
| `ml/01-perceptron/src/main.rs` | рабочий перцептрон OR/AND + тесты |
| `projects/guess_game/src/main.rs` | игра + xorshift RNG + тесты |

---

## 17. Финальная заметка

Всё содержимое папки создано ассистентом; существующие файлы пользователя вне `/home/firiao/rust-learning` не изменялись.  
Если новый чат будет продолжать работу, ему не нужно заново исследовать Rust-экосистему — версии и выводы уже есть в этом файле и в `docs/05-ml-ecosystem.md`.  
Главное — не торопиться, не переписывать курс целиком и сначала закрыть Candle-лабу.

---

## 18. Обновление 6 ноября 2026, вечер (DeepSeek Harness) — читать вместо §8.3–8.4 и P0 из §9

### 18.1. lab10 Candle закрыта

Причина «незакрытого вопроса» из §8.3 установлена и воспроизведена: это **не** ошибка API, а
попадание в локальный минимум с мёртвыми ReLU-нейронами. Симптом: loss застревает около `0.3466`
(`≈ 2·ln(2)/4`), две вероятности ровно `0.5`. Воспроизведено на `hidden=8`.

Подбор гиперпараметров (Candle 0.11.1, по 10 независимых запусков на конфигурацию, затем 50 для финалистов):

| hidden | lr | weight_decay | epochs | успех |
|---:|---:|---:|---:|---:|
| 8 | 0.1 | 0.01 | 2000 | 97/100 |
| 8 | 0.1 | 0.0 | 2000 | 98/100 |
| 8 | 0.1 | 0.0 | 5000 | 98/100 |
| 8 | 0.5 | 0.0 | 5000 | 2/10 |
| **16** | **0.1** | **0.0** | **2000** | **100/100** |
| 16 | 0.05 | 0.0 | 2000 | 50/50 |
| 32 | 0.1 | 0.0 | 2000 | 50/50 |

Выбранный конфиг: **hidden=16, lr=0.1, weight_decay=0.0, epochs=2000**. `hidden=8` не сходится
в 2–3% запусков — именно так и возникла проблема, описанная в §8.3 (loss ≈ 0.3466).

Уточнённые факты API Candle 0.11.1 (проверено компиляцией):

- `candle_nn::loss::binary_cross_entropy_with_logit` — `logit` в единственном числе, вход — **логиты**;
  функции `bce` нет;
- `ParamsAdamW::default().weight_decay == 0.01` — для XOR нужно явно `0.0`;
- **`Device::set_seed` на CPU не работает**: `cannot seed the CPU rng with set_seed`.
  Детерминизм в тестах — через `varmap.set_one("l1.weight", &tensor)` уже после создания модели;
- имена переменных: `l1.weight`, `l1.bias`, `l2.weight`, `l2.bias`;
- `VarMap::load` не создаёт переменные — порядок «модель → load» подтверждён;
- загрузка с другим `hidden_dim` даёт `Err` (несовпадение формы) — хороший тест;
- `Tensor::ge(0.5)` → `u8`, для 0.0/1.0 нужен `.to_dtype(DType::F32)`.

`assignments/lab10-candle.md` переписан: `forward` возвращает логиты, sigmoid — в `probabilities`/`predict`,
loss — `binary_cross_entropy_with_logit`, исправлен пример save/load, добавлены проверенные
гиперпараметры, раздел про двойной sigmoid, детерминизм тестов и подсказки с проверенным кодом.

Референсная реализация (5 тестов, clippy и fmt чистые) лежит в `/home/firiao/rust-learning/.scratch/lab10-ref`
(папка в `.gitignore`, target-каталоги удалены).

### 18.2. lab05 нумерация строк закрыта

Контракт зафиксирован: номер строки — **физический, 1-based, с учётом пустых строк**; колонка — 1-based
после `split(',')`. В задание добавлен раздел «Нумерация строк и колонок» с примером, шаг плана
переписан (`enumerate()` до `filter`), каркас парсера исправлен и проверен 6 тестами (в `.scratch/lab05-check`).

### 18.3. Добавлено

- `assignments/lab00-getting-started.md` — входной практикум в стиле ITIP (окружение, Cargo,
  воспроизведение `E0382`, тест на факториал, `fmt`/`clippy`).
- `NOTES.md` — шаблон журнала обучения по всем лабораторным + таблица сравнения lab09/lab10.
- `AGENTS.md` — контекст для ИИ-агентов (стиль, команды, проверенные факты, состояние курса).
  Загружается автоматически плагином `agent-instructions`.
- `.gitignore`: добавлен `.scratch/`.

### 18.4. Настройка Harness (первый запуск)

Профиль `web` (`~/.dsh/profiles/web/cordis.patch.yml`), включены плагины:

- `compaction-basic` + `command-compact` — автоматическое сжатие контекста и `/compact`;
- `tool-result-pruner` — обрезка слишком длинных выводов инструментов;
- `agent-instructions` — подхват `AGENTS.md`;
- `plan-mode`, `skill-filesystem`.

### 18.5. Важно про окружение агента

- `/tmp` в песочнице **приватный на каждый вызов** bash — временные файлы не переживают переход
  между вызовами. Для экспериментов: `~/rust-learning/.scratch/` (в `.gitignore`).
- **`RAYON_NUM_THREADS=1`** обязателен для Candle: без него `gemm` спавнит потоки и на том же
  прогоне получается 6м42с вместо 7с (в 55 раз медленнее).

### 18.6. Что осталось

- Пользователь ещё не начинал лабораторные. Следующий шаг — спросить: lab00/lab01 или ML сразу.
- `git init` не сделан (пользователь не просил).
- `rust-analyzer` не установлен.
- Каркасы lab04–lab07 не созданы — так и задумано, студент создаёт сам.

### 18.7. Проверки после изменений

```text
cargo test --workspace                  → 0 failed (ignored ожидаемы)
cargo clippy --workspace --all-targets -- -D warnings → чисто
cargo fmt --all -- --check              → чисто
./run_examples.sh                       → все 9 примеров работают
все относительные ссылки в markdown     → 0 битых
```

---

## 19. Новый линейный курс `course/` (6 ноября 2026, ночь)

По просьбе пользователя собран **отдельный линейный курс** «от нуля до junior»:
`course/` — 16 лекций, у каждой три файла: `lecture.md` (конспект), `quiz.md` (короткий тест
с ответами под `<details>`), `lab.md` (лаба на 10–20 минут). Всего 49 файлов, ~8000 строк.

| № | Папка | Тема |
|---:|---|---|
| 1 | `01-cargo` | Cargo, структура проекта, fmt/clippy |
| 2 | `02-types` | переменные, `mut`, числа, `String`/`&str`, UTF-8 |
| 3 | `03-functions` | выражения, `if` как выражение, `for`/`while`/`loop` |
| 4 | `04-ownership` | move, `Copy`, `clone` + 4 эксперимента с компилятором |
| 5 | `05-borrowing` | `&`/`&mut`, правила заимствования, срезы |
| 6 | `06-structs` | `struct`, `impl`, `derive` |
| 7 | `07-enums` | `enum` с данными, `match`, `Option`, `if let` |
| 8 | `08-errors` | свой `enum` ошибок, `Display`/`Error`, `?` |
| 9 | `09-traits` | трейты, `<T: Trait>` против `dyn Trait` |
| 10 | `10-collections` | `Vec`, `HashMap`, `entry`, сортировка |
| 11 | `11-iterators` | адаптеры/потребители, замыкания как параметры |
| 12 | `12-modules` | модули, `pub`, unit/integration/doc-тесты |
| 13 | `13-smart-pointers` | `Box` в рекурсии, `Rc`, `RefCell` |
| 14 | `14-threads` | `thread::scope`, `mpsc`, `Arc<Mutex>` |
| 15 | `15-project` | CLI `wcount`: `lib.rs` + `main.rs`, `ExitCode` |
| 16 | `16-ml-final` | линейная регрессия + чеклист junior |

Ключевые решения:

- проекты лаб создаются в `practice/`; корневой workspace получил `"practice/*"` в `members`
  и `exclude = [".scratch"]`; `practice/00-sandbox` — постоянная песочница (нужна, чтобы glob
  не был пустым: с пустым glob cargo падает с «No such file or directory»);
- у lab12 и lab15 по несколько файлов (`src/lib.rs`, `src/stats.rs`, `src/text.rs`, `tests/integration.rs`);
- в lab4/lab5/lab8/lab11/lab12/lab13/lab15 есть блоки «Эксперименты» — раскомментировать и смотреть ошибку;
- **все 16 лаб прогнаны компиляцией**: эталоны в `.scratch/verify/l01…l16` (у каждой `cargo test` зелёный,
  clippy чистый), плюс отдельно проверено, что **заготовки с `todo!()` компилируются** (`cargo check`);
- lab16 проверена сходимость: `y = 3x + 2`, x = 0..9, `lr = 0.01`, 2000 эпох →
  `w = 3.000003`, `b = 1.999982`, loss `54.85 → ~0`;
- в lab10 пришлось заменить `len() % 2 == 0` на `is_multiple_of(2)` — иначе `clippy -D warnings`
  ругается (lint `manual_is_multiple_of`, rustc 1.99).

`README.md` в корне теперь ведёт новичка в `course/`, а `assignments/`, `labs/`, `docs/`, `ml/`
описаны как справочники «поглубже».

### Что осталось

- Пользователь ещё не начинал курс. Следующий шаг — вести его по lab01…lab16, не давая готовых решений.
- Идеи на будущее (не просил): lab17 про `serde`/`clap`, lab18 про `async`/`tokio`.

---

## 20. Курс переписан с нуля (6 ноября 2026, ночь) — читать вместо §19

**Пользователь забраковал версию курса из §19.** Претензии были точные:

1. термины использовались до объяснения (`borrow checker` — без определения; `match` с `Some(x)` и `_`
   в лекции про Cargo, когда синтаксис Rust человек видит впервые);
2. вместо объяснения — «clippy ловит, например, вот такое: `match value { Some(x) => ..., _ => {} }`»,
   из чего новичок не может понять ничего;
3. после первой лекции осталось понимание только «есть свой Maven и ревьюер».

**Решение:** версия 1 отправлена в архив `.scratch/legacy/course-v1` (не удалять), `course/` переписан.

Правила версии 2 (обязательны для всех следующих лекций):

- каждое новое слово объясняется **до** первого использования;
- в начале лекции — список «Новые слова» с определением в одну строку;
- каждый блок кода разобран построчно, с разметкой частей;
- `course/GLOSSARY.md` — словарь терминов в порядке появления, пополняется с каждой лекцией;
- ни одного забегания вперёд: если конструкция нужна позже, она не появляется раньше;
- порядок лекций строго по зависимостям (`match` только после `enum`, `enum`+`Option` только после
  структур/функций, `borrow checker` только после владения).

Готово: `course/README.md` (карта + правила), `course/GLOSSARY.md`, лекции 1–6
(`01-first-program`, `02-values`, `03-functions`, `04-conditions-loops`, `05-ownership`, `06-borrowing`),
~2500 строк. **Лаб и тестов нет** — пользователь просил сначала только лекции, потом тесты и лабы.

Все примеры и тексты ошибок в лекциях 1–6 **проверены компилятором** (E0308, E0317, E0382, E0384,
E0423, E0425, E0499, E0502, E0596, E0597, E0601, `literal out of range`, `not a char boundary`,
overflow panic) — тексты скопированы из реального вывода `rustc 1.99.1`, а не написаны по памяти.
Две ошибки в собственных черновиках были найдены именно так: `println` без `!` даёт `E0423`, а не
`E0425`; `println!(...)` без `;` в конце `main` **компилируется** (нужен пример с двумя командами).

Дальше: лекции 7–18 по плану из `course/README.md` (structs, enums, match, errors, traits, collections,
iterators, modules, smart-pointers, threads, project, ml-final), затем тесты и лабы к каждой лекции.

---

## 21. План расширен до 28 лекций + ML-трек (6 ноября 2026, ночь)

Пользователь попросил проверить три чекпоинта. Честный аудит дал: (1) junior — НЕТ, (2) необъяснённое —
почти, 8 отложенных мест, (3) ML-трек — НЕТ, ML была одна лекция в конце. Пользователь выбрал:
**полный объём 28 лекций + 4 проекта + капстоун** и **забегания объяснять на месте + реестр**.

### 21.1. Что сделано

- Исправлены 8 отложенных объяснений в лекциях 1–6: макросы (`!` объяснён через «функция не может
  принять переменное число аргументов разных типов»), `'Привет'` (один символ), `crate` в цитате,
  `&str` (что значит `&`), debug/release (таблица + 255+1→0), `if` (что делает), методы (как работает
  точка), `{:?}` (убрано — срез печатается циклом), `vec!` (убрано — описано словами).
- `course/FORWARD-REFERENCES.md` — реестр всех оставшихся забеганий: что это, где появилось,
  что объяснено на месте, где полный разбор. Плюс раздел «что было исправлено».
- `course/README.md` переписан: 28 лекций по 8 блокам, ML-нить с 13-й лекции, 4 проекта, капстоун,
  честная оценка «100–120 часов», чеклист и мок-собеседование в лекции 28.
- Написаны лекции 7–10: `07-structs`, `08-match`, `09-enums-option`, `10-errors`.
  **Порядок изменён**: `match` (8) идёт ДО `enum` (9) — иначе `match` не на чем объяснять,
  а `if let` требует уже известного `Option`.

### 21.2. Правила, которые нельзя нарушать

- каждое новое слово — в списке «Новые слова» с определением в одну строку;
- каждый блок кода разобран построчно;
- любое забегание — с объяснением на месте + строка в `FORWARD-REFERENCES.md`;
- **все тексты ошибок копировать из реального `rustc 1.99.1`**, не по памяти.
  Так уже найдены 3 своих ошибки: `println` без `!` = E0423 (не E0425); `println!(...)` без `;`
  в конце `main` компилируется; `Option + i32` = E0369 (не E0277), `?` вне Result — E0277 с хвостом
  про `FromResidual`.

### 21.3. Где остановились

Готовы лекции 1–10 (~3600 строк), `GLOSSARY.md`, `FORWARD-REFERENCES.md`, README с планом.
Следующая порция: **11 traits, 12 generics, 13 collections, 14 iterators, 15 closures, 16 modules-tests-docs**
(на 13-й начинается ML-нить: лабы про CSV и статистику). Потом 17–24 (память, потоки, cargo-deep,
serde, clap, errors-in-practice, macros-unsafe, async) и 25–28 (ndarray, mlp-scratch, candle,
inference-pyo3). Тесты и лабы — после подтверждения лекций пользователем.

---

## 22. Написаны лекции 11–14 (7 ноября 2026)

Пользователь: «лекции хорошие, можешь их допиливать» — курс продолжается по плану 28 лекций.
Добавлены:

- `11-traits` — трейт как контракт, `impl Трейт for Тип`, методы по умолчанию, `dyn`/трейт-объекты,
  сравнение `enum` vs трейт, `derive` = автогенерация `impl` (замыкает тему из лекции 7);
- `12-generics` — `<T>`, ограничения `T: PartialOrd + Copy`, обобщённые структуры, `impl Trait`,
  `where`, мономорфизация (и сравнение с erasure в Java);
- `13-collections` — `Vec`, `vec!`, `[]` vs `get()`, `String` глубже, `HashMap`, `entry().or_insert()`,
  **первая ML-нить**: парсинг CSV в `Vec<Vec<f64>>`;
- `14-iterators` — `next()`, три вида `iter`, ленивость, адаптеры/потребители, `|&&n|` vs `|&n|`,
  «рыбка» `::<Vec<i32>>`, **ML-нить**: `mean` и `mse` через итераторы.

Все примеры скомпилированы и запущены; все тексты ошибок скопированы из живого `rustc`.
Найдены и исправлены 3 свои неточности:

- `text.len()` после `push_str` — не 12, а 21 (переписал пример);
- `HashMap<f64, _>`: объявление компилируется, ошибка `E0599` возникает только на `insert`
  (было написано «E0277 при объявлении»);
- `dyn Shape` в сообщении `E0277` печатается как `(dyn Shape + 'static)`.

Обновлены `course/README.md` (статус 1–14, ссылки), `FORWARD-REFERENCES.md` (+8 строк:
`'static`, `Box<dyn>`, `vec!`, `.iter().map().sum()`, `to_string`, `use`, `filter_map`, замыкания).

**Следующая порция:** `15-closures`, `16-modules-tests-docs`, `17-smart-pointers`, `18-threads`.
Важно: в лекции 17 нужен короткий раздел про lifetimes — в реестре забеганий `&'static str`
обещан разбор именно там.

Ссылки в markdown проверены: 87, битых 0. `cargo test --workspace` зелёный.

---

## 23. Написаны все 28 лекций (7 ноября 2026)

По просьбе «сделай всё сразу» дописаны лекции 15–28 плюс `CHECKLIST.md`. Итого `course/`:
28 папок с `lecture.md`, `README.md`, `GLOSSARY.md`, `FORWARD-REFERENCES.md`, `CHECKLIST.md` —
~9200 строк, 116 ссылок, битых 0.

### Что проверено компилятором

- 15 closures, 16 modules/tests (unit + integration + doc-тесты), 17 smart pointers, 18 threads —
  все примеры прогнаны, тексты ошибок взяты из живого `rustc`;
- 20 serde, 21 clap, 22 anyhow/thiserror, 24 tokio — собраны офлайн с закреплением версий
  (clap `=4.5.1`, т.к. более новые транзитивные зависимости отсутствуют в кэше);
- 25 ndarray — линейная регрессия сошлась к `w=[2.996, 1.987]`, `b=1.015`;
- 26 MLP с нуля — XOR, hidden=8, lr=0.1, 10 000 эпох: loss `0.7507 → 0.000145`, 20/20 seeds.

### Что НЕ проверено

- **лекция 28 (`ort` + `pyo3`)** — крейты не в кэше, сеть для записи в `~/.cargo` закрыта
  (read-only fs в песочнице). В тексте стоит явное предупреждение, что код — ориентир,
  сверять с docs.rs. Если пользователь даст доступ — перепроверить и убрать предупреждение.

### Найденные и исправленные ошибки (важно для доверия к материалу)

1. `>> 33` вместо `>> 32` в LCG → все веса инициализировались отрицательными, ReLU мертвы,
   сеть не училась ни при каком seed. Это та же ошибка, что была в §6.7 старой версии.
   Теперь она описана в лекции 26 как учебная ловушка.
2. `sum()` без `::<f64>` в candle-подобном коде — `E0283`.
3. `extern "C" {}` в edition 2024 требует `unsafe extern "C" {}`.
4. В примере регрессии (25) признаки были линейно зависимы → бесконечно много решений.
   Данные заменены, а история разобрана в лекции как ML-урок про коллинеарность.
5. `HashMap<f64, _>`: ошибка `E0599` возникает на `insert`, а не при объявлении (E0277).

### Осталось

- `quiz.md` (короткий тест) и `lab.md` (лаба 10–20 минут) к каждой из 28 лекций;
- 4 проекта + капстоун (описаны в README и в CHECKLIST).

Пользователь просил «сделай всё сразу, я потом если что дёрну» — то есть следующий шаг
начинать с тестов и лаб, но только после его подтверждения.

---

## 24. Тесты и лабы для лекций 1–6 (7 ноября 2026)

Пользователь: «Делай дальше. Прочитаю всё потом. Щас на 4 лекции» — то есть писать тесты и лабы,
не дожидаясь чтения.

Сделано: к каждой из лекций 1–6 добавлены `quiz.md` (8–10 вопросов с разборами под `<details>`)
и `lab.md` (задача на 15–20 минут + критерии + подсказки).

| Лаба | Тема | Проверка |
|---|---|---|
| l01-hello | первая программа, чтение ошибок | тестов нет (лаба про запуск и 3 эксперимента) |
| l02-text | `char_count`, `initials`, `mask_digits`, `shout` | 5 тестов |
| l03-functions | fizzbuzz, collatz, fib, print_and_count | 4 теста |
| l04-loops | sum_while, count_digits, max_digit, is_prime, print_table | 5 тестов |
| l05-ownership | append_bang, total, longest_word_owned, double + 4 эксперимента | 5 тестов |
| l06-borrowing | first_word, count_vowels, push_bang, sum_slice, swap + 3 эксперимента | 6 тестов |

Все эталоны — в `.scratch/verify/l01…l06`: `cargo test` зелёный, `clippy --all-targets -- -D warnings`
чистый. Отдельно проверено, что **заготовки с `todo!()` компилируются** (`cargo check --all-targets`)
для лаб 2–6.

### Грабли, найденные при проверке

`clippy` 1.99 выдаёт `error: manual implementation of '.is_multiple_of()'` на любом `x % n == 0`
(линт `manual_is_multiple_of`). Это ломало лабы 3 (fizzbuzz, collatz) и 4 (is_prime).
Решение: в эталонах и подсказках используется `n.is_multiple_of(15)` и т.д., в тексте лаб
объяснено, почему привычное `%` не пройдёт. Если менять эти лабы — не возвращать `% == 0`.

### Дальше

Лекции 7–12: `07-structs`, `08-match`, `09-enums-option`, `10-errors`, `11-traits`, `12-generics` —
сначала `quiz.md`, потом `lab.md`, с проверкой в `.scratch/verify/lNN`.

---

## 25. Крейты лаб создаются заранее (7 ноября 2026)

Пользователь: «Лучше создай заранее файлы для лаб, и сам напиши тесты и TODO'шки».
То есть студент не должен делать `cargo new` и копировать код из `lab.md`.

Сделано для лаб 1–6: в `practice/` лежат готовые крейты
`l01-hello` (bin, без тестов — лаба про запуск и чтение ошибок),
`l02-text`, `l03-functions`, `l04-loops`, `l05-ownership`, `l06-borrowing` (lib, тесты + `todo!()`).

`lab.md` переписаны: вместо «создай проект → вставь код» теперь «открывай `practice/lNN`, вот что
внутри, реализуй». Код в markdown и в крейтах больше не дублируется — единственный источник
истины это `practice/lNN/src/lib.rs`, эталон — `.scratch/verify/lNN`.

### Следствия, которые надо помнить

- `cargo test --workspace` **красный**, пока лабы не реализованы (`todo!()` паникует). Это ожидаемо;
  проверять надо `cargo test -p <пакет>`. Для проверки самого курса —
  `cargo test -p lab01-toolkit … -p guess_game` или эталоны в `.scratch/verify`.
- Заготовки дают предупреждения `unused variable` (параметры не используются из-за `todo!()`).
  Это описано в каждой лабе: предупреждения исчезнут после реализации.
- `clippy -D warnings` на заготовке падает — в критериях написано «чистый **после** реализации».

### Дальше

Для лекций 7–28 делать так же: сначала `quiz.md`, потом `lab.md` + готовый крейт
в `practice/lNN-…` с тестами и `todo!()`, затем эталон в `.scratch/verify/lNN` и прогон
`cargo test` + `clippy --all-targets -- -D warnings` на эталоне.

