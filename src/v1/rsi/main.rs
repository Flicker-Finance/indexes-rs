use super::types::{MarketCondition, RSIResult};

pub struct RSI {
    period: usize,
    gains: Vec<f64>,
    losses: Vec<f64>,
    prev_price: Option<f64>,
}

impl RSI {
    pub fn new(period: usize) -> Self {
        RSI {
            period,
            gains: Vec::new(),
            losses: Vec::new(),
            prev_price: None,
        }
    }

    pub fn calculate(&mut self, price: f64) -> Option<RSIResult> {
        if let Some(prev) = self.prev_price {
            let change = price - prev;
            if change >= 0.0 {
                self.gains.push(change);
                self.losses.push(0.0);
            } else {
                self.gains.push(0.0);
                self.losses.push(change.abs());
            }
        }

        self.prev_price = Some(price);

        if self.gains.len() > self.period {
            self.gains.remove(0);
            self.losses.remove(0);
        }

        if self.gains.len() < self.period {
            return None;
        }

        let avg_gain = self.gains.iter().sum::<f64>() / self.period as f64;
        let avg_loss = self.losses.iter().sum::<f64>() / self.period as f64;

        let rs = if avg_loss == 0.0 { 100.0 } else { avg_gain / avg_loss };
        let rsi = 100.0 - (100.0 / (1.0 + rs));

        Some(RSIResult {
            value: rsi,
            condition: self.determine_condition(rsi),
        })
    }

    pub fn determine_condition(&self, rsi: f64) -> MarketCondition {
        match rsi {
            r if r >= 70.0 => MarketCondition::Overbought,
            r if r <= 30.0 => MarketCondition::Oversold,
            _ => MarketCondition::Neutral,
        }
    }
}
