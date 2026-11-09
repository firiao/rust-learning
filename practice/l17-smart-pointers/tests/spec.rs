//! Спека лабы 17. Эти тесты — твоё ТЗ: читай их и пиши код в src/lib.rs.

use std::rc::Rc;

use std::rc::Rc;

use l17_smart_pointers::*;

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
