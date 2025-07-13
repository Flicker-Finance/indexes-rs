use crate::v2::mfi::types::{
    MFIConfig, MFIError, MFIInput, MFIMarketCondition, MFIOutput, MFIState, MoneyFlow,
};

/// Money Flow Index (MFI) Indicator
///
/// MFI is a momentum indicator that uses both price and volume to identify
/// overbought or oversold conditions. It's often called "Volume-weighted RSI".
///
/// Formula:
/// 1. Typical Price = (High + Low + Close) / 3
/// 2. Raw Money Flow = Typical Price × Volume
/// 3. Money Flow Direction: Positive if current TP > previous TP, else Negative
/// 4. Money Ratio = (Positive Money Flow Sum) / (Negative Money Flow Sum)
/// 5. MFI = 100 - (100 / (1 + Money Ratio))
pub struct MFI {
    state: MFIState,
}

impl MFI {
    /// Create a new MFI calculator with default configuration (period=14)
    pub fn new() -> Self {
        Self::with_config(MFIConfig::default())
    }

    /// Create a new MFI calculator with custom period
    pub fn with_period(period: usize) -> Result<Self, MFIError> {
        if period == 0 {
            return Err(MFIError::InvalidPeriod);
        }

        let config = MFIConfig {
            period,
            ..Default::default()
        };
        Ok(Self::with_config(config))
    }

    /// Create a new MFI calculator with custom configuration
    pub fn with_config(config: MFIConfig) -> Self {
        Self {
            state: MFIState::new(config),
        }
    }

    /// Calculate MFI for the given input
    pub fn calculate(&mut self, input: MFIInput) -> Result<MFIOutput, MFIError> {
        // Validate input
        self.validate_input(&input)?;
        self.validate_config()?;

        // Calculate typical price
        let typical_price = self.calculate_typical_price(&input);

        // Calculate raw money flow
        let raw_money_flow = typical_price * input.volume;

        // Determine money flow direction
        let flow_direction = self.determine_flow_direction(typical_price);

        // Create money flow data point
        let money_flow = MoneyFlow {
            typical_price,
            raw_money_flow,
            flow_direction,
        };

        // Add to history and update sums
        self.update_money_flow_history(money_flow);

        // Calculate MFI if we have enough data
        let mfi = if self.state.has_sufficient_data {
            self.calculate_mfi_value()?
        } else {
            50.0 // Default neutral value when insufficient data
        };

        // Determine market condition
        let market_condition = self.determine_market_condition(mfi);

        // Update previous typical price
        self.state.previous_typical_price = Some(typical_price);

        Ok(MFIOutput {
            mfi,
            typical_price,
            raw_money_flow,
            flow_direction,
            market_condition,
        })
    }

    /// Calculate MFI for a batch of inputs
    pub fn calculate_batch(&mut self, inputs: &[MFIInput]) -> Result<Vec<MFIOutput>, MFIError> {
        inputs.iter().map(|input| self.calculate(*input)).collect()
    }

    /// Reset the calculator state
    pub fn reset(&mut self) {
        self.state = MFIState::new(self.state.config);
    }

    /// Get current state (for serialization/debugging)
    pub fn get_state(&self) -> &MFIState {
        &self.state
    }

    /// Restore state (for deserialization)
    pub fn set_state(&mut self, state: MFIState) {
        self.state = state;
    }

    /// Get current positive money flow sum
    pub fn positive_money_flow(&self) -> f64 {
        self.state.positive_money_flow_sum
    }

    /// Get current negative money flow sum
    pub fn negative_money_flow(&self) -> f64 {
        self.state.negative_money_flow_sum
    }

    // Private helper methods

    fn validate_input(&self, input: &MFIInput) -> Result<(), MFIError> {
        // Check for valid prices
        if !input.high.is_finite() || !input.low.is_finite() || !input.close.is_finite() {
            return Err(MFIError::InvalidPrice);
        }

        // Check OHLC relationship
        if input.high < input.low {
            return Err(MFIError::InvalidOHLC);
        }

        if input.close < input.low || input.close > input.high {
            return Err(MFIError::InvalidOHLC);
        }

        // Check volume
        if input.volume < 0.0 {
            return Err(MFIError::NegativeVolume);
        }

        Ok(())
    }

