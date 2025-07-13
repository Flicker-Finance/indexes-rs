use crate::v2::adx::types::{
    ADXConfig, ADXError, ADXInput, ADXOutput, ADXPeriodData, ADXState, TrendDirection,
    TrendStrength,
};

/// Average Directional Index (ADX) Indicator
///
/// ADX measures trend strength regardless of direction. It's composed of:
/// - True Range (TR): Max of (H-L), (H-Cp), (Cp-L)
/// - Directional Movement: +DM = H-Hp (if positive), -DM = Lp-L (if positive)
/// - Directional Indicators: +DI = (+DM smoothed / TR smoothed) * 100
/// - Directional Index: DX = (|+DI - -DI| / (+DI + -DI)) * 100
/// - ADX = Smoothed average of DX values
///
/// ADX interpretation:
/// - 0-25: Weak trend or ranging market
/// - 25-50: Strong trend
/// - 50+: Very strong trend
pub struct ADX {
    state: ADXState,
}

impl ADX {
    /// Create a new ADX calculator with default configuration (period=14)
    pub fn new() -> Self {
        Self::with_config(ADXConfig::default())
    }

    /// Create a new ADX calculator with custom period
    pub fn with_period(period: usize) -> Result<Self, ADXError> {
        if period == 0 {
            return Err(ADXError::InvalidPeriod);
        }

        let config = ADXConfig {
            period,
            adx_smoothing: period,
            ..Default::default()
        };
        Ok(Self::with_config(config))
    }

    /// Create a new ADX calculator with custom period and ADX smoothing
    pub fn with_periods(period: usize, adx_smoothing: usize) -> Result<Self, ADXError> {
        if period == 0 || adx_smoothing == 0 {
            return Err(ADXError::InvalidPeriod);
        }

        let config = ADXConfig {
            period,
            adx_smoothing,
            ..Default::default()
        };
        Ok(Self::with_config(config))
    }

    /// Create a new ADX calculator with custom configuration
    pub fn with_config(config: ADXConfig) -> Self {
        Self {
            state: ADXState::new(config),
        }
    }

    /// Calculate ADX for the given input
    pub fn calculate(&mut self, input: ADXInput) -> Result<ADXOutput, ADXError> {
        // Validate input
        self.validate_input(&input)?;
        self.validate_config()?;

        if self.state.is_first {
            self.handle_first_calculation(input)
        } else {
            self.handle_normal_calculation(input)
        }
    }

    /// Calculate ADX for a batch of inputs
    pub fn calculate_batch(&mut self, inputs: &[ADXInput]) -> Result<Vec<ADXOutput>, ADXError> {
        inputs.iter().map(|input| self.calculate(*input)).collect()
    }

    /// Reset the calculator state
    pub fn reset(&mut self) {
        self.state = ADXState::new(self.state.config);
    }

    /// Get current state (for serialization/debugging)
    pub fn get_state(&self) -> &ADXState {
        &self.state
    }

    /// Restore state (for deserialization)
    pub fn set_state(&mut self, state: ADXState) {
        self.state = state;
    }

    /// Get current trend strength
    pub fn trend_strength(&self) -> TrendStrength {
        if let Some(adx) = self.state.current_adx {
            self.classify_trend_strength(adx)
        } else {
            TrendStrength::Insufficient
        }
    }

    /// Get current trend direction
    pub fn trend_direction(&self) -> Option<TrendDirection> {
        self.state
            .period_data
            .back()
            .map(|last_data| self.determine_trend_direction(last_data.plus_di, last_data.minus_di))
    }

    // Private helper methods

    fn validate_input(&self, input: &ADXInput) -> Result<(), ADXError> {
        // Check for valid prices
        if !input.high.is_finite() || !input.low.is_finite() || !input.close.is_finite() {
            return Err(ADXError::InvalidPrice);
        }

        // Check HLC relationship
        if input.high < input.low {
            return Err(ADXError::InvalidHLC);
        }

        if input.close < input.low || input.close > input.high {
            return Err(ADXError::InvalidHLC);
        }

        Ok(())
    }

    fn validate_config(&self) -> Result<(), ADXError> {
        if self.state.config.period == 0 || self.state.config.adx_smoothing == 0 {
            return Err(ADXError::InvalidPeriod);
        }

        if self.state.config.strong_trend_threshold >= self.state.config.very_strong_trend_threshold
        {
            return Err(ADXError::InvalidThresholds);
        }

        Ok(())
    }

