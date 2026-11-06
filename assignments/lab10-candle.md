# Лабораторная 10. Тот же MLP на Candle

## Цель

Повторить MLP из lab09 на настоящем DL-фреймворке: тензоры, `Var`, autograd, слои, оптимизатор, обучение и сохранение весов. Сравнить ручной backprop с автоматическим.

## Что нужно знать до начала

- lab09 (MLP, forward/backward, BCE);
- `Result` и `?`;
- основы `ndarray`;
- желательно прочитать [`docs/05-ml-ecosystem.md`](../docs/05-ml-ecosystem.md), раздел про Candle.

## Языковой бюджет

Разрешено: `candle-core`, `candle-nn`, `Result`, `?`, структуры, трейты Candle.

Запрещено: `unsafe`, ручные градиенты (в этой лабе их считает autograd), tch/Burn в обязательной части.

## Проект

```bash
cd ~/rust-learning
cargo new --lib labs/lab10-candle
cd labs/lab10-candle
cargo add candle-core@0.11 candle-nn@0.11
```

Добавь в `members` корневого `Cargo.toml`:

```toml
"labs/lab10-candle",
```

Запуск:

```bash
cargo test -p lab10-candle
```

> Candle — `0.x`-крейт, API может меняться. Если пример из задания не компилируется, сверься с `docs.rs/candle-nn/0.11` и примерами в репозитории Candle. Это нормальная часть работы с молодым фреймворком.

> **Важно про скорость.** Candle на CPU гоняет матричные умножения через `gemm`/`rayon`. На тензорах вида `4 × 2` накладные расходы на потоки больше, чем сама работа. Если обучение XOR идёт подозрительно медленно (секунды на эпоху), запусти с одним потоком:
>
> ```bash
> RAYON_NUM_THREADS=1 cargo test -p lab10-candle
> ```
>
> На этой лабе «много потоков» — это вред, а не польза. Разбираться с параллелизмом будешь на lab07 и в реальных моделях.

---

## Подготовительная часть

1. Запусти минимальный пример:

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

2. Объясни, что такое `Device`, `Tensor`, `Result` в Candle.
3. Найди в документации:
   - `Var`;
   - `VarMap`;
   - `VarBuilder`;
   - `Module`;
   - `Optimizer`;
   - `AdamW`/`SGD`.
4. Ответь: чем `Tensor` отличается от `Var`?
5. Проверь, какие имена получают переменные у слоя `linear(2, 8, vb.pp("l1"))`:

   ```rust
   let varmap = candle_nn::VarMap::new();
   let vb = candle_nn::VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);
   let l1 = candle_nn::linear(2, 8, vb.pp("l1"))?;
   for var in varmap.all_vars() {
       println!("{} {:?}", var.as_tensor().dims(), var);
   }
   ```

   Эти имена (`l1.weight`, `l1.bias`, …) — ключи в safetensors-файле. Их полезно знать заранее: при загрузке чужих весов имена должны совпадать.

---

## Теоретическая часть

В lab09 ты считал:

```text
dz2 = p - y
dW2 = (1/n) dz2^T h
...
```

Candle делает это через **autograd**: ты строишь вычислительный граф из тензоров, а `backward` автоматически считает градиенты. Тебе нужно:

1. Создать параметры (`Var`) — они требуют градиенты.
2. Сделать forward: `Linear` → ReLU → `Linear`.
3. Посчитать loss.
4. Вызвать `backward_step` у оптимизатора.
5. Градиенты обнулять не нужно — `backward_step` в Candle делает это сам.

Архитектура та же: `2 → 8 → 1`, ReLU на скрытом слое, на выходе — **логит**, BCE с логитом как loss.

### Ключевое различие: логиты и вероятности

Это главное место, где легко испортить обучение. Разделяй три вещи:

