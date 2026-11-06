# practice/ — рабочие места для лаб

Каждая лаба курса создаёт здесь свой Cargo-проект:

```bash
cd ~/rust-learning
cargo new --lib practice/l04-ownership
cargo test -p l04-ownership
```

Все папки `practice/*` уже включены в корневой workspace через glob, поэтому:

- отдельно добавлять крейт в `Cargo.toml` не нужно;
- имя пакета = имя папки (`l04-ownership`);
- `cargo test --workspace` проверяет и твои лабы тоже.

`00-sandbox` — постоянная песочница: код без задания, можно ломать.

> Если `cargo` ругается на твой крейт — это он и есть, твой код. Так и должно быть.
