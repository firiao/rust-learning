# 05. ML в Rust: библиотеки и реальность

Короткий ответ: **нейронки на Rust делать можно**, и экосистема за последние годы выросла. Но она моложе Python-экосистемы, и для исследовательской работы PyTorch/JAX всё ещё удобнее. Rust силён в инференсе, инфраструктуре, скорости, деплое и интеграции с Python.

Версии ниже проверены **6 ноября 2026**. Крейты `0.x` могут менять API — всегда сверяйся с `docs.rs` и README репозитория.

---

## Карта библиотек

| Библиотека | Версия | Что это | Autograd | GPU | Когда использовать |
|---|---:|---|---|---|---|
| **Candle** (`candle-core`, `candle-nn`, `candle-transformers`) | 0.11.1 | минималистичный DL-фреймворк от Hugging Face | да | CUDA, Metal | первый фреймворк, инференс, небольшие модели, LLM |
| **Burn** | 0.22.1 (0.23 в pre) | полноценный DL-фреймворк с абстракцией бэкендов | да | wgpu, CUDA, candle, tch, ndarray | обучение, переносимые модели, эксперименты |
| **tch** | 0.26.0 | биндинги к PyTorch C++ (libtorch) | да | CUDA, CPU | если уже знаешь PyTorch и нужна его мощь |
| **ndarray** | 0.17.2 | N-мерные массивы, аналог NumPy | нет | через BLAS | ручная математика, классический ML, обучение с нуля |
| **linfa** | 0.8.1 | классический ML в стиле scikit-learn | нет | CPU | регрессии, SVM, деревья, кластеризация |
| **ort** | 2.0.0-rc.15 | ONNX Runtime | нет (инференс) | CUDA, CPU | production-инференс ONNX |
| **tract** | 0.23.10 | чисто Rust-инференс ONNX/TF | нет | CPU | embedded, портативность, без C++ |
| **tokenizers** | 0.23.2 (1.0 rc) | быстрые токенизаторы Hugging Face | — | CPU | NLP/LLM |
| **hf-hub** | 1.0.0 | клиент Hugging Face Hub | — | — | скачивание моделей/токенизаторов |
| **polars** | актуальная | DataFrame, аналог pandas | — | CPU/GPU частично | подготовка данных |
| **pyo3** / **maturin** | актуальные | Rust ↔ Python | — | — | быстрые расширения для Python |

### Что не стоит брать первым

- **dfdx** (0.13, последнее обновление 2023) — интересный, но давно не обновлялся.
- **autograd** (1.1.1, 2021) — устарел.
- **luminal** (0.2, 2024) — нишевый, мало примеров.
- Экзотические крейты без документации и тестов.

---

## Candle

**Candle** — самый простой вход в DL на Rust. Его делает Hugging Face; на нём написаны `mistral.rs`, `text-embeddings-inference` и другие инференс-сервисы.

Плюсы:

- PyTorch-подобный API, тензоры и модули;
- CPU из коробки, CUDA/Metal — фичами;
- `candle-nn` — слои, лоссы, оптимизаторы;
- `candle-transformers` — готовые архитектуры (Llama, Whisper, ViT и т.д.);
- хорош для инференса и небольших экспериментов.

Минусы:

- API `0.x`, меняется;
- меньше готовых туториалов, чем у PyTorch;
- для серьёзного обучения Burn может быть удобнее.

```toml
[dependencies]
candle-core = "0.11"
candle-nn = "0.11"
```

```rust
use candle_core::{Device, Tensor};

fn main() -> candle_core::Result<()> {
    let device = Device::Cpu;
    let a = Tensor::new(&[[1f32, 2.], [3., 4.]], &device)?;
    let b = a.matmul(&a)?;
    println!("{b}");
    Ok(())
}
```

Для GPU:

```toml
candle-core = { version = "0.11", features = ["cuda"] }
```

CUDA требует установленного CUDA Toolkit; на Arch это отдельная возня. Начни с CPU.

---

## Burn

**Burn** — более «взрослый» фреймворк: абстракция бэкендов, autodiff, обучение, метрики, чекпоинты, импорт ONNX. Если Candle — это «быстрый и минималистичный», то Burn — «полноценная инфраструктура».

Плюсы:

- один код — разные бэкенды (`ndarray`, `wgpu`, `candle`, `tch`, `cuda`);
- обучение и инференс;
- `burn-import` для ONNX;
- активная разработка.

Минусы:

- больше абстракций, выше порог входа;
- API тоже молодой;
- сборка с тяжёлыми бэкендами может быть долгой.

```toml
[dependencies]
burn = "0.22"
```

