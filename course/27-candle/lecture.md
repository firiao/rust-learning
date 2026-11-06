# Лекция 27. Candle: autograd и обучение

**Время:** ~35 минут

**Нужно знать заранее:** MLP с нуля (лекция 26), `ndarray`, трейты, `Result`.

**Новые слова в этой лекции:**
- **`Tensor`** — многомерный массив в Candle (как `ndarray::Array`, но с поддержкой вычислений на устройстве);
- **`Device`** — где считать: `Cpu`, `Cuda`, `Metal`;
- **`Var`** — тензор, по которому можно считать градиенты;
- **`VarMap`** — хранилище именованных `Var` (параметров модели);
- **`VarBuilder`** — «строитель», который создаёт параметры с нужными именами;
- **autograd** — автоматическое вычисление градиентов;
- **`backward_step`** — «посчитать градиенты и обновить веса».

---

## 1. Что даёт фреймворк

В лекции 26 мы писали backprop руками: 40 строк, где легко перепутать порядок обновления. Candle делает то же самое автоматически.

```bash
cargo add candle-core@0.11 candle-nn@0.11
```

Два крейта: `candle-core` — тензоры и устройство, `candle-nn` — слои, оптимизаторы, функции потерь.

> **Совет по скорости.** Candle на CPU запускает матричные умножения через `gemm`/`rayon`. На тензорах вида `4×2` накладные расходы на потоки больше, чем сама работа. Если обучение XOR идёт медленно (секунды на эпоху), запускай с одним потоком:
>
> ```bash
> RAYON_NUM_THREADS=1 cargo run --release
> ```
>
> Проверено: тот же прогон ускоряется примерно в 50 раз.

---

## 2. Тензоры

```rust
use candle_core::{DType, Device, Tensor};

fn main() -> candle_core::Result<()> {
    let device = Device::Cpu;

    let a = Tensor::new(&[[1f32, 2.], [3., 4.]], &device)?;
    println!("{:?}", a.dims());          // [2, 2]

    let b = a.matmul(&a)?;               // матричное умножение
    println!("{b}");

    let z = Tensor::zeros((4, 1), DType::F32, &device)?;
    println!("{:?}", z.dims());          // [4, 1]

    Ok(())
}
```

Разберём:

- `Device::Cpu` — считать на процессоре;
- `Tensor::new(&[...], &device)` — создать из «вложенного массива»;
- `.dims()` — форма (аналог `shape()` в ndarray);
- `.matmul(&a)` — матричное умножение;
- `Tensor::zeros((4, 1), DType::F32, &device)` — тензор из нулей заданной формы;
- `candle_core::Result<T>` — свой тип ошибки крейта; `?` работает как обычно (лекция 10).

---

## 3. Параметры: `Var`, `VarMap`, `VarBuilder`

Чтобы autograd знал, **по чему** считать градиенты, параметры должны быть типа `Var` и лежать в `VarMap`.

```rust
use candle_core::{DType, Device};
use candle_nn::{linear, Linear, Module, VarBuilder, VarMap};

struct Mlp {
    l1: Linear,
    l2: Linear,
    varmap: VarMap,
}

impl Mlp {
    fn new(input_dim: usize, hidden_dim: usize, vb: VarBuilder, varmap: VarMap) -> candle_core::Result<Self> {
        let l1 = linear(input_dim, hidden_dim, vb.pp("l1"))?;
        let l2 = linear(hidden_dim, 1, vb.pp("l2"))?;
        Ok(Self { l1, l2, varmap })
    }

    /// Логиты, форма (n, 1). Sigmoid здесь НЕ применяется.
    fn forward(&self, x: &Tensor) -> candle_core::Result<Tensor> {
        let h = self.l1.forward(x)?.relu()?;
        self.l2.forward(&h)
    }
}
```

Разберём:

