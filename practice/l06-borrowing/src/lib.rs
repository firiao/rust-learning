//! Заимствование: `&str`, `&mut`, срезы. Только лекции 1–6 (без `Vec` и итераторов).

/// Первое слово строки. Если пробелов нет — вся строка.
///
/// Возвращает срез входных данных, а не новый String.
pub fn first_word(text: &str) -> &str {
    todo!()
}

/// Дописывает '!' на место, не забирая владение.
pub fn push_bang(text: &mut String) {
    todo!()
}

/// Сумма среза. Работает и с массивом, и с вектором `Vec` (лекция 13, но сигнатура та же).
pub fn sum_slice(values: &[i32]) -> i32 {
    todo!()
}

/// Сколько гласных (русских и латинских) в тексте.
pub fn count_vowels(text: &str) -> usize {
    todo!()
}

// ===== ЭКСПЕРИМЕНТЫ =====

// --- Блок 1: читатель мешает писателю ---
// pub fn experiment_1() {
//     let mut s = String::from("привет");
//     let r = &s;
//     let w = &mut s;
//     println!("{r}");
//     w.push('!');
// }

// --- Блок 2: два писателя ---
// pub fn experiment_2() {
//     let mut s = String::from("a");
//     let a = &mut s;
//     let b = &mut s;
//     a.push('1');
//     b.push('2');
// }

// --- Блок 3: &mut без mut ---
// pub fn experiment_3() {
//     let s = String::from("a");
//     let r = &mut s;
//     r.push('!');
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_first_word() {
        assert_eq!(first_word("привет мир"), "привет");
        assert_eq!(first_word("привет"), "привет");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn first_word_is_a_slice() {
        let text = String::from("раз два");
        let word = first_word(&text);
        assert_eq!(word, "раз");
        assert_eq!(word.len(), 6);
    }

    #[test]
    fn mutates_in_place() {
        let mut text = String::from("привет");
        push_bang(&mut text);
        push_bang(&mut text);
        assert_eq!(text, "привет!!");
    }

    #[test]
    fn sums_any_slice() {
        let array = [1, 2, 3];
        assert_eq!(sum_slice(&array), 6);
        assert_eq!(sum_slice(&array[0..2]), 3);
        assert_eq!(sum_slice(&[]), 0);
    }

    #[test]
    fn counts_vowels() {
        assert_eq!(count_vowels("привет"), 2);
        assert_eq!(count_vowels("ёж"), 1);
        assert_eq!(count_vowels("Rust"), 1);
        assert_eq!(count_vowels(""), 0);
    }
}
