# lab01-toolkit

Каркас первой лабораторной.

- Условие: [`../../assignments/lab01-toolkit.md`](../../assignments/lab01-toolkit.md).
- Код: [`src/lib.rs`](src/lib.rs).
- Запуск из корня `rust-learning`:

```bash
cargo test -p lab01-toolkit
```

Сейчас:

- `is_even` реализована, её тесты проходят;
- `is_prime`, `is_palindrome`, `average` — заглушки `todo!()`;
- тесты для заглушек помечены `#[ignore]`, поэтому стартовая сборка зелёная.

Порядок:

1. Прочитай условие.
2. Реализуй функции.
3. Снимай `#[ignore]` по мере готовности.
4. Добавь свои тесты границ.
5. `cargo clippy -p lab01-toolkit -- -D warnings` и `cargo fmt`.

Не меняй сигнатуры функций.