| Что | Формула | Где используется |
|---|---|---|
| логит `z` | `z = W₂·relu(W₁x + b₁) + b₂` | выход `Linear`, вход в loss |
| вероятность `p` | `p = σ(z)` | для `predict`, метрик, вывода |
| loss | `BCEWithLogits(z, y)` | обучение |

Функция `binary_cross_entropy_with_logit` **сама применяет sigmoid внутри**. Если ты применишь sigmoid в `forward` и передашь результат в эту loss, sigmoid применится дважды:

- градиент раздавится (`σ(σ(z))` меняется очень медленно);
- loss застрянет около `0.598`;
- модель будет выглядеть «почти обученной», но accuracy останется низкой.

Правило: **loss получает логиты, человек получает вероятности.**

---

## Обязательное задание

### Структура

```text
lab10-candle/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── data.rs      ← XOR
│   ├── model.rs     ← MLP на Candle
│   └── train.rs     ← цикл обучения
└── tests/
    └── xor.rs
```

Можно и одним `lib.rs`, но модули лучше: это lab06.

### Данные

```rust
pub fn xor_data(device: &candle_core::Device) -> candle_core::Result<(Tensor, Tensor)>;
```

- `X` shape `(4, 2)`, dtype `F32`;
- `y` shape `(4, 1)`, dtype `F32`;
- значения 0/1;
- тензоры на переданном device.

### Модель

```rust
pub struct Mlp {
    // Linear-слои Candle + VarMap
}
```

Требования:

1. Два линейных слоя: `2 -> hidden` и `hidden -> 1`.
2. Скрытая активация — ReLU.
3. **`forward` возвращает логиты** shape `(n, 1)`, без sigmoid.
4. Вероятности — отдельный метод `probabilities` (sigmoid от логитов).
5. Параметры создаются через `VarMap`/`VarBuilder`, чтобы их видел оптимизатор.
6. Модель хранит свой `VarMap` (или отдаёт его наружу) — иначе нечего сохранять.
7. `loss(&self, x, y) -> Result<Tensor>` считает BCE. Используй
   **`candle_nn::loss::binary_cross_entropy_with_logit`** (обрати внимание: `logit` в единственном числе, и функция ожидает логиты, а не вероятности).
   Функции `candle_nn::loss::bce` в 0.11 **нет** — не ищи её.
8. `predict(&self, x) -> Result<Tensor>` возвращает 0.0/1.0: sigmoid + порог 0.5.

### Обучение

```rust
pub fn train_xor(
    hidden_dim: usize,
    learning_rate: f64,
    epochs: usize,
    device: &Device,
) -> Result<(Mlp, Vec<f32>)>;
```

- optimizer — `AdamW` или `SGD` (выбери и объясни);
- на каждой эпохе: forward → loss → `backward_step` → запись loss;
- loss должен убывать;
- accuracy на XOR >= 0.99 после разумного числа эпох;
- вернуть обученную модель и историю loss.

#### Проверенные гиперпараметры

Эти значения я прогнал на Candle 0.11.1: 10–100 независимых запусков на конфигурацию, у финальных — 100 (инициализация случайная; `Device::set_seed` на CPU в 0.11 не работает — см. ниже):

| hidden | lr | weight_decay | epochs | успешных запусков |
|---:|---:|---:|---:|---:|
| 8 | 0.1 | 0.01 | 2000 | 97/100 |
| 8 | 0.1 | 0.0 | 2000 | 98/100 |
| 8 | 0.1 | 0.0 | 5000 | 98/100 |
| 8 | 0.5 | 0.0 | 5000 | 2/10 |
| **16** | **0.1** | **0.0** | **2000** | **100/100** |
| 16 | 0.05 | 0.0 | 2000 | 50/50 |
| 32 | 0.1 | 0.0 | 2000 | 50/50 |

Бери **`hidden = 16`, `lr = 0.1`, `weight_decay = 0.0`, `epochs = 2000`**.

Обрати внимание на `8`: 2–3% запусков не сходятся даже с «правильным» `lr`. Именно поэтому в задании
`hidden = 16`, а не 8 — на четырёх примерах разница в цене нулевая, а в надёжности заметная.

