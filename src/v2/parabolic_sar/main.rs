use crate::v2::parabolic_sar::types::{
    ParabolicSARConfig, ParabolicSARError, ParabolicSARInput, ParabolicSAROutput,
    ParabolicSARState, TrendDirection,
};

/// Parabolic SAR (Stop and Reverse) Indicator
///
/// Parabolic SAR is a trend-following indicator that provides stop-loss levels
/// and potential reversal points. It accelerates when the trend continues.
///
/// Formula:
/// - Uptrend: SAR = Previous SAR + AF × (EP - Previous SAR)
/// - Downtrend: SAR = Previous SAR + AF × (EP - Previous SAR)
///
/// Where:
/// - AF = Acceleration Factor (starts at 0.02, increments by 0.02, max 0.20)
/// - EP = Extreme Point (highest high in uptrend, lowest low in downtrend)
pub struct ParabolicSAR {
    pub state: ParabolicSARState,
}

impl ParabolicSAR {
    /// Create a new Parabolic SAR calculator with default configuration
    pub fn new() -> Self {
        Self::with_config(ParabolicSARConfig::default())
    }

    /// Create a new Parabolic SAR calculator with custom acceleration parameters
    pub fn with_acceleration(
        start: f64,
        increment: f64,
        maximum: f64,
    ) -> Result<Self, ParabolicSARError> {
        let config = ParabolicSARConfig {
            acceleration_start: start,
            acceleration_increment: increment,
            acceleration_maximum: maximum,
        };

        // Validate configuration
        if start <= 0.0 || increment <= 0.0 || maximum <= start {
            return Err(ParabolicSARError::InvalidAcceleration);
        }

        Ok(Self::with_config(config))
    }

    /// Create a new Parabolic SAR calculator with custom configuration
    pub fn with_config(config: ParabolicSARConfig) -> Self {
        Self {
            state: ParabolicSARState::new(config),
        }
    }

    /// Calculate Parabolic SAR for the given input
    pub fn calculate(
        &mut self,
        input: ParabolicSARInput,
    ) -> Result<ParabolicSAROutput, ParabolicSARError> {
        // Validate input
        self.validate_input(&input)?;
        self.validate_config()?;

        let result = if self.state.is_first {
            self.handle_first_calculation(input)
        } else if self.state.is_second {
            self.handle_second_calculation(input)
        } else {
            self.handle_normal_calculation(input)
        };

        // Update state for next calculation
        self.update_state_after_calculation(input);

        result
    }

    /// Calculate Parabolic SAR for a batch of inputs
    pub fn calculate_batch(
        &mut self,
        inputs: &[ParabolicSARInput],
    ) -> Result<Vec<ParabolicSAROutput>, ParabolicSARError> {
        inputs.iter().map(|input| self.calculate(*input)).collect()
    }

    /// Reset the calculator state
    pub fn reset(&mut self) {
        self.state = ParabolicSARState::new(self.state.config);
    }

    /// Get current state (for serialization/debugging)
    pub fn get_state(&self) -> &ParabolicSARState {
        &self.state
    }

    /// Restore state (for deserialization)
    pub fn set_state(&mut self, state: ParabolicSARState) {
        self.state = state;
    }

    /// Get current trend direction
    pub fn current_trend(&self) -> Option<TrendDirection> {
        self.state.trend
    }

    /// Get current acceleration factor
    pub fn current_acceleration_factor(&self) -> f64 {
        self.state.acceleration_factor
    }

    // Private helper methods

    fn validate_input(&self, input: &ParabolicSARInput) -> Result<(), ParabolicSARError> {
        // Check for valid prices
        if !input.high.is_finite() || !input.low.is_finite() {
            return Err(ParabolicSARError::InvalidPrice);
        }

        // Check HL relationship
        if input.high < input.low {
            return Err(ParabolicSARError::InvalidHL);
        }

        // Check close if provided
        if let Some(close) = input.close {
            if !close.is_finite() {
                return Err(ParabolicSARError::InvalidPrice);
            }
            if close < input.low || close > input.high {
                return Err(ParabolicSARError::CloseOutOfRange);
            }
        }

        Ok(())
    }

