//! Спека лабы 12. Эти тесты — твоё ТЗ: читай их и пиши код в src/lib.rs.

use l12_generics::*;

#[test]
fn finds_largest() {
    assert_eq!(largest(&[3, 7, 2]), Some(7));
    assert_eq!(largest(&[1.5, 0.5]), Some(1.5));
    assert_eq!(largest(&['я', 'б']), Some('я'));
    let empty: [i32; 0] = [];
    assert_eq!(largest(&empty), None);
}

#[test]
fn builds_pairs() {
    let p = Pair::new(1, 2);
    assert_eq!(p.first, 1);
    assert_eq!(p.second, 2);
}

#[test]
fn picks_larger() {
    let ints = Pair::new(1, 5);
    assert_eq!(*ints.larger(), 5);

    let texts = Pair::new("а", "б");
    assert_eq!(*texts.larger(), "б");
}

#[test]
fn prints_without_panicking() {
    print_all(&[1, 2, 3]);
    print_all(&["а", "б"]);
    let empty: [i32; 0] = [];
    print_all(&empty);
}

#[test]
fn makes_mixed_pair() {
    let p = make_pair(1, "один");
    assert_eq!(p, (1, "один"));
}
