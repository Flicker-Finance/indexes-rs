use crate::v1::types::TradingSignal;

use super::types::ROCResult;

pub struct ROC {
    period: usize,
    values: Vec<f64>,
    prev_roc: Option<f64>,
}

impl ROC {
    pub const DEFAULT_PERIOD: usize = 12;
    pub const SIGNAL_THRESHOLD: f64 = 2.0;

    pub fn new(period: usize) -> Self {
        ROC {
            period,
            values: Vec::new(),
            prev_roc: None,
        }
    }

    pub fn calculate(&mut self, price: f64) -> Option<ROCResult> {
        self.values.push(price);

        if self.values.len() > self.period + 1 {
            self.values.remove(0);
        }

        if self.values.len() <= self.period {
            return None;
        }

        let old_price = self.values.first()?;
        let current_roc = ((price - old_price) / old_price) * 100.0;

        let acceleration = self.prev_roc.map(|prev| current_roc - prev);
        self.prev_roc = Some(current_roc);

        Some(ROCResult {
            value: current_roc,
            momentum: self.normalize_momentum(current_roc),
            acceleration,
            signal: self.get_signal(current_roc),
        })
    }

    fn normalize_momentum(&self, roc: f64) -> f64 {
        // Using a more gradual normalization
        // Assuming typical ROC values range from -10 to +10
        let normalized = (roc / 10.0) * 100.0;
        normalized.clamp(-100.0, 100.0)
    }

    fn get_signal(&self, roc: f64) -> TradingSignal {
        if roc > Self::SIGNAL_THRESHOLD {
            TradingSignal::Buy
        } else if roc < -Self::SIGNAL_THRESHOLD {
            TradingSignal::Sell
        } else {
            TradingSignal::Hold
        }
    }
}
