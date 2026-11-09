//! Заимствование: `&str`, `&mut`, `*`, срезы. Только лекции 1–6.

/// Возвращает первое слово строки — срез входных данных, без копирования.
///
/// `"привет мир"` -> `"привет"`, `"привет"` -> `"привет"`, `""` -> `""`.
/// Слово заканчивается на первом пробеле.
pub fn first_word(text: &str) -> &str {
    todo!()
}

/// Дописывает '!' в конец строки прямо на месте.
///
/// Владение остаётся у вызывающего: функция получает `&mut String`,
/// а не `String`. Поэтому после вызова строка на месте — только длиннее.
pub fn push_bang(text: &mut String) {
    todo!()
}

/// Сумма чисел в готовом срезе.
///
/// Срез создаёт **вызывающий**: `&array` или `&array[0..2]`.
/// Функция ничего не копирует, только читает.
pub fn sum_slice(values: &[i32]) -> i32 {
    todo!()
}

/// Сколько раз символ `target` встречается в тексте.
pub fn count_char(text: &str, target: char) -> usize {
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
    fn first_word_is_a_slice_not_a_copy() {
        let text = String::from("раз два");
        let word = first_word(&text);
        assert_eq!(word, "раз");
        // 3 символа кириллицы = 6 байт, срез «раз» тоже 6 байт
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
    fn counts_char_occurrences() {
        assert_eq!(count_char("привет", 'и'), 1);
        assert_eq!(count_char("ааа б", 'а'), 3);
        assert_eq!(count_char("", 'а'), 0);
        assert_eq!(count_char("ёж", 'ё'), 1);
    }
}
