use crate::v1::sma::main::SimpleMovingAverage;

use super::types::BBResult;

pub struct BollingerBands {
    sma: SimpleMovingAverage,
    period: usize,
    multiplier: f64,
    values: Vec<f64>,
}

impl BollingerBands {
    pub fn new(period: usize, multiplier: f64) -> Self {
        BollingerBands {
            sma: SimpleMovingAverage::new(period),
            period,
            multiplier,
            values: Vec::new(),
        }
    }

    pub fn calculate(&mut self, price: f64) -> Option<BBResult> {
        self.sma.add_value(price);
        self.values.push(price);

        if self.values.len() > self.period {
            self.values.remove(0);
        }

        let middle = self.sma.calculate()?;
        let std_dev = self.calculate_std_dev(middle)?;
        let band_width = std_dev * self.multiplier;

        Some(BBResult {
            upper: middle + band_width,
            middle,
            lower: middle - band_width,
        })
    }

    fn calculate_std_dev(&self, mean: f64) -> Option<f64> {
        if self.values.len() < self.period {
            return None;
        }
        let variance = self
            .values
            .iter()
            .map(|value| {
                let diff = mean - value;
                diff * diff
            })
            .sum::<f64>()
            / self.period as f64;

        Some(variance.sqrt())
    }
}
