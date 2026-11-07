#![allow(clippy::manual_is_multiple_of)]
// `is_multiple_of` не объяснялся; используем `%`.
//! Коллекции: `Vec`, `HashMap`, разбор CSV.

use std::collections::HashMap;

/// Частоты слов: в нижнем регистре, разделитель — любой не-буквенный символ.
pub fn word_frequencies(text: &str) -> HashMap<String, usize> {
    todo!()
}

/// Топ-N слов: по частоте (убывание), при равенстве — по алфавиту.
pub fn top_words(freq: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    todo!()
}

/// Медиана. Пустой срез -> None.
pub fn median(values: &[f64]) -> Option<f64> {
    todo!()
}

/// Разбирает одну строку CSV: "1.5,2.0,3" -> [1.5, 2.0, 3.0].
/// Что не разобралось — пропускается.
pub fn parse_line(line: &str) -> Vec<f64> {
    todo!()
}

/// Разбирает текст в таблицу чисел, пропуская пустые строки.
pub fn parse_csv(text: &str) -> Vec<Vec<f64>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
