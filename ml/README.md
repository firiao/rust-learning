# ML-трек на Rust

Этот раздел — мост между «я выучил Rust» и «я делаю нейронки на Rust». Он идёт **после** labs 01–07. Исключение — [`01-perceptron`](01-perceptron/): его можно запустить уже после lab03, потому что там только структуры, массивы и циклы.

---

## Порядок

```text
labs 01–07  →  lab08 ndarray  →  lab09 MLP с нуля  →  lab10 Candle
                                                       │
                                                       ├── Burn
                                                       ├── tch-rs
                                                       ├── ort / tract
                                                       └── tokenizers + hf-hub
```

### 1. `01-perceptron` — сейчас

Один нейрон, OR/AND, обучение по ошибке. Показывает, что «нейронка» — это несколько умножений, сложение и порог.

```bash
cargo run -p perceptron
cargo test -p perceptron
```

Задания:

- поменяй `learning_rate` и число эпох; как меняется сходимость?
- попробуй XOR и убедись, что один перцептрон не справляется;
- нарисуй на бумаге разделяющую прямую для OR и AND;
- добавь третий вход и новый датасет.

### 2. lab08 — `ndarray` и линейная регрессия

Массивы, формы, broadcasting, `dot`, MSE, градиентный спуск. См. [`../assignments/lab08-ndarray.md`](../assignments/lab08-ndarray.md).

### 3. lab09 — MLP с нуля

Скрытый слой, ReLU, sigmoid, BCE, ручной backprop. См. [`../assignments/lab09-mlp-scratch.md`](../assignments/lab09-mlp-scratch.md).

### 4. lab10 — Candle

Тот же MLP, но autograd, `Var`, оптимизатор и устройство. См. [`../assignments/lab10-candle.md`](../assignments/lab10-candle.md).

### 5. Дальше

- **Burn** — если нужен полноценный фреймворк с бэкендами и обучением.
- **tch-rs** — если нужен PyTorch/libtorch.
- **ort** / **tract** — если нужен инференс ONNX.
- **tokenizers** + **hf-hub** — если нужны LLM/NLP.
- **polars** — если нужны табличные данные.
- **pyo3** — если нужно ускорить Python-код.

---

## Библиотеки

Кратко (подробно — в [`../docs/05-ml-ecosystem.md`](../docs/05-ml-ecosystem.md)):

| Библиотека | Роль |
|---|---|
| `candle-core`, `candle-nn`, `candle-transformers` | минималистичный DL, инференс, LLM |
| `burn` | полноценный DL-фреймворк, обучение |
| `tch` | PyTorch/libtorch |
| `ndarray` | массивы, ручная математика |
| `linfa` | классический ML |
| `ort` | ONNX Runtime |
| `tract` | чистый Rust-инференс |
| `tokenizers`, `hf-hub` | NLP/LLM |
| `polars` | DataFrame |

Версии проверены в ноябре 2026; `0.x`-крейты могут менять API.

---

## Честно о Rust в ML

- **Research** — Python/PyTorch/JAX пока удобнее: больше моделей, туториалов, людей.
- **Inference / production** — Rust отличен: один бинарник, низкая latency, предсказуемая память, нет GC и GIL.
- **Data preprocessing** — Rust + Polars/ndarray быстрые и надёжные.
- **Python extensions** — PyO3/maturin: писать горячие функции на Rust, вызывать из Python.
- **Обучение с нуля** — Candle/Burn уже можно, но готовых рецептов меньше.

Не переписывай весь ML-пайплайн на Rust. Начни с одной функции или инференса, сравни с Python, реши по цифрам.

---

## Мини-глоссарий

- **Tensor** — N-мерный массив чисел.
- **Shape** — размеры тензора, например `(32, 784)`.
- **Autograd** — автоматическое дифференцирование вычислительного графа.
- **Var** — параметр, для которого нужен градиент.
- **Loss** — функция потерь.
- **Optimizer** — правило обновления параметров (SGD, AdamW).
- **Epoch** — один проход по обучающему набору.
- **Batch** — порция примеров за один шаг.
- **Inference** — прямой проход без обучения.

---

## Что почитать

- Candle examples: <https://github.com/huggingface/candle/tree/main/candle-examples>
- Burn book: <https://burn.dev/book/>
- tch-rs: <https://github.com/LaurentMazare/tch-rs>
- ndarray docs: <https://docs.rs/ndarray/>
- linfa: <https://github.com/rust-ml/linfa>
- Are we learning yet?: <https://www.arewelearningyet.com/>
