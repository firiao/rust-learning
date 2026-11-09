// ЭКСПЕРИМЕНТ 2: move при передаче в функцию.
// Запуск: cargo check --example e2_move_into_fn -p l05-ownership --features experiments
// Ожидание: error[E0382], пометка `value moved here` на вызове take(s)

fn take(text: String) {
    println!("{text}");
}

fn main() {
    let s = String::from("привет");
    take(s);
    println!("{s}");
}
