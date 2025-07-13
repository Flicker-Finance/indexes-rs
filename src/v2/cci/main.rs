use crate::v2::cci::types::{
    CCIConfig, CCIError, CCIInput, CCIMarketCondition, CCIOutput, CCIState,
};

/// Commodity Channel Index (CCI) Indicator
///
/// CCI measures how far the current price deviates from its statistical average.
/// It's used to identify cyclical trends and overbought/oversold conditions.
///
/// Formula:
/// 1. Typical Price = (High + Low + Close) / 3
/// 2. SMA of Typical Price = Sum(TP) / Period
/// 3. Mean Deviation = Sum(|TP - SMA|) / Period
/// 4. CCI = (TP - SMA) / (0.015 × Mean Deviation)
///
/// The constant 0.015 ensures about 70-80% of CCI values fall between -100 and +100.
///
/// Interpretation:
/// - Above +100: Overbought, potential sell signal
/// - Below -100: Oversold, potential buy signal
/// - Above +200: Extremely overbought
/// - Below -200: Extremely oversold
pub struct CCI {
    state: CCIState,
}

impl CCI {
    /// Create a new CCI calculator with default configuration (period=20)
    pub fn new() -> Self {
        Self::with_config(CCIConfig::default())
    }

    /// Create a new CCI calculator with custom period
    pub fn with_period(period: usize) -> Result<Self, CCIError> {
        if period == 0 {
            return Err(CCIError::InvalidPeriod);
        }

        let config = CCIConfig {
            period,
            ..Default::default()
        };
        Ok(Self::with_config(config))
    }

    /// Create a new CCI calculator with custom period and thresholds
    pub fn with_thresholds(
        period: usize,
        overbought: f64,
        oversold: f64,
        extreme_overbought: f64,
        extreme_oversold: f64,
    ) -> Result<Self, CCIError> {
        if period == 0 {
            return Err(CCIError::InvalidPeriod);
        }

        if overbought <= oversold
            || extreme_overbought <= overbought
            || extreme_oversold >= oversold
        {
            return Err(CCIError::InvalidThresholds);
        }

        let config = CCIConfig {
            period,
            overbought,
            oversold,
            extreme_overbought,
            extreme_oversold,
        };
        Ok(Self::with_config(config))
    }

    /// Create a new CCI calculator with custom configuration
    pub fn with_config(config: CCIConfig) -> Self {
        Self {
            state: CCIState::new(config),
        }
    }

    /// Calculate CCI for the given input
    pub fn calculate(&mut self, input: CCIInput) -> Result<CCIOutput, CCIError> {
        // Validate input
        self.validate_input(&input)?;
        self.validate_config()?;

        // Calculate typical price
        let typical_price = self.calculate_typical_price(&input);

        // Update typical price history
        self.update_typical_price_history(typical_price);

        // Calculate CCI if we have enough data
        let (cci, sma_tp, mean_deviation) = if self.state.has_sufficient_data {
            self.calculate_cci_value(typical_price)?
        } else {
            (0.0, typical_price, 0.0) // Default values when insufficient data
        };

        // Determine market condition
        let market_condition = self.determine_market_condition(cci);

        // Calculate distance from zero
        let distance_from_zero = cci.abs();

        Ok(CCIOutput {
            cci,
            typical_price,
            sma_tp,
            mean_deviation,
            market_condition,
            distance_from_zero,
        })
    }

    /// Calculate CCI for a batch of inputs
    pub fn calculate_batch(&mut self, inputs: &[CCIInput]) -> Result<Vec<CCIOutput>, CCIError> {
        inputs.iter().map(|input| self.calculate(*input)).collect()
    }

    /// Reset the calculator state
    pub fn reset(&mut self) {
        self.state = CCIState::new(self.state.config);
    }

    /// Get current state (for serialization/debugging)
    pub fn get_state(&self) -> &CCIState {
        &self.state
    }

    /// Restore state (for deserialization)
    pub fn set_state(&mut self, state: CCIState) {
        self.state = state;
    }

    /// Get current market condition
    pub fn market_condition(&self) -> CCIMarketCondition {
        if !self.state.has_sufficient_data {
            CCIMarketCondition::Insufficient
        } else {
            // Would need the last CCI value to determine this
            CCIMarketCondition::Normal
        }
    }

    /// Check if currently overbought
    pub fn is_overbought(&self, cci: f64) -> bool {
        cci >= self.state.config.overbought
    }

    /// Check if currently oversold
    pub fn is_oversold(&self, cci: f64) -> bool {
        cci <= self.state.config.oversold
    }

