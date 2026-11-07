//! `Box`, `Rc`, `RefCell`.

use std::cell::RefCell;
use std::rc::Rc;

/// Односвязный список — рекурсивный тип, поэтому `Box`.
#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    /// Строит список из среза, сохраняя порядок.
    pub fn from_slice(values: &[i32]) -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn sum(&self) -> i32 {
        todo!()
    }
}

/// Журнал, которым владеют несколько мест в программе.
pub type SharedLog = Rc<RefCell<Vec<String>>>;

pub fn new_log() -> SharedLog {
    Rc::new(RefCell::new(Vec::new()))
}

/// Дописывает запись.
pub fn log_message(log: &SharedLog, message: &str) {
    todo!()
}

/// Сколько записей накопилось.
pub fn log_len(log: &SharedLog) -> usize {
    todo!()
}

/// Снимок журнала.
pub fn log_snapshot(log: &SharedLog) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_and_measures_list() {
        let list = List::from_slice(&[1, 2, 3]);
        assert_eq!(list.len(), 3);
        assert!(!list.is_empty());
        assert_eq!(list.sum(), 6);

        let empty = List::from_slice(&[]);
        assert_eq!(empty.len(), 0);
        assert!(empty.is_empty());
        assert_eq!(empty.sum(), 0);
    }

    #[test]
    fn log_is_shared() {
        let log = new_log();
        let second = Rc::clone(&log);

        log_message(&log, "первая");
        log_message(&second, "вторая");

        assert_eq!(log_len(&log), 2);
        assert_eq!(
            log_snapshot(&log),
            vec!["первая".to_string(), "вторая".to_string()]
        );
        assert_eq!(Rc::strong_count(&log), 2);
    }

    #[test]
    fn refcell_survives_many_borrows() {
        let log = new_log();
        for i in 0..3 {
            log_message(&log, &format!("запись {i}"));
        }
        assert_eq!(log_len(&log), 3);
    }
}
