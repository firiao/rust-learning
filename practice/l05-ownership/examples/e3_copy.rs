// ЭКСПЕРИМЕНТ 3: Copy — ошибки НЕТ.
// Запуск: cargo run --example e3_copy -p l05-ownership --features experiments

fn main() {
    let a = 42;
    let b = a;
    println!("{a} {b}");
}
