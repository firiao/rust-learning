# Лабораторная 05. Ошибки, итераторы, замыкания

## Цель

Научиться возвращать ошибки через `Result<T, E>`, применять `?`, писать свои типы ошибок и обрабатывать данные итераторами и замыканиями.

## Что нужно знать до начала

- lab01–lab04;
- `Option`, `match`, структуры, трейты;
- базовые итераторы (`for`, `.iter()`);
- желательно прочитать главу про `Result` в официальной книге.

## Языковой бюджет

Разрешено: `Result<T, E>`, `?`, `From`, `Box<dyn Error>`, `impl Iterator`, `map`/`filter`/`zip`/`enumerate`/`collect`/`fold`, замыкания, `Display`, `std::error::Error`.

Запрещено: `unsafe`, async, внешние крейты (кроме `thiserror` в дополнительной части), `unwrap()` в библиотечной логике.

## Проект

```bash
cd ~/rust-learning
cargo new --lib labs/lab05-dataset-parser
```

Добавь в `members` корневого `Cargo.toml`:

```toml
"labs/lab05-dataset-parser",
```

Запуск:

```bash
cargo test -p lab05-dataset-parser
```

---

## Подготовительная часть

1. Напиши в отдельном файле функцию:

   ```rust
   fn parse_positive(text: &str) -> Result<u32, String> {
       let value: i64 = text.parse().map_err(|_| format!("не число: {text}"))?;
       if value < 0 {
           return Err(format!("отрицательное число: {value}"));
       }
       Ok(value as u32)
   }
   ```

   Объясни, что делает `?` и почему `map_err` нужен.

2. Напиши итераторную цепочку, которая считает сумму квадратов чётных чисел от 1 до 10.

3. Ответь: чем `panic!` отличается от `Result`? Когда допустим каждый?

4. Создай проект и убедись, что `cargo test -p lab05-dataset-parser` проходит (тестов пока нет).

---

## Обязательное задание

