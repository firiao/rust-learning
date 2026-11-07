//! MLP с нуля: XOR, forward и backprop руками.

/// Простой генератор псевдослучайных чисел (LCG).
pub struct Rng(u64);

impl Rng {
    /// Число в диапазоне [-0.5, 0.5).
    pub fn next_f64(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 32) as f64) / (u32::MAX as f64) - 0.5
    }
}

pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

pub struct Mlp {
    /// hidden × 2, по строкам: [w_i1, w_i2, ...]
    pub w1: Vec<f64>,
    pub b1: Vec<f64>,
    /// hidden
    pub w2: Vec<f64>,
    pub b2: f64,
}

impl Mlp {
    pub fn new(hidden: usize, seed: u64) -> Self {
        let mut rng = Rng(seed);
        Self {
            w1: (0..hidden * 2).map(|_| rng.next_f64()).collect(),
            b1: (0..hidden).map(|_| rng.next_f64()).collect(),
            w2: (0..hidden).map(|_| rng.next_f64()).collect(),
            b2: 0.0,
        }
    }

    /// Возвращает (активации скрытого слоя, вероятность).
    pub fn forward(&self, x: &[f64]) -> (Vec<f64>, f64) {
        todo!()
    }

    /// Один шаг SGD по одному примеру. Возвращает loss.
    pub fn step(&mut self, x: &[f64], y: f64, lr: f64) -> f64 {
        todo!()
    }
}

/// Данные XOR: (вход, метка).
pub fn xor_data() -> [([f64; 2], f64); 4] {
    [
        ([0.0, 0.0], 0.0),
        ([0.0, 1.0], 1.0),
        ([1.0, 0.0], 1.0),
        ([1.0, 1.0], 0.0),
    ]
}

/// Обучает сеть на XOR. Возвращает сеть и историю loss.
pub fn train_xor(hidden: usize, lr: f64, epochs: usize, seed: u64) -> (Mlp, Vec<f64>) {
    let mut net = Mlp::new(hidden, seed);
    let data = xor_data();
    let mut history = Vec::with_capacity(epochs);
    for _ in 0..epochs {
        let mut loss = 0.0;
        for (x, y) in &data {
            loss += net.step(x, *y, lr);
        }
        history.push(loss / data.len() as f64);
    }
    (net, history)
}

/// Точность на XOR.
pub fn accuracy(net: &Mlp) -> f64 {
    let data = xor_data();
    let correct = data
        .iter()
        .filter(|(x, y)| {
            let (_, p) = net.forward(x);
            let pred = if p > 0.5 { 1.0 } else { 0.0 };
            (pred - y).abs() < 1e-9
        })
        .count();
    correct as f64 / data.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forward_shape_and_range() {
        let net = Mlp::new(8, 1);
        let (h, p) = net.forward(&[1.0, 0.0]);
        assert_eq!(h.len(), 8);
        assert!(p > 0.0 && p < 1.0, "p = {p}");
        assert!(h.iter().all(|v| *v >= 0.0));
    }

    #[test]
    fn trains_xor() {
        let (net, history) = train_xor(8, 0.1, 10_000, 1);
        let first = *history.first().unwrap();
        let last = *history.last().unwrap();
        assert!(last < first, "loss должен убывать: {first} -> {last}");
        assert!(last < 0.01, "loss = {last}");
        assert_eq!(accuracy(&net), 1.0);
    }
}
