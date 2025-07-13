use crate::v2::williams_r::types::{
    WilliamsRConfig, WilliamsRError, WilliamsRInput, WilliamsRMarketCondition, WilliamsROutput,
    WilliamsRState,
};

/// Williams %R Indicator
///
/// Williams %R is a momentum oscillator that measures overbought and oversold levels.
/// It's similar to the Stochastic Oscillator but with an inverted scale (0 to -100).
///
/// Formula:
/// Williams %R = (Highest High - Close) / (Highest High - Lowest Low) × -100
///
/// Where:
/// - Highest High = Highest high over the lookback period
/// - Lowest Low = Lowest low over the lookback period
/// - Close = Current closing price
///
/// Interpretation:
/// - Values range from 0 to -100
/// - Above -20: Overbought condition
/// - Below -80: Oversold condition
/// - Above -10: Extremely overbought
/// - Below -90: Extremely oversold
///
/// Williams %R is particularly useful for:
/// - Identifying extreme price conditions
/// - Timing entry/exit points
/// - Confirming trend reversals
pub struct WilliamsR {
    state: WilliamsRState,
}

impl WilliamsR {
    /// Create a new Williams %R calculator with default configuration (period=14)
    pub fn new() -> Self {
        // Use default config directly to avoid validation issues during testing
        Self {
            state: WilliamsRState::new(WilliamsRConfig::default()),
        }
    }

    /// Create a new Williams %R calculator with custom period
    pub fn with_period(period: usize) -> Result<Self, WilliamsRError> {
        if period == 0 {
            return Err(WilliamsRError::InvalidPeriod);
        }

        let config = WilliamsRConfig {
            period,
            ..Default::default()
        };
        Ok(Self::with_config(config))
    }

    /// Create a new Williams %R calculator with custom period and thresholds
    pub fn with_thresholds(
        period: usize,
        overbought: f64,
        oversold: f64,
        extreme_overbought: f64,
        extreme_oversold: f64,
    ) -> Result<Self, WilliamsRError> {
        if period == 0 {
            return Err(WilliamsRError::InvalidPeriod);
        }

        // Williams %R thresholds should be negative and in descending order (towards more negative)
        // Scale: 0 (most overbought) to -100 (most oversold)
        // Valid order: extreme_overbought > overbought > oversold > extreme_oversold
        // Example: -10 > -20 > -80 > -90
        if overbought >= 0.0
            || oversold >= overbought          // Want: -80 < -20 → -80 >= -20 is false ✅
            || extreme_overbought >= 0.0
            || extreme_overbought <= overbought // Want: -10 > -20 → -10 <= -20 is false ✅
            || extreme_oversold >= oversold
        {
            // Want: -90 < -80 → -90 >= -80 is false ✅
            return Err(WilliamsRError::InvalidThresholds);
        }

        let config = WilliamsRConfig {
            period,
            overbought,
            oversold,
            extreme_overbought,
            extreme_oversold,
        };
        Ok(Self::with_config(config))
    }

    /// Create a new Williams %R calculator with custom configuration
    pub fn with_config(config: WilliamsRConfig) -> Self {
        Self {
            state: WilliamsRState::new(config),
        }
    }

    /// Calculate Williams %R for the given input
    pub fn calculate(&mut self, input: WilliamsRInput) -> Result<WilliamsROutput, WilliamsRError> {
        // Validate input
        self.validate_input(&input)?;
        self.validate_config()?;

        // Update price history
        self.update_price_history(input.high, input.low);

        // Calculate Williams %R if we have enough data
        let williams_r = if self.state.has_sufficient_data {
            self.calculate_williams_r_value(input.close)?
        } else {
            -50.0 // Default middle value when insufficient data
        };

        // Determine market condition
        let market_condition = self.determine_market_condition(williams_r);

        // Calculate distances from key levels
        let distance_from_overbought = williams_r - self.state.config.overbought;
        let distance_from_oversold = williams_r - self.state.config.oversold;

        // Calculate price range
        let price_range = self.state.highest_high - self.state.lowest_low;

        Ok(WilliamsROutput {
            williams_r,
            highest_high: self.state.highest_high,
            lowest_low: self.state.lowest_low,
            close: input.close,
            price_range,
            market_condition,
            distance_from_overbought,
            distance_from_oversold,
        })
    }

    /// Calculate Williams %R for a batch of inputs
    pub fn calculate_batch(
        &mut self,
        inputs: &[WilliamsRInput],
    ) -> Result<Vec<WilliamsROutput>, WilliamsRError> {
        inputs.iter().map(|input| self.calculate(*input)).collect()
    }

    /// Reset the calculator state
    pub fn reset(&mut self) {
        self.state = WilliamsRState::new(self.state.config);
    }

    /// Get current state (for serialization/debugging)
    pub fn get_state(&self) -> &WilliamsRState {
        &self.state
    }

    /// Restore state (for deserialization)
    pub fn set_state(&mut self, state: WilliamsRState) {
        self.state = state;
    }

    /// Check if currently overbought
    pub fn is_overbought(&self, williams_r: f64) -> bool {
        williams_r >= self.state.config.overbought
    }