### Типы

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Sample {
    pub features: Vec<f64>,
    pub label: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dataset {
    pub samples: Vec<Sample>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DatasetError {
    Empty,
    WrongColumnCount { line: usize, expected: usize, actual: usize },
    ParseFloat { line: usize, column: usize, value: String },
}
```

### `DatasetError`

Реализуй:

- `std::fmt::Display` — человекочитаемое сообщение с номером строки/колонки;
- `std::error::Error` (можно пустой impl, если нет source).

### `parse_dataset`

```rust
pub fn parse_dataset(input: &str) -> Result<Dataset, DatasetError>;
```

Формат: каждая непустая строка — `feature1,feature2,...,label`.

Контракт:

1. Пустой ввод или ввод только из пробелов/переводов строк → `DatasetError::Empty`.
2. Пустые строки и строки из пробелов игнорируются.
3. Поддерживаются `\n` и `\r\n`.
4. Первая непустая строка задаёт количество колонок. Во всех строках должно быть столько же колонок.
5. Колонок должно быть **минимум 2** (хотя бы один признак + метка).
6. Значения обрезаются от пробелов по краям и парсятся как `f64`.
7. Последняя колонка — метка, остальные — признаки.
8. Если значение не парсится — `ParseFloat` с номером строки, номером колонки и исходной строкой значения.
9. Если колонок меньше/больше ожидаемого — `WrongColumnCount` с номером строки, ожидаемым и фактическим количеством.
10. Если в строке меньше двух колонок — тоже `WrongColumnCount` (expected — значение первой строки, либо 2, если это первая строка).
11. Парсер не паникует и не использует `unwrap()`.

#### Нумерация строк и колонок

Это место, где легко ошибиться незаметно, поэтому контракт жёсткий:

- **строка** — номер физической строки во входной строке, 1-based, **с учётом пустых строк**;
- **колонка** — номер поля в строке, 1-based, считается **после** разделения по `,`, до обрезки пробелов.

То есть `line` в ошибке должен совпадать с номером строки, который ты увидишь в редакторе, открыв этот CSV. Пустые строки нумерацию **не** сдвигают.

```text
файл:                      ошибка:
1: 1,2,3                   ParseFloat { line: 4, column: 2, .. }
2:                        ← пустая строка, пропускается, но линия считается
3: 4,5,6
4: 7,oops,9
```

Проверка: если вставить пустую строку в начало файла, `line` в ошибке должен увеличиться на 1. Если он не изменился — ты считаешь непустые строки, а не физические.

Пример:

```rust
let dataset = parse_dataset("1,2,3\n4,5,6\n").unwrap();
assert_eq!(dataset.samples.len(), 2);
assert_eq!(dataset.samples[0].features, vec![1.0, 2.0]);
assert_eq!(dataset.samples[0].label, 3.0);
```

### `Dataset`

```rust
impl Dataset {
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn feature_dim(&self) -> Option<usize>;
    pub fn mean_feature(&self, index: usize) -> Option<f64>;
    pub fn mean_label(&self) -> Option<f64>;
    pub fn label_range(&self) -> Option<(f64, f64)>;
}
```

Контракт:

1. `feature_dim`: пустой датасет → `None`; если размерности разные → `None`; иначе `Some(dim)`.
2. `mean_feature`: пустой датасет или `index >= dim` → `None`; иначе среднее по признаку.
3. `mean_label`, `label_range`: пустой датасет → `None`.
4. Методы не меняют датасет.

### `mse`

```rust
pub fn mse(y_true: &[f64], y_pred: &[f64]) -> Option<f64>;
```

- пустые срезы или разная длина → `None`;
- иначе среднее `(t - p)^2`.

### Тесты

Добавь тесты:

- пустой ввод, только пробелы;
- одна строка, несколько строк;
- `\r\n`;
- пустые строки между данными **и проверка, что нумерация в ошибках физическая** (вставь пустую строку в начало и убедись, что `line` вырос на 1);
- меньше двух колонок;
- разное число колонок;
- нечисловое значение (проверь номер строки/колонки);
- отрицательные и дробные значения;
- `feature_dim`, `mean_feature`, `mean_label`, `label_range`;
- `mse` на известных значениях, пустые срезы, разная длина.

---

## Пошаговый план

1. Определи типы и `DatasetError`.
2. Реализуй `Display` для ошибки. Сначала простыми `match`-ветками.
3. Реализуй `parse_dataset`:
   - **сначала** пронумеруй физические строки: `.lines().enumerate()` даёт `(index, line)`, номер строки — `index + 1`;
   - потом отбрось пустые: `map(str::trim)`, `filter(|s| !s.is_empty())` — но номер строки сохрани;
   - определи `expected` по первой непустой строке;
   - для каждой строки раздели по `,`, проверь количество;
   - распарси значения; при ошибке верни `ParseFloat` с сохранённым физическим номером;
   - собери `Sample`.
4. Реализуй методы `Dataset`.
5. Реализуй `mse` через `zip`, `map`, `sum`.
6. Напиши тесты, включая ошибки.
7. Запусти:

   ```bash
   cargo test -p lab05-dataset-parser
   cargo clippy -p lab05-dataset-parser -- -D warnings
   cargo fmt
   ```

---

## Дополнительная часть

Выбери одно или несколько:

- поддержка заголовка: если первая строка не парсится как числа, считать её заголовком и пропустить; добавь `parse_dataset_with_header`;
- `impl std::str::FromStr for Dataset` с `type Err = DatasetError`;
- `Dataset::standardize(&self) -> Option<Dataset>` — `(x - mean) / std` по каждому признаку; продумай `std == 0`;
- `split_train_test(&self, ratio: f64) -> Option<(Dataset, Dataset)>`;
- `Dataset::iter(&self) -> impl Iterator<Item = &Sample>`;
- замени ручной `DatasetError` на `thiserror` и сравни количество кода.

---

## Критерии приемки

- [ ] `cargo test -p lab05-dataset-parser` проходит;
- [ ] в библиотечной логике нет `unwrap()`/`expect()` на пользовательских данных;
- [ ] `DatasetError` реализует `Display` и `Error`;
- [ ] парсер корректно сообщает строку и колонку ошибки;
- [ ] используются итераторы/замыкания, а не только ручные индексы;
- [ ] `cargo clippy -p lab05-dataset-parser -- -D warnings` без предупреждений;
- [ ] `cargo fmt --check` проходит.

---

## На защите

1. Чем `Result<T, E>` лучше исключений? В чём уступает Python/Java?
2. Что делает `?`? Какое преобразование ошибки он применяет?
3. Зачем нужен `From<E>` для своего типа ошибки?
4. Чем `Box<dyn Error>` отличается от конкретного `DatasetError`?
5. Когда `panic!` уместен, а когда нет?
6. Почему итераторы ленивы? Где начинается вычисление?
7. Чем `map` отличается от `filter` и `fold`?
8. Как собрать `Vec<Result<T, E>>` в `Result<Vec<T>, E>`?
9. Почему `zip` удобен для метрик?
10. Как бы ты добавил поддержку CSV с заголовком, не ломая текущий API?

---

## Подсказки (открывать после попытки)

<details>
<summary>Display для DatasetError</summary>

```rust
impl std::fmt::Display for DatasetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatasetError::Empty => write!(f, "датасет пуст"),
            DatasetError::WrongColumnCount { line, expected, actual } => write!(
                f,
                "строка {line}: ожидалось колонок {expected}, получено {actual}"
            ),
            DatasetError::ParseFloat { line, column, value } => write!(
                f,
                "строка {line}, колонка {column}: не удалось разобрать {value:?} как число"
            ),
        }
    }
}

