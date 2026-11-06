# lab02-ownership

Каркас второй лабораторной.

- Условие: [`../../assignments/lab02-ownership.md`](../../assignments/lab02-ownership.md).
- Код: [`src/lib.rs`](src/lib.rs).
- Запуск из корня `rust-learning`:

```bash
cargo test -p lab02-ownership
```

Сейчас реализованы `count_words`, `shout_in_place`, `sum`, `consume_and_len`, `greet`. Нужно реализовать `first_word`, `longest`, `split_even_odd`; тесты для них помечены `#[ignore]`.

Главное правило: не использовать `clone()` в реализациях. Если очень хочется — сначала объясни, какое владение/заимствование не сходится.
