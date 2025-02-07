pub struct ATR {
    period: usize,
    prev_close: Option<f64>,
    values: Vec<f64>,
}

impl ATR {
    pub fn new(period: usize) -> Self {
        ATR {
            period,
            prev_close: None,
            values: Vec::new(),
        }
    }

    pub fn calculate(&mut self, close: f64) -> Option<f64> {
        let true_range = self.prev_close.map_or(0.0, |prev| {
            (close - prev).abs() // Using price change as true range
        });

        self.values.push(true_range);
        if self.values.len() > self.period {
            self.values.remove(0);
        }

        self.prev_close = Some(close);

        if self.values.len() == self.period {
            Some(self.values.iter().sum::<f64>() / self.period as f64)
        } else {
            None
        }
    }
}