    fn validate_config(&self) -> Result<(), ParabolicSARError> {
        let config = &self.state.config;

        if config.acceleration_start <= 0.0
            || config.acceleration_increment <= 0.0
            || config.acceleration_maximum <= config.acceleration_start
        {
            return Err(ParabolicSARError::InvalidAcceleration);
        }

        Ok(())
    }

    fn handle_first_calculation(
        &mut self,
        input: ParabolicSARInput,
    ) -> Result<ParabolicSAROutput, ParabolicSARError> {
        // First calculation - just store data, no SAR yet
        // Initial trend determination will happen on second calculation
        self.state.is_first = false;
        self.state.is_second = true;

        Ok(ParabolicSAROutput {
            sar: input.low,            // Placeholder - will be properly calculated next period
            trend: TrendDirection::Up, // Placeholder
            acceleration_factor: self.state.config.acceleration_start,
            extreme_point: input.high,
            trend_reversal: false,
            trend_periods: 1,
        })
    }

    fn handle_second_calculation(
        &mut self,
        input: ParabolicSARInput,
    ) -> Result<ParabolicSAROutput, ParabolicSARError> {
        // Second calculation - determine initial trend and set initial SAR
        self.state.is_second = false;

        let prev_high = self.state.previous_high.unwrap();
        let prev_low = self.state.previous_low.unwrap();

        // Determine initial trend direction
        let trend = if input.high > prev_high {
            TrendDirection::Up
        } else {
            TrendDirection::Down
        };

        // Set initial SAR and extreme point
        let (sar, extreme_point) = match trend {
            TrendDirection::Up => (prev_low, input.high.max(prev_high)),
            TrendDirection::Down => (prev_high, input.low.min(prev_low)),
        };

        self.state.trend = Some(trend);
        self.state.current_sar = Some(sar);
        self.state.extreme_point = Some(extreme_point);
        self.state.trend_periods = 1;

        Ok(ParabolicSAROutput {
            sar,
            trend,
            acceleration_factor: self.state.acceleration_factor,
            extreme_point,
            trend_reversal: false,
            trend_periods: self.state.trend_periods,
        })
    }

    fn handle_normal_calculation(
        &mut self,
        input: ParabolicSARInput,
    ) -> Result<ParabolicSAROutput, ParabolicSARError> {
        let current_trend = self.state.trend.unwrap();
        let current_sar = self.state.current_sar.unwrap();
        let current_ep = self.state.extreme_point.unwrap();

        // Check for trend reversal
        let trend_reversal = match current_trend {
            TrendDirection::Up => input.low <= current_sar,
            TrendDirection::Down => input.high >= current_sar,
        };

        if trend_reversal {
            self.handle_trend_reversal(input, current_trend, current_ep)
        } else {
            self.handle_trend_continuation(input, current_trend, current_sar, current_ep)
        }
    }

    fn handle_trend_reversal(
        &mut self,
        input: ParabolicSARInput,
        old_trend: TrendDirection,
        old_ep: f64,
    ) -> Result<ParabolicSAROutput, ParabolicSARError> {
        // Trend reversal - flip direction
        let new_trend = match old_trend {
            TrendDirection::Up => TrendDirection::Down,
            TrendDirection::Down => TrendDirection::Up,
        };

        // New SAR is the old extreme point
        let new_sar = old_ep;

        // New extreme point
        let new_ep = match new_trend {
            TrendDirection::Up => input.high,
            TrendDirection::Down => input.low,
        };

        // Reset acceleration factor
        self.state.acceleration_factor = self.state.config.acceleration_start;
        self.state.trend = Some(new_trend);
        self.state.current_sar = Some(new_sar);
        self.state.extreme_point = Some(new_ep);
        self.state.trend_periods = 1;

        Ok(ParabolicSAROutput {
            sar: new_sar,
            trend: new_trend,
            acceleration_factor: self.state.acceleration_factor,
            extreme_point: new_ep,
            trend_reversal: true,
            trend_periods: self.state.trend_periods,
        })
    }