- `VarMap::new()` — пустое хранилище параметров;
- `VarBuilder::from_varmap(&varmap, DType::F32, device)` — «строитель», который будет создавать `Var` в этом `VarMap`;
- `vb.pp("l1")` — префикс имён: параметры получат имена `l1.weight`, `l1.bias`. Это важно для сохранения;
- `linear(in, out, vb.pp("l1"))` — готовый линейный слой: сам создаёт `weight` и `bias`;
- `Linear::forward` — через трейт `Module` (лекция 11): нужно импортировать `Module`;
- `.relu()` — метод тензора;
- `forward` возвращает **логиты**, без sigmoid. Это ключевое решение — см. ниже.

### Почему логиты, а не вероятности

Функция потерь `binary_cross_entropy_with_logit` **сама** применяет sigmoid внутри. Если применить sigmoid в `forward` и передать результат в неё, sigmoid применится дважды:

- градиент раздавится;
- loss застрянет около `0.598`;
- accuracy останется низкой.

Правило: **loss получает логиты, человек получает вероятности.**

```rust
fn probabilities(&self, x: &Tensor) -> candle_core::Result<Tensor> {
    candle_nn::ops::sigmoid(&self.forward(x)?)
}
```

---

## 4. Обучение

```rust
use candle_nn::{AdamW, Optimizer, ParamsAdamW};

fn train_xor(hidden_dim: usize, lr: f64, epochs: usize, device: &Device)
    -> candle_core::Result<(Mlp, Vec<f32>)>
{
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, device);
    let model = Mlp::new(2, hidden_dim, vb, varmap)?;

    let params = ParamsAdamW {
        lr,
        weight_decay: 0.0,          // по умолчанию 0.01 — для XOR мешает
        ..Default::default()
    };
    let mut optimizer = AdamW::new(model.varmap.all_vars(), params)?;

    let x = Tensor::new(&[[0f32, 0.], [0., 1.], [1., 0.], [1., 1.]], device)?;
    let y = Tensor::new(&[[0f32], [1.], [1.], [0.]], device)?;

    let mut losses = Vec::with_capacity(epochs);
    for _ in 0..epochs {
        let logits = model.forward(&x)?;
        let loss = candle_nn::loss::binary_cross_entropy_with_logit(&logits, &y)?;
        optimizer.backward_step(&loss)?;
        losses.push(loss.to_scalar::<f32>()?);
    }

    Ok((model, losses))
}
```

Разберём:

- `ParamsAdamW { lr, weight_decay: 0.0, ..Default::default() }` — настройки оптимизатора. `..Default::default()` заполняет остальные поля (лекция 7);
- `AdamW::new(model.varmap.all_vars(), params)` — оптимизатор получает **все** параметры из `VarMap`;
- `binary_cross_entropy_with_logit(&logits, &y)` — обрати внимание: `logit` в единственном числе, функции `bce` нет;
- `optimizer.backward_step(&loss)` — посчитать градиенты **и** обновить веса. Отдельный `backward` не нужен;
- `loss.to_scalar::<f32>()` — вытащить число из тензора-скаляра.

### Проверенные гиперпараметры

| hidden | lr | weight_decay | epochs | успех |
|---:|---:|---:|---:|---:|
| 8 | 0.1 | 0.0 | 2000 | 98/100 |
| **16** | **0.1** | **0.0** | **2000** | **100/100** |
| 8 | 0.5 | 0.0 | 5000 | 2/10 |

`hidden = 8` иногда застревает: ReLU-нейроны «умирают», loss остаётся на `≈ 0.3466`, а две вероятности — ровно `0.5`. Лечится `hidden = 16`.

`lr = 0.5` разносит обучение. Для AdamW на такой маленькой задаче рабочий диапазон — `0.05–0.2`.

---

## 5. Сохранение и загрузка весов

