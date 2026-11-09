// Эксперимент: `len()` считает байты, `chars().count()` — символы.
//
// Запуск: cargo run --example e1_bytes_vs_chars -p l02-text

fn main() {
    for text in ["привет", "hello", "ёж"] {
        println!("{text:?}: len = {}, chars = {}", text.len(), text.chars().count());
    }
}
