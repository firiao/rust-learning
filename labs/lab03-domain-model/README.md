# lab03-domain-model

Каркас третьей лабораторной.

- Условие: [`../../assignments/lab03-domain-model.md`](../../assignments/lab03-domain-model.md).
- Код: [`src/lib.rs`](src/lib.rs).
- Запуск из корня `rust-learning`:

```bash
cargo test -p lab03-domain-model
```

Модель ML-эксперимента: `Sample`, `Dataset`, `Task`, `Activation`, `ModelConfig`, `Metric`. Часть методов реализована, часть — `todo!()` с `#[ignore]`-тестами.

Начни с `feature_dim`, затем `mean_label`/`label_range`, потом `Activation`, `ModelConfig`, `Metric`. Не меняй публичные сигнатуры.
