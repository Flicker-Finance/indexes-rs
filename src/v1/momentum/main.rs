use super::types::MomentumResult;

pub struct Momentum {
    period: usize,
    values: Vec<f64>,
}

impl Momentum {
    pub const DEFAULT_PERIOD: usize = 14;

    pub fn new(period: usize) -> Self {
        Momentum { period, values: Vec::new() }
    }

    pub fn calculate(&mut self, price: f64) -> Option<MomentumResult> {
        self.values.push(price);

        if self.values.len() > self.period {
            self.values.remove(0);
        }

        if self.values.len() < self.period {
            return None;
        }

        let past_price = self.values.first()?;
        let momentum = price - past_price;
        let momentum_ratio = (price / past_price) * 100.0;

        Some(MomentumResult {
            value: momentum,
            ratio: momentum_ratio,
        })
    }
}