impl std::error::Error for DatasetError {}
```

</details>

<details>
<summary>Каркас парсера</summary>

```rust
pub fn parse_dataset(input: &str) -> Result<Dataset, DatasetError> {
    // 1) Физическая нумерация: номер строки = index + 1.
    //    Пустые строки ещё не отброшены — поэтому номера настоящие.
    let numbered: Vec<(usize, &str)> = input
        .lines()
        .enumerate()
        .map(|(index, line)| (index + 1, line))
        .collect();

    // 2) Первая непустая строка задаёт количество колонок.
    let Some((first_line_number, first)) = numbered
        .iter()
        .map(|(number, line)| (*number, line.trim()))
        .find(|(_, line)| !line.is_empty())
    else {
        return Err(DatasetError::Empty);
    };

    let expected = first.split(',').count();
    if expected < 2 {
        return Err(DatasetError::WrongColumnCount {
            line: first_line_number,
            expected: 2,
            actual: expected,
        });
    }

    let mut samples = Vec::new();
    for &(line_number, raw) in &numbered {
        let line = raw.trim();
        if line.is_empty() {
            continue; // пропускаем, но номер строки уже зафиксирован
        }

        let columns: Vec<&str> = line.split(',').map(str::trim).collect();

        if columns.len() != expected {
            return Err(DatasetError::WrongColumnCount {
                line: line_number,
                expected,
                actual: columns.len(),
            });
        }

        let mut values = Vec::with_capacity(columns.len());
        for (column_index, value) in columns.iter().enumerate() {
            let parsed = value.parse::<f64>().map_err(|_| DatasetError::ParseFloat {
                line: line_number,
                column: column_index + 1,
                value: (*value).to_string(),
            })?;
            values.push(parsed);
        }

        let label = values.pop().ok_or(DatasetError::WrongColumnCount {
            line: line_number,
            expected,
            actual: columns.len(),
        })?;
        samples.push(Sample { features: values, label });
    }

    Ok(Dataset { samples })
}
```

Главное здесь — **порядок операций**: `enumerate()` стоит до `filter`/`continue`. Если сначала отфильтровать пустые строки, а потом нумеровать, номера будут врать на всех файлах, где есть пропуски. Ошибка тихая: компилятор её не поймает, тесты с «плотным» CSV тоже.

`ok_or` превращает `Option` в `Result` без паники. Ветка ошибки здесь недостижима из-за проверки `expected >= 2`, но компилятор этого не знает — и это нормально.

</details>