    fn handle_first_calculation(&mut self, input: ADXInput) -> Result<ADXOutput, ADXError> {
        // First calculation - just store data
        self.state.previous_high = Some(input.high);
        self.state.previous_low = Some(input.low);
        self.state.previous_close = Some(input.close);
        self.state.is_first = false;

        // Return default values for first calculation
        Ok(ADXOutput {
            adx: 0.0,
            plus_di: 0.0,
            minus_di: 0.0,
            dx: 0.0,
            true_range: 0.0,
            trend_strength: TrendStrength::Insufficient,
            trend_direction: TrendDirection::Sideways,
            di_spread: 0.0,
        })
    }

    fn handle_normal_calculation(&mut self, input: ADXInput) -> Result<ADXOutput, ADXError> {
        // Calculate True Range
        let true_range = self.calculate_true_range(&input);

        // Calculate Directional Movements
        let (plus_dm, minus_dm) = self.calculate_directional_movements(&input);

        // Update or initialize smoothed values
        if self.state.period_data.len() < self.state.config.period {
            // Not enough data for smoothing yet - accumulate
            self.accumulate_initial_data(true_range, plus_dm, minus_dm);
        } else {
            // Use smoothing formula
            self.update_smoothed_values(true_range, plus_dm, minus_dm);
        }

        // Calculate DI values
        let (plus_di, minus_di) = self.calculate_directional_indicators();

        // Calculate DX
        let dx = self.calculate_dx(plus_di, minus_di)?;

        // Calculate ADX
        let adx = self.calculate_adx(dx);

        // Create period data
        let period_data = ADXPeriodData {
            true_range,
            plus_dm,
            minus_dm,
            plus_di,
            minus_di,
            dx,
        };

        // Store period data
        if self.state.period_data.len() >= self.state.config.period {
            self.state.period_data.pop_front();
        }
        self.state.period_data.push_back(period_data);

        // Update state
        self.state.previous_high = Some(input.high);
        self.state.previous_low = Some(input.low);
        self.state.previous_close = Some(input.close);

        // Determine outputs
        let trend_strength = self.classify_trend_strength(adx);
        let trend_direction = self.determine_trend_direction(plus_di, minus_di);
        let di_spread = plus_di - minus_di;

        Ok(ADXOutput {
            adx,
            plus_di,
            minus_di,
            dx,
            true_range,
            trend_strength,
            trend_direction,
            di_spread,
        })
    }

    fn calculate_true_range(&self, input: &ADXInput) -> f64 {
        if let Some(prev_close) = self.state.previous_close {
            let hl = input.high - input.low;
            let hc = (input.high - prev_close).abs();
            let lc = (input.low - prev_close).abs();
            hl.max(hc).max(lc)
        } else {
            input.high - input.low
        }
    }

    fn calculate_directional_movements(&self, input: &ADXInput) -> (f64, f64) {
        if let (Some(prev_high), Some(prev_low)) =
            (self.state.previous_high, self.state.previous_low)
        {
            let up_move = input.high - prev_high;
            let down_move = prev_low - input.low;

            let plus_dm = if up_move > down_move && up_move > 0.0 {
                up_move
            } else {
                0.0
            };

            let minus_dm = if down_move > up_move && down_move > 0.0 {
                down_move
            } else {
                0.0
            };

            (plus_dm, minus_dm)
        } else {
            (0.0, 0.0)
        }
    }

    fn accumulate_initial_data(&mut self, true_range: f64, plus_dm: f64, minus_dm: f64) {
        // For the first period values, we sum them up
        match self.state.smoothed_tr {
            Some(tr) => self.state.smoothed_tr = Some(tr + true_range),
            None => self.state.smoothed_tr = Some(true_range),
        }

        match self.state.smoothed_plus_dm {
            Some(dm) => self.state.smoothed_plus_dm = Some(dm + plus_dm),
            None => self.state.smoothed_plus_dm = Some(plus_dm),
        }

        match self.state.smoothed_minus_dm {
            Some(dm) => self.state.smoothed_minus_dm = Some(dm + minus_dm),
            None => self.state.smoothed_minus_dm = Some(minus_dm),
        }

        // Check if we have enough data for DI calculation
        if self.state.period_data.len() + 1 >= self.state.config.period {
            self.state.has_di_data = true;
        }
    }

