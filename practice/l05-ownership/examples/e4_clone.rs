// ЭКСПЕРИМЕНТ 4: clone — ошибки НЕТ.
// Запуск: cargo run --example e4_clone -p l05-ownership --features experiments

fn main() {
    let s1 = String::from("привет");
    let s2 = s1.clone();
    println!("{s1} {s2}");
}