Два предупреждения, которые сэкономят тебе вечер:

- **`weight_decay` по умолчанию `0.01`.** Если написать `ParamsAdamW { lr, ..Default::default() }`, ты незаметно включишь L2-регуляризацию. Для XOR с 4 примерами она только мешает — ставь `0.0` явно.
- **`lr = 0.5` на XOR ломает обучение** (2/10). Не «ускоряй» обучение большим шагом, пока не убедился, что базовый конфиг работает.

#### Почему `hidden = 8` иногда застревает

С ReLU часть нейронов может «умереть»: для всех четырёх примеров выход 0, градиент тоже 0, и нейрон больше никогда не оживёт. При `hidden = 8` это случается примерно в 2–3 запусках из 100, при `hidden = 16` — ни разу из 100.

Симптом: loss стабилизируется около **0.3466**, а вероятности выглядят так:

```text
probs = [[0.0], [0.5], [1.0], [0.5]]
```

`0.3466 ≈ 2·ln(2)/4` — это ровно два примера, на которых модель выдаёт `p = 0.5` (логит 0). Модель не «сломалась», она сидит в локальном минимуме. Лечится не героизмом, а `hidden = 16` и `weight_decay = 0.0`.

### Сохранение/загрузка

```rust
pub fn save(&self, path: &str) -> Result<()>;
pub fn load(path: &str, input_dim: usize, hidden_dim: usize, device: &Device) -> Result<Self>;
```

Используй `VarMap::save`/`VarMap::load` (safetensors). Проверь, что после загрузки предсказания совпадают.

#### Порядок загрузки — не перепутай

`VarMap::load` **не создаёт переменные**, а обновляет уже существующие по именам. Поэтому:

```rust
// ПРАВИЛЬНО: сначала модель (переменные созданы), потом загрузка
let varmap = VarMap::new();
let vb = VarBuilder::from_varmap(&varmap, DType::F32, device);
let mut model = Mlp::new(2, hidden_dim, vb, varmap)?;
model.varmap_mut().load(path)?;   // теперь есть куда грузить
```

```rust
// НЕПРАВИЛЬНО: пустой VarMap + load, потом модель
let mut varmap = VarMap::new();
varmap.load(path)?;               // загружать некуда — молча получишь пустоту
let model = Mlp::new(2, hidden_dim, vb, varmap)?;  // случайные веса
```

Второй вариант не падает с ошибкой — он просто тихо обучает тебя неправильному. Это типичная ловушка `VarMap`.

Хорошая проверка: загрузи файл с другим `hidden_dim` — должен получиться `Err` (несовпадение формы). Если ошибки нет, ты грузишь не туда.

### Детерминизм тестов

`Device::set_seed(42)` в Candle 0.11.1 на CPU **не работает**:

```text
cannot seed the CPU rng with set_seed
```

Поэтому тесты, которым нужен фиксированный результат, не «сеют», а задают веса руками:

```rust
model.varmap_mut().set_one("l1.weight", &tensor)?;
```

`set_one` требует, чтобы переменная уже существовала, — то есть модель нужно создать до этого вызова.

XOR можно решить руками двумя ReLU-нейронами:

```text
h1 = relu(x1 - x2)
h2 = relu(x2 - x1)
logit = 4·(h1 + h2) - 2

(0,0) → -2  → p≈0.12 → 0
(0,1) →  2  → p≈0.88 → 1
(1,0) →  2  → p≈0.88 → 1
(1,1) → -2  → p≈0.12 → 0
```

Это удобно для тестов `forward`/`predict`/save-load: они становятся полностью детерминированными и не зависят от удачи с инициализацией.

### Тесты

