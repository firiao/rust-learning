//! Спека лабы 27. Эти тесты — твоё ТЗ: читай их и пиши код в src/lib.rs.

use l27_candle::*;

#[test]
fn forward_returns_logits() -> Result<()> {
    let device = Device::Cpu;
    let (model, _) = train_xor(8, 0.1, 1, &device)?;
    let (x, _) = xor_data(&device)?;
    let logits = model.forward(&x)?;
    assert_eq!(logits.dims(), &[4, 1]);

    let probs = model.probabilities(&x)?.to_vec2::<f32>()?;
    for row in &probs {
        assert!(row[0] > 0.0 && row[0] < 1.0, "p = {}", row[0]);
    }
    Ok(())
}

#[test]
fn learns_xor() -> Result<()> {
    let device = Device::Cpu;
    let (model, losses) = train_xor(16, 0.1, 2000, &device)?;

    let first = *losses.first().unwrap();
    let last = *losses.last().unwrap();
    assert!(last < first, "loss должен убывать: {first} -> {last}");
    assert!(last < 0.01, "loss = {last}");
    assert_eq!(accuracy(&model, &device)?, 1.0);
    Ok(())
}
