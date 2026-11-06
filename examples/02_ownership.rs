// 02. Ownership: move, Copy, Clone.
//
// Запуск:
//   rustc --edition 2024 examples/02_ownership.rs -o /tmp/ownership
//   /tmp/ownership
//
// Темы: владение, перемещение, копирование, заимствование, время жизни.

fn main() {
    // i32 — Copy: копируется побитово, обе переменные живы.
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // String владеет данными в куче. Присваивание перемещает владение.
    let s1 = String::from("hello");
    let s2 = s1; // move
    // println!("{s1}"); // ошибка E0382: borrow of moved value
    println!("s2 = {s2}");

    // Явная глубокая копия.
    let s3 = s2.clone();
    println!("s2 = {s2}, s3 = {s3}");

    // Передача в функцию перемещает владение.
    let s4 = String::from("Rust");
    take_ownership(s4);
    // println!("{s4}"); // ошибка: s4 перемещена

    // Возврат владения.
    let s5 = String::from("back");
    let s5 = give_back(s5);
    println!("s5 = {s5}");

    // Ссылка не перемещает владение.
    let s6 = String::from("borrow");
    print_borrowed(&s6);
    println!("s6 всё ещё наш: {s6}");

    // Строковые литералы — &'static str.
    let literal = "я скомпилирован в бинарник";
    println!("{literal}");

    // Владение в коллекциях.
    let mut v = Vec::new();
    v.push(String::from("a"));
    let first = v.remove(0); // владение строкой перешло в first
    println!("first = {first}, v = {v:?}");

    // Упражнение: убери комментарий у println!("{s1}") и прочитай ошибку.
    // Затем исправь код, не используя clone().
}

fn take_ownership(s: String) {
    println!("забрали владение: {s}");
} // здесь s уничтожается, память освобождается

fn give_back(s: String) -> String {
    s
}

fn print_borrowed(s: &str) {
    println!("одолжили: {s}");
}
