//! Спека лабы 19. Эти тесты — твоё ТЗ: читай их и пиши код в src/lib.rs.

use l19_cargo::*;

#[test]
fn base_works() {
    assert_eq!(base(), 42);
}

#[cfg(feature = "extra")]
#[test]
fn extra_works_only_with_feature() {
    assert_eq!(extra(), 100);
}
