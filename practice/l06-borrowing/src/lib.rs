//! Заимствование: `&str`, `&mut`, `*`, срезы. Только лекции 1–6.

/// Возвращает **последнее** слово строки — срез входных данных, без копирования.
///
/// `"привет мир"` -> `"мир"`, `"привет"` -> `"привет"`, `""` -> `""`.
/// Слово начинается сразу после последнего пробела, поэтому `"привет "` -> `""`.
pub fn last_word(text: &str) -> &str {
    let mut index_of_last_spacewhite: usize = 0;
    for (i, ch) in text.char_indices() {
        if ch == ' ' {
            index_of_last_spacewhite = i;
        }
    }
    &text[index_of_last_spacewhite..].trim()
}

/// Дописывает '!' в конец строки прямо на месте.
///
/// Владение остаётся у вызывающего: функция получает `&mut String`,
/// а не `String`. Поэтому после вызова строка на месте — только длиннее.
pub fn push_bang(text: &mut String) {
    text.push('!');
}

/// Сумма чисел в готовом срезе.
///
/// Срез создаёт **вызывающий** — функция его не строит:
///
/// ```text
/// sum_slice(&array)        // массив целиком   (&[i32; 3] -> &[i32])
/// sum_slice(&array[0..2])  // кусок массива    (уже &[i32])
/// ```
pub fn sum_slice(values: &[i32]) -> i32 {
    let mut sum: i32 = 0; 
    for i in values {
        sum += i;
    }
    sum
}

/// Сколько раз символ `target` встречается в тексте.
pub fn count_char(text: &str, target: char) -> usize {
    let mut counter: usize = 0;
    for (_, ch) in text.char_indices() {
        if ch == target {
            counter +=1;
        }
    }
    counter
}

pub fn count_words(text: &str) -> usize {
    let mut counter: usize = 0;
    
    if text.trim() == ""  {
        return counter;
    }

    for (_, ch) in text.trim().char_indices() {
        if ch == ' ' {
            counter += 1;
        }
    }
    counter + 1
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

// --- Блок 2: `*` перед вызовом метода ---
// pub fn experiment_2() {
//     let text = &mut String::from("a");
//     *text.push('!');
// }

// --- Блок 3: `&mut` без mut ---
// pub fn experiment_3() {
//     let s = String::from("a");
//     let r = &mut s;
//     r.push('!');
// }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn counting_words() {
        assert_eq!(count_words("Тут три слова"), 3);
        assert_eq!(count_words("Тут два "), 2);
        assert_eq!(count_words("Одно"), 1);
        assert_eq!(count_words(" "), 0);
    }

    #[test]
    fn finds_last_word() {
        assert_eq!(last_word("привет мир"), "мир");
        assert_eq!(last_word("привет"), "привет");
        assert_eq!(last_word(""), "");
    }

    #[test]
    fn last_word_is_a_slice_not_a_copy() {
        let text = String::from("раз два");
        let word = last_word(&text);
        assert_eq!(word, "два");
        assert_eq!(word.len(), 6); // 3 кириллических символа = 6 байт
    }

    #[test]
    fn trailing_space_gives_empty_word() {
        assert_eq!(last_word("привет "), "");
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
        assert_eq!(sum_slice(&array), 6);        // массив -> срез
        assert_eq!(sum_slice(&array[0..2]), 3);  // кусок массива
        assert_eq!(sum_slice(&[]), 0);           // пустой массив
    }

    #[test]
    fn counts_char_occurrences() {
        assert_eq!(count_char("привет", 'и'), 1);
        assert_eq!(count_char("ааа б", 'а'), 3);
        assert_eq!(count_char("", 'а'), 0);
        assert_eq!(count_char("ёж", 'ё'), 1);
    }
}