    fn handle_trend_continuation(
        &mut self,
        input: ParabolicSARInput,
        trend: TrendDirection,
        current_sar: f64,
        current_ep: f64,
    ) -> Result<ParabolicSAROutput, ParabolicSARError> {
        // Check if we have a new extreme point
        let (new_ep, ep_updated) = match trend {
            TrendDirection::Up => {
                if input.high > current_ep {
                    (input.high, true)
                } else {
                    (current_ep, false)
                }
            }
            TrendDirection::Down => {
                if input.low < current_ep {
                    (input.low, true)
                } else {
                    (current_ep, false)
                }
            }
        };

        // Update acceleration factor if we have a new extreme point
        if ep_updated {
            self.state.acceleration_factor = (self.state.acceleration_factor
                + self.state.config.acceleration_increment)
                .min(self.state.config.acceleration_maximum);
        }

        // Calculate new SAR
        let mut new_sar = current_sar + self.state.acceleration_factor * (new_ep - current_sar);

        // Apply SAR rules to prevent SAR from moving into the price range
        new_sar = match trend {
            TrendDirection::Up => {
                // In uptrend, SAR cannot be above the low of current or previous period
                let prev_low = self.state.previous_low.unwrap_or(input.low);
                new_sar.min(input.low).min(prev_low)
            }
            TrendDirection::Down => {
                // In downtrend, SAR cannot be below the high of current or previous period
                let prev_high = self.state.previous_high.unwrap_or(input.high);
                new_sar.max(input.high).max(prev_high)
            }
        };

        self.state.current_sar = Some(new_sar);
        self.state.extreme_point = Some(new_ep);
        self.state.trend_periods += 1;

        Ok(ParabolicSAROutput {
            sar: new_sar,
            trend,
            acceleration_factor: self.state.acceleration_factor,
            extreme_point: new_ep,
            trend_reversal: false,
            trend_periods: self.state.trend_periods,
        })
    }

    fn update_state_after_calculation(&mut self, input: ParabolicSARInput) {
        self.state.previous_high = Some(input.high);
        self.state.previous_low = Some(input.low);
        if let Some(close) = input.close {
            self.state.previous_close = Some(close);
        }
    }
}

impl Default for ParabolicSAR {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to calculate Parabolic SAR for HL data without maintaining state
pub fn calculate_parabolic_sar_simple(
    highs: &[f64],
    lows: &[f64],
    acceleration_start: Option<f64>,
    acceleration_increment: Option<f64>,
    acceleration_maximum: Option<f64>,
) -> Result<Vec<f64>, ParabolicSARError> {
    if highs.len() != lows.len() {
        return Err(ParabolicSARError::InvalidInput(
            "Highs and lows must have same length".to_string(),
        ));
    }

    if highs.is_empty() {
        return Ok(Vec::new());
    }

    let config = ParabolicSARConfig {
        acceleration_start: acceleration_start.unwrap_or(0.02),
        acceleration_increment: acceleration_increment.unwrap_or(0.02),
        acceleration_maximum: acceleration_maximum.unwrap_or(0.20),
    };

    let mut sar_calculator = ParabolicSAR::with_config(config);
    let mut results = Vec::with_capacity(highs.len());

    for i in 0..highs.len() {
        let input = ParabolicSARInput {
            high: highs[i],
            low: lows[i],
            close: None,
        };
        let output = sar_calculator.calculate(input)?;
        results.push(output.sar);
    }

    Ok(results)
}
