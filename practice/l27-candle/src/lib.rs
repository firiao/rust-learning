//! Лаба 27. Candle.
//!
//! Реализация — твоя задача. Спека лежит в `tests/spec.rs`.
//!
//! Исключение: `xor_data()` уже написан. Данные — не задача лабы, а вход,
//! от которого зависят тесты, поэтому их не надо угадывать.

use candle_core::{Device, Result, Tensor};

/// Данные XOR: X (4,2) и y (4,1), dtype F32.
pub fn xor_data(device: &Device) -> Result<(Tensor, Tensor)> {
    let x = Tensor::new(&[[0f32, 0.], [0., 1.], [1., 0.], [1., 1.]], device)?;
    let y = Tensor::new(&[[0f32], [1.], [1.], [0.]], device)?;
    Ok((x, y))
}