- `xor_data`: формы, dtype, значения;
- forward без обучения (веса заданы через `set_one`): логиты shape `(4, 1)`, вероятности в `(0, 1)`, `predict` попадает в цель;
- обучение: loss убывает, accuracy >= 0.99;
- save/load: предсказания до и после совпадают с точностью `1e-5`;
- ошибки: неверный hidden_dim при загрузке — `Err`. Реши и задокументируй, что делать с неверным `learning_rate`/`epochs` (например, `epochs = 0`).

---

## Пошаговый план

1. Создай проект, добавь Candle, собери минимальный пример.
2. Разберись с `VarMap`/`VarBuilder` на простом линейном слое; распечатай имена переменных.
3. Реализуй `xor_data`.
4. Реализуй модель: `linear(2, h)`, ReLU, `linear(h, 1)`. **`forward` возвращает логиты.**
5. Реализуй `loss` через `binary_cross_entropy_with_logit` от логитов.
6. Реализуй цикл обучения с `AdamW`, `weight_decay = 0.0`. Проверь, что loss падает.
7. Реализуй `probabilities`/`predict`/accuracy.
8. Добавь save/load в правильном порядке.
9. Запусти:

   ```bash
   RAYON_NUM_THREADS=1 cargo test -p lab10-candle
   cargo clippy -p lab10-candle --all-targets -- -D warnings
   cargo fmt -p lab10-candle -- --check
   ```

10. Напиши в `NOTES.md` сравнение с lab09.

---

## Дополнительная часть

Выбери одно или несколько:

- повтори тот же MLP на **Burn** и сравни API/скорость;
- повтори на **tch-rs** (нужен libtorch);
- обучи на датасете «две луны» вместо XOR;
- добавь второй скрытый слой;
- попробуй GPU (CUDA/Metal), если есть;
- экспортируй/импортируй ONNX через `burn-import` или `ort`;
- сравни время эпохи Candle vs ручной ndarray (не забудь `RAYON_NUM_THREADS`);
- сохрани loss в CSV и построй график;
- обучи тот же XOR с `SGD` и `lr` из разумного диапазона и сравни число эпох до сходимости.

---

## Критерии приемки

- [ ] `cargo test -p lab10-candle` проходит;
- [ ] `forward` возвращает **логиты**, sigmoid — только в `probabilities`/`predict`;
- [ ] loss — `binary_cross_entropy_with_logit` от логитов (не sigmoid + BCE и не двойной sigmoid);
- [ ] используется autograd Candle, а не ручные градиенты;
- [ ] loss убывает, XOR accuracy >= 0.99;
- [ ] есть save/load, загрузка идёт **после** создания модели, есть тест на совпадение предсказаний;
- [ ] `cargo clippy -p lab10-candle --all-targets -- -D warnings` без предупреждений;
- [ ] `cargo fmt --check` проходит;
- [ ] в `NOTES.md` есть сравнение с lab09: что autograd взял на себя, что осталось.

---

## На защите

1. Чем `Tensor` отличается от `Var`?
2. Что такое `VarMap` и зачем он нужен оптимизатору?
3. Как Candle понимает, по каким параметрам считать градиенты?
4. Что делает `backward_step`? Кто обнуляет старые градиенты?
5. Чем `AdamW` отличается от обычного SGD? Что такое `weight_decay` и почему `0.01` по умолчанию может помешать?
6. Почему почти все функции возвращают `Result`?
7. Что такое device и как переключиться на GPU?
8. Как сохраняются веса? Что такое safetensors?
9. Почему `forward` возвращает логиты, а не вероятности? Что будет, если применить sigmoid дважды?
10. Почему `VarMap::load` не создаёт переменные? Что произойдёт, если нарушить порядок?
11. Почему `Device::set_seed` не сработал на CPU и как ты сделал тесты детерминированными?
12. Сравни код lab09 и lab10: где меньше шансов ошибиться и почему?
13. Почему для обучения LLM одного Candle может быть недостаточно?

---

## Подсказки (открывать после попытки)

<details>
<summary>Каркас модели (проверено на 0.11.1)</summary>

