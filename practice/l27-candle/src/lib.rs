//! Тот же MLP, но на Candle: autograd вместо ручного backprop.

use candle_core::{DType, Device, Result, Tensor};
use candle_nn::{linear, AdamW, Linear, Module, Optimizer, ParamsAdamW, VarBuilder, VarMap};

pub struct Mlp {
    pub l1: Linear,
    pub l2: Linear,
    pub varmap: VarMap,
}

impl Mlp {
    /// Два линейных слоя: `input_dim -> hidden_dim -> 1`.
    pub fn new(
        input_dim: usize,
        hidden_dim: usize,
        vb: VarBuilder,
        varmap: VarMap,
    ) -> Result<Self> {
        todo!()
    }

    /// Логиты, форма (n, 1). Sigmoid здесь НЕ применяется.
    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        todo!()
    }

    /// Вероятности: sigmoid от логитов.
    pub fn probabilities(&self, x: &Tensor) -> Result<Tensor> {
        todo!()
    }
}

/// Данные XOR: X (4,2) и y (4,1), dtype F32.
pub fn xor_data(device: &Device) -> Result<(Tensor, Tensor)> {
    let x = Tensor::new(&[[0f32, 0.], [0., 1.], [1., 0.], [1., 1.]], device)?;
    let y = Tensor::new(&[[0f32], [1.], [1.], [0.]], device)?;
    Ok((x, y))
}

/// Обучение XOR. Loss — BCE по логитам.
pub fn train_xor(
    hidden_dim: usize,
    lr: f64,
    epochs: usize,
    device: &Device,
) -> Result<(Mlp, Vec<f32>)> {
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, device);
    let model = Mlp::new(2, hidden_dim, vb, varmap)?;

    let params = ParamsAdamW {
        lr,
        weight_decay: 0.0,
        ..Default::default()
    };
    let mut optimizer = AdamW::new(model.varmap.all_vars(), params)?;

    let (x, y) = xor_data(device)?;
    let mut losses = Vec::with_capacity(epochs);
    for _ in 0..epochs {
        let logits = model.forward(&x)?;
        let loss = candle_nn::loss::binary_cross_entropy_with_logit(&logits, &y)?;
        optimizer.backward_step(&loss)?;
        losses.push(loss.to_scalar::<f32>()?);
    }
    Ok((model, losses))
}

/// Доля верных ответов на XOR.
pub fn accuracy(model: &Mlp, device: &Device) -> Result<f64> {
    let (x, _) = xor_data(device)?;
    let probs = model.probabilities(&x)?.to_vec2::<f32>()?;
    let target = [0f32, 1., 1., 0.];
    let correct = probs
        .iter()
        .enumerate()
        .filter(|(i, row)| {
            let pred = if row[0] > 0.5 { 1.0 } else { 0.0 };
            (pred - target[*i]).abs() < 1e-6
        })
        .count();
    Ok(correct as f64 / target.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