    fn validate_config(&self) -> Result<(), MFIError> {
        if self.state.config.period == 0 {
            return Err(MFIError::InvalidPeriod);
        }

        if self.state.config.overbought <= self.state.config.oversold {
            return Err(MFIError::InvalidThresholds);
        }

        if self.state.config.overbought > 100.0 || self.state.config.oversold < 0.0 {
            return Err(MFIError::InvalidThresholds);
        }

        Ok(())
    }

    fn calculate_typical_price(&self, input: &MFIInput) -> f64 {
        (input.high + input.low + input.close) / 3.0
    }

    fn determine_flow_direction(&self, current_typical_price: f64) -> f64 {
        match self.state.previous_typical_price {
            Some(prev_tp) => {
                if current_typical_price > prev_tp {
                    1.0 // Positive money flow
                } else if current_typical_price < prev_tp {
                    -1.0 // Negative money flow
                } else {
                    0.0 // Neutral (unchanged)
                }
            }
            None => 0.0, // First calculation - neutral
        }
    }

    fn update_money_flow_history(&mut self, money_flow: MoneyFlow) {
        // Remove oldest if at capacity
        if self.state.money_flows.len() >= self.state.config.period {
            if let Some(oldest) = self.state.money_flows.pop_front() {
                // Remove from sums
                if oldest.flow_direction > 0.0 {
                    self.state.positive_money_flow_sum -= oldest.raw_money_flow;
                } else if oldest.flow_direction < 0.0 {
                    self.state.negative_money_flow_sum -= oldest.raw_money_flow;
                }
            }
        }

        // Add new money flow
        if money_flow.flow_direction > 0.0 {
            self.state.positive_money_flow_sum += money_flow.raw_money_flow;
        } else if money_flow.flow_direction < 0.0 {
            self.state.negative_money_flow_sum += money_flow.raw_money_flow;
        }

        self.state.money_flows.push_back(money_flow);

        // Check if we have sufficient data
        self.state.has_sufficient_data = self.state.money_flows.len() >= self.state.config.period;
    }

    fn calculate_mfi_value(&self) -> Result<f64, MFIError> {
        if self.state.negative_money_flow_sum == 0.0 {
            // All positive money flow
            return Ok(100.0);
        }

        if self.state.positive_money_flow_sum == 0.0 {
            // All negative money flow
            return Ok(0.0);
        }

        let money_ratio = self.state.positive_money_flow_sum / self.state.negative_money_flow_sum;
        let mfi = 100.0 - (100.0 / (1.0 + money_ratio));

        if !mfi.is_finite() {
            return Err(MFIError::DivisionByZero);
        }

        Ok(mfi)
    }

    fn determine_market_condition(&self, mfi: f64) -> MFIMarketCondition {
        if !self.state.has_sufficient_data {
            MFIMarketCondition::Insufficient
        } else if mfi >= self.state.config.overbought {
            MFIMarketCondition::Overbought
        } else if mfi <= self.state.config.oversold {
            MFIMarketCondition::Oversold
        } else {
            MFIMarketCondition::Normal
        }
    }
}

impl Default for MFI {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to calculate MFI for OHLCV data without maintaining state
pub fn calculate_mfi_simple(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    volumes: &[f64],
    period: usize,
) -> Result<Vec<f64>, MFIError> {
    let len = highs.len();
    if len != lows.len() || len != closes.len() || len != volumes.len() {
        return Err(MFIError::InvalidInput(
            "All price and volume arrays must have same length".to_string(),
        ));
    }

    if len == 0 {
        return Ok(Vec::new());
    }

    let mut mfi_calculator = MFI::with_period(period)?;
    let mut results = Vec::with_capacity(len);

    for i in 0..len {
        let input = MFIInput {
            high: highs[i],
            low: lows[i],
            close: closes[i],
            volume: volumes[i],
        };
        let output = mfi_calculator.calculate(input)?;
        results.push(output.mfi);
    }

    Ok(results)
}