Конкретные backend-фичи (`ndarray`, `wgpu`, `candle`, `tch`, `cuda`) выбираются по [burn-book](https://burn.dev/book/). Для CPU начни с `ndarray` или `wgpu`; для NVIDIA — `cuda`.

---

## tch-rs

**tch** — тонкая обёртка над libtorch (C++ PyTorch). Если ты уже знаешь PyTorch, это самый прямой путь: те же тензоры, слои, оптимизаторы, CUDA.

Плюсы:

- практически полный PyTorch API;
- можно загружать `state_dict` из Python-моделей;
- CUDA через libtorch.

Минусы:

- нужен libtorch (сотни мегабайт, отдельная установка);
- C++/CMake-зависимости;
- ошибки могут приходить из C++.

```toml
[dependencies]
tch = "0.26"
```

У tch есть фича `download-libtorch` для автоматической загрузки, но она тяжёлая и капризная. На Arch проще поставить `libtorch` из AUR/официального репозитория и указать `LIBTORCH`/`LD_LIBRARY_PATH`. Если не уверен — начни с Candle.

---

## ndarray

**ndarray** — это NumPy для Rust: `Array1`, `Array2`, `ArrayD`, срезы, views, broadcasting, `dot`, `map`, `zip`, `axis_iter`.

- **Нет autograd.** Градиенты пишешь сам.
- Отлично подходит для lab08 и lab09: понять, что происходит внутри фреймворка.
- `ndarray-linalg` добавляет BLAS/LAPACK-операции (нужна системная BLAS).

```toml
[dependencies]
ndarray = "0.17"
```

```rust
use ndarray::{array, Array1, Array2};

fn main() {
    let x: Array2<f64> = array![[1.0, 2.0], [3.0, 4.0]];
    let w: Array1<f64> = array![0.5, -0.5];
    let pred = x.dot(&w);
    println!("{pred:?}");
}
```

---

## linfa

**linfa** — scikit-learn для Rust. Алгоритмы разнесены по крейтам:

- `linfa-linear` — линейная регрессия;
- `linfa-logistic` — логистическая регрессия;
- `linfa-svm` — SVM;
- `linfa-trees` — деревья;
- `linfa-clustering` — KMeans, DBSCAN;
- `linfa-bayes` — наивный Байес;
- `linfa-datasets` — учебные датасеты.

Подходит для классического ML, но не для глубокого обучения. API стабильнее, чем у DL-фреймворков.

```toml
[dependencies]
linfa = "0.8"
linfa-linear = "0.8"
```

---

## Инференс: ort и tract

Обычный production-путь: обучить модель в PyTorch → экспортировать в ONNX → запустить в Rust.

### ort

Биндинги к ONNX Runtime. Быстрый, поддерживает CUDA, хорошо документирован. Версия 2.0 пока RC — API может меняться.

```toml
[dependencies]
ort = "2.0.0-rc.15"
```

Нужен ONNX Runtime (системный или через фичу `download-binaries`).

### tract

Чисто Rust-инференс ONNX/TensorFlow. Не требует C++ и системных библиотек, отлично подходит для embedded и портативных бинарников. Медленнее ONNX Runtime на больших моделях, но предсказуем.

```toml
[dependencies]
tract-onnx = "0.23"
```

---

## LLM и NLP

- **tokenizers** — быстрая токенизация (BPE, WordPiece, Unigram), совместима с Hugging Face.
- **hf-hub** — скачивание моделей и токенизаторов с Hugging Face.
- **candle-transformers** — готовые архитектуры (Llama, Mistral, Whisper, ViT, Stable Diffusion и др.).
- **mistral.rs** — отдельный проект инференса LLM на Candle.

Типичный стек:

```toml
[dependencies]
candle-core = "0.11"
candle-nn = "0.11"
candle-transformers = "0.11"
tokenizers = "0.23"
hf-hub = "1.0"
```

Это уже после lab10, не раньше.

---

## Данные и Python-интероп

- **polars** — DataFrame с ленивыми вычислениями, быстрый аналог pandas.
- **arrow** / **parquet** — колоночные форматы.
- **csv** — простой CSV-парсер.
- **pyo3** — вызывать Python из Rust и наоборот.
- **maturin** — собирать Rust-расширения для Python.

Практичный гибрид: исследование и обучение — в PyTorch/Python; предобработка, метрики, инференс, сервис — на Rust; связать через PyO3 или ONNX.

---

## Реальность: Rust vs Python для ML

| | Python + PyTorch | Rust + Candle/Burn |
|---|---|---|
| Скорость разработки | очень высокая | ниже |
| Экосистема | огромная | меньше |
| Готовые модели | Hugging Face, torchvision | Candle-transformers, ONNX |
| Отладка | привычная | компилятор + меньше runtime-сюрпризов |
| Инференс | PyTorch/Triton/ONNX | отличный, один бинарник |
| CPU-параллелизм | GIL/async | настоящие потоки без GIL |
| GPU | зрелый CUDA | работает, но настройка сложнее |
| Embedded/WASM | тяжело | отлично |
| Надёжность | зависит от тестов | сильная статическая проверка |

**Вывод:** не переписывай research-пайплайн на Rust «потому что модно». Используй Rust там, где он даёт реальное преимущество: инференс, latency, память, деплой, preprocessing, CLI, WebAssembly, embedded, Python-расширения.

---

## Путь в этом курсе

1. **labs 01–07** — язык. Без этого ML-API будет магией.
2. **lab08** — `ndarray`, линейная регрессия, MSE, градиентный спуск.
3. **lab09** — MLP с нуля, ручной forward/backward. Это даёт понимание, которое не даст никакой фреймворк.
4. **lab10** — тот же MLP на Candle. Сравни код и скорость.
5. **Дальше по интересу:**
   - Burn — если хочешь обучение и переносимость;
   - tch — если нужен PyTorch;
   - ort/tract — если нужен production-инференс;
   - tokenizers + hf-hub + candle-transformers — если LLM;
   - polars — если табличные данные;
   - pyo3 — если хочешь ускорить Python-код.

---

## Проверка перед стартом ML-трека

- [ ] Свободно пишу функции, структуры, enum, трейты.
- [ ] Понимаю `&T`, `&mut T`, move и lifetimes.
- [ ] Умею возвращать `Result` и обрабатывать ошибки.
- [ ] Пишу тесты и запускаю `cargo test`.
- [ ] Понимаю, что такое матричное умножение и производная.
- [ ] Могу объяснить MSE и шаг градиентного спуска.

Если часть пунктов не закрыта — это нормально, вернись к соответствующей лабе. ML-трек никуда не убежит.
