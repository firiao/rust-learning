// ЭКСПЕРИМЕНТ 1: читатель мешает писателю.
//
// Запуск:  cargo check --example e1_reader_vs_writer -p l06-borrowing
// Ожидание: error[E0502]
//
// Этот файл НЕ мешает `cargo test`: примеры собираются только по отдельному запросу.

fn main() {
    let mut s = String::from("привет");
    let r = &s;
    let w = &mut s;
    println!("{r}");
    w.push('!');
}
