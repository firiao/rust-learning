// 08. Коллекции, итераторы, замыкания.
//
// Запуск:
//   rustc --edition 2024 examples/08_collections.rs -o /tmp/collections
//   /tmp/collections
//
// Темы: Vec, HashMap, entry API, map/filter/collect/fold, замыкания.

use std::collections::HashMap;

fn main() {
    let mut numbers = vec![3, 1, 4, 1, 5];
    numbers.push(9);
    numbers.sort();
    println!("numbers = {numbers:?}");
    println!("sum = {}", numbers.iter().sum::<i32>());

    let evens: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|value| value % 2 == 0)
        .collect();
    println!("evens = {evens:?}");

    // HashMap: считаем слова.
    let text = "я люблю rust и rust любит меня";
    let mut counts: HashMap<&str, usize> = HashMap::new();

    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }

    let mut pairs: Vec<(&str, usize)> = counts.into_iter().collect();
    pairs.sort_by(|left, right| left.0.cmp(right.0));

    for (word, count) in &pairs {
        println!("{word}: {count}");
    }
    println!("уникальных слов: {}", pairs.len());

    // Цепочка итераторов: сумма квадратов чётных чисел.
    let result: i32 = (1..=10)
        .filter(|value| value % 2 == 0)
        .map(|value| value * value)
        .sum();
    println!("сумма квадратов чётных 1..=10 = {result}");

    // Замыкание захватывает threshold по ссылке.
    let threshold = 3;
    let big: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|&value| value > threshold)
        .collect();
    println!("big = {big:?}");

    // map + collect в Vec<String>.
    let words: Vec<String> = text
        .split_whitespace()
        .map(|word| word.to_uppercase())
        .collect();
    println!("words = {words:?}");

    // fold: ручная свёртка.
    let product = numbers.iter().fold(1, |acc, value| acc * *value);
    println!("product = {product}");

    // Упражнение: посчитай частоты букв в строке через HashMap<char, usize>.
}