    fn update_smoothed_values(&mut self, true_range: f64, plus_dm: f64, minus_dm: f64) {
        let period = self.state.config.period as f64;

        // Wilder's smoothing: New = (Old * (n-1) + Current) / n
        if let Some(smoothed_tr) = self.state.smoothed_tr {
            self.state.smoothed_tr = Some((smoothed_tr * (period - 1.0) + true_range) / period);
        }

        if let Some(smoothed_plus_dm) = self.state.smoothed_plus_dm {
            self.state.smoothed_plus_dm =
                Some((smoothed_plus_dm * (period - 1.0) + plus_dm) / period);
        }

        if let Some(smoothed_minus_dm) = self.state.smoothed_minus_dm {
            self.state.smoothed_minus_dm =
                Some((smoothed_minus_dm * (period - 1.0) + minus_dm) / period);
        }
    }

    fn calculate_directional_indicators(&self) -> (f64, f64) {
        if let (Some(smoothed_tr), Some(smoothed_plus_dm), Some(smoothed_minus_dm)) = (
            self.state.smoothed_tr,
            self.state.smoothed_plus_dm,
            self.state.smoothed_minus_dm,
        ) {
            if smoothed_tr != 0.0 {
                let plus_di = (smoothed_plus_dm / smoothed_tr) * 100.0;
                let minus_di = (smoothed_minus_dm / smoothed_tr) * 100.0;
                (plus_di, minus_di)
            } else {
                (0.0, 0.0)
            }
        } else {
            (0.0, 0.0)
        }
    }

    fn calculate_dx(&self, plus_di: f64, minus_di: f64) -> Result<f64, ADXError> {
        let di_sum = plus_di + minus_di;
        if di_sum == 0.0 {
            Ok(0.0)
        } else {
            let di_diff = (plus_di - minus_di).abs();
            Ok((di_diff / di_sum) * 100.0)
        }
    }

    fn calculate_adx(&mut self, dx: f64) -> f64 {
        // Add DX to history
        if self.state.dx_history.len() >= self.state.config.adx_smoothing {
            self.state.dx_history.pop_front();
        }
        self.state.dx_history.push_back(dx);

        // Calculate ADX
        if self.state.dx_history.len() >= self.state.config.adx_smoothing {
            if !self.state.has_adx_data {
                // First ADX calculation - simple average
                let adx =
                    self.state.dx_history.iter().sum::<f64>() / self.state.dx_history.len() as f64;
                self.state.current_adx = Some(adx);
                self.state.has_adx_data = true;
                adx
            } else {
                // Subsequent ADX calculations - use smoothing
                if let Some(prev_adx) = self.state.current_adx {
                    let period = self.state.config.adx_smoothing as f64;
                    let adx = (prev_adx * (period - 1.0) + dx) / period;
                    self.state.current_adx = Some(adx);
                    adx
                } else {
                    0.0
                }
            }
        } else {
            0.0
        }
    }

    fn classify_trend_strength(&self, adx: f64) -> TrendStrength {
        if !self.state.has_adx_data {
            TrendStrength::Insufficient
        } else if adx >= self.state.config.very_strong_trend_threshold {
            TrendStrength::VeryStrong
        } else if adx >= self.state.config.strong_trend_threshold {
            TrendStrength::Strong
        } else {
            TrendStrength::Weak
        }
    }

    fn determine_trend_direction(&self, plus_di: f64, minus_di: f64) -> TrendDirection {
        if plus_di > minus_di {
            TrendDirection::Up
        } else if minus_di > plus_di {
            TrendDirection::Down
        } else {
            TrendDirection::Sideways
        }
    }
}

impl Default for ADX {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to calculate ADX for HLC data without maintaining state
pub fn calculate_adx_simple(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    period: usize,
) -> Result<Vec<f64>, ADXError> {
    let len = highs.len();
    if len != lows.len() || len != closes.len() {
        return Err(ADXError::InvalidInput(
            "All price arrays must have same length".to_string(),
        ));
    }

    if len == 0 {
        return Ok(Vec::new());
    }

    let mut adx_calculator = ADX::with_period(period)?;
    let mut results = Vec::with_capacity(len);

    for i in 0..len {
        let input = ADXInput {
            high: highs[i],
            low: lows[i],
            close: closes[i],
        };
        let output = adx_calculator.calculate(input)?;
        results.push(output.adx);
    }

    Ok(results)
}
