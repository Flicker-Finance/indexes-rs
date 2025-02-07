use super::types::*;

pub struct StochasticOscillator {
    period: usize,   // %K period (typically 14)
    k_smooth: usize, // %K smoothing (typically 3)
    d_period: usize, // %D period (typically 3)
    prices: Vec<f64>,
    highs: Vec<f64>,
    lows: Vec<f64>,
    k_values: Vec<f64>, // Store K values for D calculation
}

impl StochasticOscillator {
    pub const DEFAULT_PERIOD: usize = 14;
    pub const DEFAULT_K_SMOOTH: usize = 3;
    pub const DEFAULT_D_PERIOD: usize = 3;

    const OVERBOUGHT_THRESHOLD: f64 = 80.0;
    const OVERSOLD_THRESHOLD: f64 = 20.0;

    pub fn new(period: usize, k_smooth: usize, d_period: usize) -> Self {
        StochasticOscillator {
            period,
            k_smooth,
            d_period,
            prices: Vec::new(),
            highs: Vec::new(),
            lows: Vec::new(),
            k_values: Vec::new(),
        }
    }

    pub fn calculate(&mut self, price: f64) -> Option<StochResult> {
        self.prices.push(price);
        self.highs.push(price);
        self.lows.push(price);

        if self.prices.len() > self.period {
            self.prices.remove(0);
            self.highs.remove(0);
            self.lows.remove(0);
        }

        if self.prices.len() < self.period {
            return None;
        }

        let highest_high = self.highs.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let lowest_low = self.lows.iter().fold(f64::INFINITY, |a, &b| a.min(b));

        let k = if highest_high == lowest_low {
            50.0
        } else {
            ((price - lowest_low) / (highest_high - lowest_low)) * 100.0
        };

        // Store K value for D calculation
        self.k_values.push(k);
        if self.k_values.len() > self.d_period {
            self.k_values.remove(0);
        }

        let d = self.calculate_d();
        let signal = self.generate_signal(k, d);
        let condition = self.determine_condition(k);
        let crossover = self.detect_crossover(k, d);

        Some(StochResult {
            k_value: k,
            d_value: d,
            signal,
            condition,
            crossover,
            strength: self.calculate_strength(k, d),
        })
    }

    fn calculate_d(&self) -> f64 {
        if self.k_values.len() < self.d_period {
            self.k_values.last().copied().unwrap_or(50.0)
        } else {
            self.k_values.iter().sum::<f64>() / self.d_period as f64
        }
    }

    fn generate_signal(&self, k: f64, d: f64) -> StochSignal {
        match (k, d) {
            (k, d) if k > d && k < Self::OVERBOUGHT_THRESHOLD => StochSignal::Buy,
            (k, d) if k < d && k > Self::OVERSOLD_THRESHOLD => StochSignal::Sell,
            (k, _) if k >= Self::OVERBOUGHT_THRESHOLD => StochSignal::Overbought,
            (k, _) if k <= Self::OVERSOLD_THRESHOLD => StochSignal::Oversold,
            _ => StochSignal::Neutral,
        }
    }

    fn determine_condition(&self, k: f64) -> StochCondition {
        match k {
            k if k >= Self::OVERBOUGHT_THRESHOLD => StochCondition::Overbought,
            k if k <= Self::OVERSOLD_THRESHOLD => StochCondition::Oversold,
            k if k > 50.0 => StochCondition::Strong,
            k if k < 50.0 => StochCondition::Weak,
            _ => StochCondition::Neutral,
        }
    }

    fn detect_crossover(&self, k: f64, d: f64) -> StochCrossover {
        if self.k_values.len() < 2 {
            return StochCrossover::None;
        }

        let prev_k = self.k_values[self.k_values.len() - 2];
        if k > d && prev_k <= d {
            StochCrossover::Bullish
        } else if k < d && prev_k >= d {
            StochCrossover::Bearish
        } else {
            StochCrossover::None
        }
    }

    fn calculate_strength(&self, k: f64, d: f64) -> f64 {
        let trend_strength = (k - 50.0).abs() / 50.0;
        let momentum = (k - d).abs() / 20.0; // Normalize difference
        ((trend_strength + momentum) / 2.0 * 100.0).min(100.0)
    }
}
