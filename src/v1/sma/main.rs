pub struct SimpleMovingAverage {
    pub period: usize,
    pub values: Vec<f64>,
}

impl SimpleMovingAverage {
    pub fn new(period: usize) -> Self {
        SimpleMovingAverage { period, values: Vec::new() }
    }

    pub fn add_value(&mut self, value: f64) {
        self.values.push(value);
        if self.values.len() > self.period {
            self.values.remove(0);
        }
    }

    pub fn calculate(&self) -> Option<f64> {
        if self.values.len() < self.period {
            return None;
        }

        let sum: f64 = self.values.iter().sum();
        Some(sum / self.period as f64)
    }
}