```rust
use candle_core::{DType, Device, Result, Tensor};
use candle_nn::{linear, Linear, Module, VarBuilder, VarMap};

pub struct Mlp {
    l1: Linear,
    l2: Linear,
    varmap: VarMap,
}

impl Mlp {
    pub fn new(
        input_dim: usize,
        hidden_dim: usize,
        vb: VarBuilder,
        varmap: VarMap,
    ) -> Result<Self> {
        let l1 = linear(input_dim, hidden_dim, vb.pp("l1"))?;
        let l2 = linear(hidden_dim, 1, vb.pp("l2"))?;
        Ok(Self { l1, l2, varmap })
    }

    /// Логиты, shape (n, 1). Sigmoid здесь НЕ применяется.
    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let h = self.l1.forward(x)?.relu()?;
        self.l2.forward(&h)
    }

    /// Вероятности, shape (n, 1).
    pub fn probabilities(&self, x: &Tensor) -> Result<Tensor> {
        candle_nn::ops::sigmoid(&self.forward(x)?)
    }

    /// 0.0/1.0, shape (n, 1).
    pub fn predict(&self, x: &Tensor) -> Result<Tensor> {
        self.probabilities(x)?.ge(0.5)?.to_dtype(DType::F32)
    }

    pub fn varmap_mut(&mut self) -> &mut VarMap {
        &mut self.varmap
    }
}
```

</details>

<details>
<summary>Каркас обучения (проверено на 0.11.1)</summary>

```rust
use candle_core::{DType, Device, Result};
use candle_nn::{AdamW, Optimizer, ParamsAdamW, VarBuilder, VarMap};

use crate::data::xor_data;
use crate::model::Mlp;

pub fn train_xor(
    hidden_dim: usize,
    learning_rate: f64,
    epochs: usize,
    device: &Device,
) -> Result<(Mlp, Vec<f32>)> {
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, device);
    let model = Mlp::new(2, hidden_dim, vb, varmap)?;

    let params = ParamsAdamW {
        lr: learning_rate,
        weight_decay: 0.0, // по умолчанию 0.01 — для XOR это мешает
        ..Default::default()
    };
    let mut optimizer = AdamW::new(model.varmap().all_vars(), params)?;

    let (x, y) = xor_data(device)?;
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

</details>

<details>
<summary>Каркас save/load (проверено на 0.11.1)</summary>

```rust
use candle_core::{DType, Device, Result};

impl Mlp {
    pub fn save(&self, path: &str) -> Result<()> {
        self.varmap.save(path)
    }

    pub fn load(path: &str, input_dim: usize, hidden_dim: usize, device: &Device) -> Result<Self> {
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, DType::F32, device);
        // 1) сначала модель — она создаёт переменные
        let mut model = Self::new(input_dim, hidden_dim, vb, varmap)?;
        // 2) только теперь load, он обновляет существующие переменные
        model.varmap.load(path)?;
        Ok(model)
    }
}
```

</details>

<details>
<summary>Что реально ломает обучение XOR</summary>

Проверено экспериментально на 0.11.1:

1. **Двойной sigmoid.** `forward` возвращает `σ(z)`, а loss получает `binary_cross_entropy_with_logit` — sigmoid применяется второй раз. Loss стабилизируется около `0.598`, accuracy низкая.
2. **`weight_decay = 0.01` по умолчанию.** Работает, но на грани: с `hidden = 8` заметно чаще ловит локальный минимум.
3. **`hidden = 8` + неудачная инициализация.** Loss застревает на `0.3466`, две вероятности ровно `0.5`. Лечится `hidden = 16`.
4. **`lr = 0.5`.** Разносит обучение: 2/10 успешных запусков. Для AdamW на такой маленькой задаче `0.05–0.2` — рабочий диапазон.
5. **`RAYON_NUM_THREADS` по умолчанию.** Не ломает обучение, но на тензорах `4 × 2` может замедлить в десятки раз.

</details>