    /// Check if currently oversold
    pub fn is_oversold(&self, williams_r: f64) -> bool {
        williams_r <= self.state.config.oversold
    }

    /// Check if in extreme condition
    pub fn is_extreme_condition(&self, williams_r: f64) -> bool {
        williams_r >= self.state.config.extreme_overbought
            || williams_r <= self.state.config.extreme_oversold
    }

    /// Get signal strength (0.0 to 1.0, where 1.0 is strongest)
    pub fn signal_strength(&self, williams_r: f64) -> f64 {
        // Convert Williams %R to signal strength
        // More extreme values (closer to 0 or -100) have higher strength
        let distance_from_center = (williams_r + 50.0).abs(); // Distance from -50 (center)
        (distance_from_center / 50.0).min(1.0)
    }

    // Private helper methods

    fn validate_input(&self, input: &WilliamsRInput) -> Result<(), WilliamsRError> {
        // Check for valid prices
        if !input.high.is_finite() || !input.low.is_finite() || !input.close.is_finite() {
            return Err(WilliamsRError::InvalidPrice);
        }

        // Check HLC relationship
        if input.high < input.low {
            return Err(WilliamsRError::InvalidHLC);
        }

        if input.close < input.low || input.close > input.high {
            return Err(WilliamsRError::InvalidHLC);
        }

        Ok(())
    }

    fn validate_config(&self) -> Result<(), WilliamsRError> {
        if self.state.config.period == 0 {
            return Err(WilliamsRError::InvalidPeriod);
        }

        let config = &self.state.config;

        // Williams %R thresholds validation:
        // Scale: 0 (most overbought) to -100 (most oversold)
        // We need: extreme_overbought > overbought > oversold > extreme_oversold
        if config.overbought >= 0.0
            || config.oversold >= config.overbought
            || config.extreme_overbought >= 0.0
            || config.extreme_overbought <= config.overbought
            || config.extreme_oversold >= config.oversold
        {
            return Err(WilliamsRError::InvalidThresholds);
        }

        Ok(())
    }

    fn update_price_history(&mut self, high: f64, low: f64) {
        // Remove oldest prices if at capacity
        if self.state.highs.len() >= self.state.config.period {
            self.state.highs.pop_front();
            self.state.lows.pop_front();
        }

        // Add new prices
        self.state.highs.push_back(high);
        self.state.lows.push_back(low);

        // Update highest/lowest values
        self.update_extremes();

        // Check if we have sufficient data
        self.state.has_sufficient_data = self.state.highs.len() >= self.state.config.period;
    }

    fn update_extremes(&mut self) {
        if self.state.highs.is_empty() {
            return;
        }

        // Find highest high and lowest low in the current period
        self.state.highest_high = self
            .state
            .highs
            .iter()
            .fold(f64::NEG_INFINITY, |acc, &x| acc.max(x));
        self.state.lowest_low = self
            .state
            .lows
            .iter()
            .fold(f64::INFINITY, |acc, &x| acc.min(x));
    }

    fn calculate_williams_r_value(&self, close: f64) -> Result<f64, WilliamsRError> {
        if !self.state.has_sufficient_data {
            return Ok(-50.0); // Default middle value
        }

        let price_range = self.state.highest_high - self.state.lowest_low;

        if price_range == 0.0 {
            // All prices are the same - return middle value
            return Ok(-50.0);
        }

        // Williams %R formula: (Highest High - Close) / (Highest High - Lowest Low) × -100
        let williams_r = ((self.state.highest_high - close) / price_range) * -100.0;

        if !williams_r.is_finite() {
            return Err(WilliamsRError::DivisionByZero);
        }

        // Clamp to valid range (0 to -100)
        Ok(williams_r.max(-100.0).min(0.0))
    }

    fn determine_market_condition(&self, williams_r: f64) -> WilliamsRMarketCondition {
        if !self.state.has_sufficient_data {
            WilliamsRMarketCondition::Insufficient
        } else if williams_r >= self.state.config.extreme_overbought {
            WilliamsRMarketCondition::ExtremeOverbought
        } else if williams_r >= self.state.config.overbought {
            WilliamsRMarketCondition::Overbought
        } else if williams_r <= self.state.config.extreme_oversold {
            WilliamsRMarketCondition::ExtremeOversold
        } else if williams_r <= self.state.config.oversold {
            WilliamsRMarketCondition::Oversold
        } else {
            WilliamsRMarketCondition::Normal
        }
    }
}

impl Default for WilliamsR {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to calculate Williams %R for HLC data without maintaining state
pub fn calculate_williams_r_simple(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    period: usize,
) -> Result<Vec<f64>, WilliamsRError> {
    let len = highs.len();
    if len != lows.len() || len != closes.len() {
        return Err(WilliamsRError::InvalidInput(
            "All price arrays must have same length".to_string(),
        ));
    }

    if len == 0 {
        return Ok(Vec::new());
    }

    let mut williams_r_calculator = WilliamsR::with_period(period)?;
    let mut results = Vec::with_capacity(len);

    for i in 0..len {
        let input = WilliamsRInput {
            high: highs[i],
            low: lows[i],
            close: closes[i],
        };
        let output = williams_r_calculator.calculate(input)?;
        results.push(output.williams_r);
    }

    Ok(results)
}
