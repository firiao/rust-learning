//! Лаба про Cargo: features и профили.

/// Базовая функция, доступна всегда.
pub fn base() -> u32 {
    todo!()
}

/// Доступна только при включённой feature `extra`.
///
/// Проверить: `cargo test -p l19-cargo --features extra`
#[cfg(feature = "extra")]
pub fn extra() -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_works() {
        assert_eq!(base(), 42);
    }

    #[cfg(feature = "extra")]
    #[test]
    fn extra_works_only_with_feature() {
        assert_eq!(extra(), 100);
    }
}
