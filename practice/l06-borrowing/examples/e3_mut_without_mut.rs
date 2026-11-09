// ЭКСПЕРИМЕНТ 3: `&mut` у переменной без `mut`.
//
// Запуск:  cargo check --example e3_mut_without_mut -p l06-borrowing
// Ожидание: error[E0596] + help: consider changing this to be mutable

fn main() {
    let s = String::from("a");
    let r = &mut s;
    r.push('!');
}
