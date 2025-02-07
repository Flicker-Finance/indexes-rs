use super::types::*;

pub struct SupportResistance {
    period: usize,
    prices: Vec<f64>,
    swing_high_threshold: f64,
    swing_low_threshold: f64,
    support_levels: Vec<f64>,
    resistance_levels: Vec<f64>,
}

impl SupportResistance {
    pub const DEFAULT_PERIOD: usize = 20;
    pub const DEFAULT_THRESHOLD: f64 = 0.02; // 2% threshold

    pub fn new(period: usize, threshold: f64) -> Self {
        SupportResistance {
            period,
            prices: Vec::new(),
            swing_high_threshold: 1.0 + threshold,
            swing_low_threshold: 1.0 - threshold,
            support_levels: Vec::new(),
            resistance_levels: Vec::new(),
        }
    }

    pub fn calculate(&mut self, price: f64) -> Option<SRResult> {
        self.prices.push(price);
        if self.prices.len() > self.period * 2 {
            self.prices.remove(0);
        }

        if self.prices.len() < self.period {
            return None;
        }

        self.update_levels(price);

        Some(SRResult {
            nearest_support: self.find_nearest_support(price),
            nearest_resistance: self.find_nearest_resistance(price),
            support_strength: self.calculate_support_strength(price),
            resistance_strength: self.calculate_resistance_strength(price),
            breakout_potential: self.calculate_breakout_potential(price),
            price_position: self.determine_price_position(price),
        })
    }

    fn update_levels(&mut self, current_price: f64) {
        // Detect swing highs and lows
        if let Some(window) = self.prices.get(self.prices.len().saturating_sub(self.period)..) {
            if self.is_swing_high(window) {
                self.resistance_levels.push(current_price);
            }
            if self.is_swing_low(window) {
                self.support_levels.push(current_price);
            }
        }

        // Clean up old levels
        self.clean_levels(current_price);
    }

    fn is_swing_high(&self, window: &[f64]) -> bool {
        if window.len() < 3 {
            return false;
        }
        let mid = window.len() / 2;
        let mid_price = window[mid];

        window[..mid].iter().all(|&p| p < mid_price) && window[mid + 1..].iter().all(|&p| p < mid_price)
    }

    fn is_swing_low(&self, window: &[f64]) -> bool {
        if window.len() < 3 {
            return false;
        }
        let mid = window.len() / 2;
        let mid_price = window[mid];

        window[..mid].iter().all(|&p| p > mid_price) && window[mid + 1..].iter().all(|&p| p > mid_price)
    }

    fn clean_levels(&mut self, current_price: f64) {
        // Remove invalidated levels
        self.support_levels.retain(|&level| level < current_price * self.swing_high_threshold);
        self.resistance_levels.retain(|&level| level > current_price * self.swing_low_threshold);
    }

    fn find_nearest_support(&self, price: f64) -> Option<f64> {
        self.support_levels.iter().filter(|&&s| s < price).max_by(|a, b| a.partial_cmp(b).unwrap()).copied()
    }

    fn find_nearest_resistance(&self, price: f64) -> Option<f64> {
        self.resistance_levels.iter().filter(|&&r| r > price).min_by(|a, b| a.partial_cmp(b).unwrap()).copied()
    }

    fn calculate_support_strength(&self, price: f64) -> f64 {
        if let Some(support) = self.find_nearest_support(price) {
            let distance = (price - support).abs() / price;
            (1.0 - distance).clamp(0.0, 1.0) * 100.0
        } else {
            0.0
        }
    }

    fn calculate_resistance_strength(&self, price: f64) -> f64 {
        if let Some(resistance) = self.find_nearest_resistance(price) {
            let distance = (resistance - price).abs() / price;
            (1.0 - distance).clamp(0.0, 1.0) * 100.0
        } else {
            0.0
        }
    }

    fn calculate_breakout_potential(&self, price: f64) -> f64 {
        let support_strength = self.calculate_support_strength(price);
        let resistance_strength = self.calculate_resistance_strength(price);

        if support_strength > resistance_strength {
            resistance_strength
        } else {
            support_strength
        }
    }

    fn determine_price_position(&self, price: f64) -> PricePosition {
        match (self.find_nearest_support(price), self.find_nearest_resistance(price)) {
            (Some(s), Some(r)) => {
                let mid_point = (s + r) / 2.0;
                if (price - mid_point).abs() < (r - s) * 0.1 {
                    PricePosition::Middle
                } else if price > mid_point {
                    PricePosition::NearResistance
                } else {
                    PricePosition::NearSupport
                }
            }
            (Some(_), None) => PricePosition::AboveResistance,
            (None, Some(_)) => PricePosition::BelowSupport,
            (None, None) => PricePosition::Unknown,
        }
    }
}
