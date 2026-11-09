//! Move, Copy и clone. Только то, что было в лекциях 1–5 (без `Vec`).

/// Забирает владение, дописывает '!' и возвращает обратно.
pub fn append_bang(mut text: String) -> String {
    text.push('!');
    text
}

/// Дописывает два '!' — вызвать `push` дважды.
pub fn append_twice(mut text: String) -> String {
    text.push_str("!!");
    text
}

/// Copy-тип: принимает по значению, но вызывающий не теряет переменную.
pub fn double(n: i32) -> i32 {
    n * 2
}

/// Самое длинное слово (по числу символов). При равенстве — первое.
pub fn longest_word_owned(text: &str) -> String {
    let mut best_length = 0;
    let mut best_length_word = String::from("");
    for word in text.split_whitespace() {
        let word_length = word.chars().count();
        if word_length > best_length {
            best_length = word_length;
            best_length_word = String::from(word); 
        }
    }
    best_length_word
}

// ===== ЭКСПЕРИМЕНТЫ =====
// Раскомментируй ОДИН блок, выполни `cargo check -p l05-ownership`,
// прочитай ошибку, закомментируй обратно.

// --- Блок 1: move при присваивании ---
// pub fn experiment_1() {
//     let s1 = String::from("привет");
//     let s2 = s1;
//     println!("{s1}");
//     println!("{s2}");
// }

// --- Блок 2: move при передаче в функцию ---
// pub fn experiment_2() {
//     fn take(text: String) {
//         println!("{text}");
//     }
//     let s = String::from("привет");
//     take(s);
//     println!("{s}");
// }

// --- Блок 3: Copy ---
// pub fn experiment_3() {
//     let a = 42;
//     let b = a;
//     println!("{a} {b}");
// }

// --- Блок 4: clone ---
// pub fn experiment_4() {
//     let s1 = String::from("привет");
//     let s2 = s1.clone();
//     println!("{s1} {s2}");
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_bang() {
        let greeting = String::from("привет");
        let shouted = append_bang(greeting);
        assert_eq!(shouted, "привет!");
    }

    #[test]
    fn ownership_can_be_returned() {
        let mut text = String::from("a");
        text = append_bang(text);
        text = append_twice(text);
        assert_eq!(text, "a!!!");
    }

    #[test]
    fn copy_types_stay_alive() {
        let n = 21;
        let doubled = double(n);
        assert_eq!(doubled, 42);
        assert_eq!(n, 21);
    }

    #[test]
    fn finds_longest_word() {
        assert_eq!(longest_word_owned("я учу rust"), "rust");
        assert_eq!(longest_word_owned("привет мир"), "привет");
        assert_eq!(longest_word_owned(""), "");
        assert_eq!(longest_word_owned("  а   бб  "), "бб");
    }
}
