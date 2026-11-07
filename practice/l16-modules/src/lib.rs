//! Модули, тесты, документация.
//!
//! Плоский публичный API: снаружи доступно `l16_modules::area` и `l16_modules::normalize`,
//! хотя лежат они в разных модулях.

pub mod geometry;
pub mod text;

pub use geometry::area;
pub use text::normalize;
