//! Move, Copy и clone.

/// Забирает владение, дописывает '!' и возвращает обратно.
pub fn append_bang(mut text: String) -> String {
    todo!()
}

/// Забирает вектор и возвращает сумму.
pub fn total(values: Vec<i32>) -> i32 {
    todo!()
}

/// Самое длинное слово (по числу символов). При равенстве — первое.
pub fn longest_word_owned(text: &str) -> String {
    todo!()
}

/// Copy-тип: принимает по значению, но вызывающий не теряет переменную.
pub fn double(n: i32) -> i32 {
    todo!()
}

// ===== ЭКСПЕРИМЕНТЫ =====
// Раскомментируй ОДИН блок (убери `//` в начале строк),
// выполни `cargo check -p l05-ownership`, прочитай ошибку,
// закомментируй обратно.

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
        text = append_bang(text);
        assert_eq!(text, "a!!");
    }

    #[test]
    fn sums_vector() {
        assert_eq!(total(vec![1, 2, 3, 4]), 10);
        assert_eq!(total(vec![]), 0);
        assert_eq!(total(vec![-5, 5]), 0);
    }

    #[test]
    fn finds_longest_word() {
        assert_eq!(longest_word_owned("я учу rust"), "rust");
        assert_eq!(longest_word_owned("привет мир"), "привет");
        assert_eq!(longest_word_owned(""), "");
        assert_eq!(longest_word_owned("  а   бб  "), "бб");
    }

    #[test]
    fn copy_types_stay_alive() {
        let n = 21;
        let doubled = double(n);
        assert_eq!(doubled, 42);
        assert_eq!(n, 21); // n жив: i32 — Copy
    }
}
