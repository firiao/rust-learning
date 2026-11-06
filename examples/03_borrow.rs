// 03. Borrowing и срезы.
//
// Запуск:
//   rustc --edition 2024 examples/03_borrow.rs -o /tmp/borrow
//   /tmp/borrow
//
// Темы: &T, &mut T, NLL, срезы, &str, &[T], lifetimes.

fn main() {
    // Неизменяемые ссылки: сколько угодно одновременно.
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("r1 = {r1}, r2 = {r2}");

    // Изменяемая ссылка: ровно одна, без одновременных чтений.
    let mut s2 = String::from("world");
    let m = &mut s2;
    m.push_str("!");
    println!("m = {m}");

    // Область изменяемого заимствования можно ограничить блоком.
    let mut s3 = String::from("abc");
    {
        let m = &mut s3;
        m.push_str("def");
    } // заимствование закончилось
    println!("s3 = {s3}");

    // NLL: borrow живёт до последнего использования.
    let mut v = vec![1, 2, 3];
    {
        let first = &mut v[0];
        *first += 10;
    }
    v.push(4);
    println!("v = {v:?}");

    // Срез массива.
    let arr = [10, 20, 30, 40, 50];
    let slice: &[i32] = &arr[1..4];
    println!("slice = {slice:?}, len = {}", slice.len());

    // Срез строки — это &str. Индексы в байтах, не в символах!
    let sentence = String::from("Привет, мир!");
    let first = first_word(&sentence);
    println!("первое слово: {first}");

    // Ссылка на часть строки. "Привет" — 6 символов, но 12 байт в UTF-8.
    let hello = &sentence[..12];
    println!("hello = {hello}");

    // Функция, которая только читает, берёт &str.
    let len = byte_len(&sentence);
    println!("byte_len = {len}");

    // Возврат владения вместо ссылки на локальную переменную.
    let owned = make_greeting("Rust");
    println!("{owned}");

    // Упражнение:
    // 1. Попробуй одновременно взять &s3 и &mut s3 — прочитай ошибку.
    // 2. Сделай sentence изменяемой и вызови sentence.clear() до первого слова.
    //    Почему это может не скомпилироваться?
    // 3. Напиши функцию longest(left, right) с lifetime 'a.
}

fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn byte_len(text: &str) -> usize {
    text.len()
}

fn make_greeting(name: &str) -> String {
    format!("Привет, {name}!")
}
