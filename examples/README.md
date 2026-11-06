# Примеры

Девять самостоятельных файлов. Каждый компилируется отдельно через `rustc` и не входит в Cargo-workspace.

```bash
cd ~/rust-learning
./run_examples.sh
```

Или вручную:

```bash
rustc --edition 2024 examples/01_hello.rs -o /tmp/hello
/tmp/hello
```

| Файл | Темы | Когда смотреть |
|---|---|---|
| `01_hello.rs` | `main`, переменные, `mut`, shadowing, функции | до lab01 |
| `02_ownership.rs` | move, `Copy`, `Clone`, владение в функциях | lab02 |
| `03_borrow.rs` | `&T`, `&mut T`, NLL, срезы, lifetimes | lab02 |
| `04_structs_enums.rs` | struct, enum, `impl`, `Option`, match | lab03 |
| `05_match.rs` | match, guards, `if let`, `while let`, let-else | lab03 |
| `06_errors.rs` | `Result`, `?`, свои ошибки, `From` | lab05 |
| `07_traits.rs` | traits, generics, `dyn Trait`, `Add`, `Display` | lab04 |
| `08_collections.rs` | `Vec`, `HashMap`, итераторы, замыкания | lab05 |
| `09_threads.rs` | потоки, каналы, `Arc<Mutex<T>>`, scoped threads | lab07 |

Примеры — не замена лабораторным. Это «живая шпаргалка»: читай, запускай, ломай, меняй.
