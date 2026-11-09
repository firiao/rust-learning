//! Спека лабы 13. Эти тесты — твоё ТЗ: читай их и пиши код в src/lib.rs.

use l13_collections::*;

#[test]
fn counts_words() {
    let freq = word_frequencies("Привет, мир! Привет");
    assert_eq!(freq.get("привет"), Some(&2));
    assert_eq!(freq.get("мир"), Some(&1));
    assert_eq!(freq.len(), 2);
}

#[test]
fn ranks_top_words() {
    let freq = word_frequencies("a b b c c c d");
    assert_eq!(
        top_words(&freq, 2),
        vec![("c".to_string(), 3), ("b".to_string(), 2)]
    );
}

#[test]
fn ties_are_alphabetical() {
    let freq = word_frequencies("б а в");
    let top = top_words(&freq, 3);
    assert_eq!(top[0].0, "а");
    assert_eq!(top[1].0, "б");
    assert_eq!(top[2].0, "в");
}

#[test]
fn computes_median() {
    assert_eq!(median(&[3.0, 1.0, 2.0]), Some(2.0));
    assert_eq!(median(&[4.0, 1.0, 3.0, 2.0]), Some(2.5));
    assert_eq!(median(&[]), None);
}

#[test]
fn parses_line() {
    assert_eq!(parse_line("1.5,2.0,3"), vec![1.5, 2.0, 3.0]);
    assert_eq!(parse_line("1,x,3"), vec![1.0, 3.0]);
    assert!(parse_line("").is_empty());
}

#[test]
fn parses_csv_table() {
    let table = parse_csv("1.0,2.0\n3.0,4.0\n\n5.0,6.0\n");
    assert_eq!(table, vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]]);
    assert!(parse_csv("   \n\n").is_empty());
}
