// ЭКСПЕРИМЕНТ 1: move при присваивании.
// Запуск: cargo check --example e1_move_assign -p l05-ownership --features experiments
// Ожидание: error[E0382]: borrow of moved value: `s1`

fn main() {
    let s1 = String::from("привет");
    let s2 = s1;
    println!("{s1}");
    println!("{s2}");
}
