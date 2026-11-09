// ЭКСПЕРИМЕНТ 2: `*` перед вызовом метода.
//
// Запуск:  cargo check --example e2_deref_method -p l06-borrowing
// Ожидание: error[E0614]: type `()` cannot be dereferenced
//
// Разбор: `*text.push('!')` разбирается как `*(text.push('!'))`,
// потому что вызов метода связывает крепче унарного `*`.

fn main() {
    let text = &mut String::from("a");
    *text.push('!');
}
