pub struct ExponentialMovingAverage {
    pub alpha: f64,
    pub current_ema: Option<f64>,
}

impl ExponentialMovingAverage {
    pub fn new(period: usize) -> Self {
        ExponentialMovingAverage {
            alpha: 2.0 / (period as f64 + 1.0),
            current_ema: None,
        }
    }

    pub fn add_value(&mut self, price: f64) -> Option<f64> {
        self.current_ema = Some(match self.current_ema {
            Some(ema) => price * self.alpha + ema * (1.0 - self.alpha),
            None => price,
        });
        self.current_ema
    }

    pub fn get_current_value(&self) -> Option<f64> {
        self.current_ema
    }
}