```rust
impl Mlp {
    fn save(&self, path: &str) -> candle_core::Result<()> {
        self.varmap.save(path)          // формат safetensors
    }

    fn load(path: &str, input_dim: usize, hidden_dim: usize, device: &Device)
        -> candle_core::Result<Self>
    {
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, DType::F32, device);
        // 1) сначала модель — она создаёт переменные
        let mut model = Self::new(input_dim, hidden_dim, vb, varmap)?;
        // 2) только теперь load: он обновляет существующие переменные
        model.varmap.load(path)?;
        Ok(model)
    }
}
```

**Порядок критичен.** `VarMap::load` не создаёт переменные, а обновляет уже существующие по именам. Если сделать наоборот (пустой `VarMap` → `load` → модель), загрузка молча ничего не сделает, и модель останется со случайными весами.

Хороший тест: загрузить файл с другим `hidden_dim` — должен получиться `Err` (несовпадение формы).

---

## 6. Воспроизводимость

```rust
device.set_seed(42)?;
```

```text
cannot seed the CPU rng with set_seed
```

В Candle 0.11.1 **`set_seed` не работает на CPU**. Поэтому детерминированные тесты делают иначе: создают модель, а потом **вручную задают веса**:

```rust
model.varmap.set_one("l1.weight", &tensor)?;
```

`set_one` требует, чтобы переменная уже существовала, — значит, модель создаётся до вызова.

Для XOR можно задать веса, которые решают задачу точно: `h1 = relu(x1 - x2)`, `h2 = relu(x2 - x1)`, `logit = 4·(h1 + h2) - 2`.

---

## 7. Типичные ошибки

### Двойной sigmoid

`forward` возвращает вероятности, loss — `binary_cross_entropy_with_logit`. Loss застревает около `0.598`.

### `load` до создания модели

Ошибки нет, но веса случайные: загружать было некуда.

### Забыли `use candle_nn::Optimizer`

```text
error[E0599]: no method named `backward_step` found for struct `AdamW`
```

Это метод трейта — его нужно импортировать.

### Забыли `use candle_nn::Module`

```text
error[E0599]: no method named `forward` found for struct `Linear`
```

### `weight_decay` по умолчанию

`ParamsAdamW::default().weight_decay == 0.01`. Если не указать `0.0`, получишь L2-регуляризацию, о которой не просил.

---

## 8. Сравнение с лекцией 26

| | MLP с нуля (26) | Candle (27) |
|---|---|---|
| Forward | вручную | `Linear` + `Module` |
| Backward | вручную, по формулам | autograd |
| Обновление весов | вручную | `backward_step` |
| Хранение параметров | свои `Vec<f64>` | `VarMap` |
| Сохранение | писать самому | safetensors |
| Воспроизводимость | свой генератор с seed | `set_seed` не работает на CPU |
| Скорость | быстрее для крошечных задач | лучше масштабируется |

Понимание, что делает autograd, — именно то, что ты получил в лекции 26.

---

## 9. Что ты теперь знаешь

- `Tensor` + `Device`; `dims()`, `matmul`, `zeros`.
- Параметры — `Var` в `VarMap`; создаются через `VarBuilder` с префиксами.
- `Module::forward` у слоёв; `relu()`, `candle_nn::ops::sigmoid`.
- Loss — `binary_cross_entropy_with_logit` от **логитов**.
- `AdamW` + `weight_decay: 0.0`; `backward_step` обновляет веса.
- `VarMap::save/load`; порядок «модель → load».
- `set_seed` на CPU не работает; в тестах — `set_one`.

---

## Проверь себя

1. Чем `Var` отличается от `Tensor`?
2. Зачем нужен `VarBuilder` и что делает `.pp("l1")`?
3. Почему `forward` возвращает логиты, а не вероятности?
4. Что делает `backward_step`?
5. Почему `VarMap::load` идёт после создания модели?
6. Почему `weight_decay: 0.0` указывают явно?
7. Как сделать детерминированный тест, если `set_seed` не работает?

---

**Дальше:** [лекция 28 — инференс и Python-расширения](../28-inference-pyo3/lecture.md).