    /// Check if in extreme condition
    pub fn is_extreme_condition(&self, cci: f64) -> bool {
        cci >= self.state.config.extreme_overbought || cci <= self.state.config.extreme_oversold
    }

    // Private helper methods

    fn validate_input(&self, input: &CCIInput) -> Result<(), CCIError> {
        // Check for valid prices
        if !input.high.is_finite() || !input.low.is_finite() || !input.close.is_finite() {
            return Err(CCIError::InvalidPrice);
        }

        // Check HLC relationship
        if input.high < input.low {
            return Err(CCIError::InvalidHLC);
        }

        if input.close < input.low || input.close > input.high {
            return Err(CCIError::InvalidHLC);
        }

        Ok(())
    }

    fn validate_config(&self) -> Result<(), CCIError> {
        if self.state.config.period == 0 {
            return Err(CCIError::InvalidPeriod);
        }

        let config = &self.state.config;
        if config.overbought <= config.oversold
            || config.extreme_overbought <= config.overbought
            || config.extreme_oversold >= config.oversold
        {
            return Err(CCIError::InvalidThresholds);
        }

        Ok(())
    }

    fn calculate_typical_price(&self, input: &CCIInput) -> f64 {
        (input.high + input.low + input.close) / 3.0
    }

    fn update_typical_price_history(&mut self, typical_price: f64) {
        // Remove oldest if at capacity
        if self.state.typical_prices.len() >= self.state.config.period {
            if let Some(oldest) = self.state.typical_prices.pop_front() {
                self.state.tp_sum -= oldest;
            }
        }

        // Add new typical price
        self.state.typical_prices.push_back(typical_price);
        self.state.tp_sum += typical_price;

        // Check if we have sufficient data
        self.state.has_sufficient_data =
            self.state.typical_prices.len() >= self.state.config.period;
    }

    fn calculate_cci_value(&self, current_tp: f64) -> Result<(f64, f64, f64), CCIError> {
        if !self.state.has_sufficient_data {
            return Ok((0.0, current_tp, 0.0));
        }

        // Calculate SMA of typical prices
        let sma_tp = self.state.tp_sum / self.state.config.period as f64;

        // Calculate mean absolute deviation
        let mean_deviation = self.calculate_mean_deviation(sma_tp);

        // Calculate CCI
        if mean_deviation == 0.0 {
            // If mean deviation is zero, prices are identical
            return Ok((0.0, sma_tp, mean_deviation));
        }

        // CCI formula: (TP - SMA) / (0.015 × Mean Deviation)
        let cci = (current_tp - sma_tp) / (0.015 * mean_deviation);

        if !cci.is_finite() {
            return Err(CCIError::DivisionByZero);
        }

        Ok((cci, sma_tp, mean_deviation))
    }

    fn calculate_mean_deviation(&self, sma_tp: f64) -> f64 {
        let sum_deviations: f64 = self
            .state
            .typical_prices
            .iter()
            .map(|&tp| (tp - sma_tp).abs())
            .sum();

        sum_deviations / self.state.config.period as f64
    }

    fn determine_market_condition(&self, cci: f64) -> CCIMarketCondition {
        if !self.state.has_sufficient_data {
            CCIMarketCondition::Insufficient
        } else if cci >= self.state.config.extreme_overbought {
            CCIMarketCondition::ExtremeOverbought
        } else if cci >= self.state.config.overbought {
            CCIMarketCondition::Overbought
        } else if cci <= self.state.config.extreme_oversold {
            CCIMarketCondition::ExtremeOversold
        } else if cci <= self.state.config.oversold {
            CCIMarketCondition::Oversold
        } else {
            CCIMarketCondition::Normal
        }
    }
}

impl Default for CCI {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to calculate CCI for HLC data without maintaining state
pub fn calculate_cci_simple(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    period: usize,
) -> Result<Vec<f64>, CCIError> {
    let len = highs.len();
    if len != lows.len() || len != closes.len() {
        return Err(CCIError::InvalidInput(
            "All price arrays must have same length".to_string(),
        ));
    }

    if len == 0 {
        return Ok(Vec::new());
    }

    let mut cci_calculator = CCI::with_period(period)?;
    let mut results = Vec::with_capacity(len);

    for i in 0..len {
        let input = CCIInput {
            high: highs[i],
            low: lows[i],
            close: closes[i],
        };
        let output = cci_calculator.calculate(input)?;
        results.push(output.cci);
    }

    Ok(results)
}
