use crate::v2::obv::types::{OBVConfig, OBVError, OBVInput, OBVOutput, OBVState};

/// On Balance Volume (OBV) Indicator
///
/// OBV is a momentum indicator that uses volume flow to predict changes in stock price.
/// It adds volume on up days and subtracts volume on down days.
///
/// Formula:
/// - If Close > Previous Close: OBV = Previous OBV + Volume
/// - If Close < Previous Close: OBV = Previous OBV - Volume
/// - If Close = Previous Close: OBV = Previous OBV
#[derive(Default)]
pub struct OBV {
    state: OBVState,
}

impl OBV {
    /// Create a new OBV calculator with default configuration
    pub fn new() -> Self {
        Self::with_config(OBVConfig::default())
    }

    /// Create a new OBV calculator with custom configuration
    pub fn with_config(config: OBVConfig) -> Self {
        Self {
            state: OBVState::new(config),
        }
    }

    /// Calculate OBV for the given input
    pub fn calculate(&mut self, input: OBVInput) -> Result<OBVOutput, OBVError> {
        // Validate input
        self.validate_input(&input)?;

        let flow_direction = if self.state.is_first {
            // First calculation - no direction yet
            self.state.previous_close = Some(input.close);
            self.state.cumulative_obv = input.volume;
            self.state.is_first = false;
            0.0
        } else {
            let prev_close = self.state.previous_close.unwrap();
            let direction = self.determine_flow_direction(input.close, prev_close);

            // Update OBV based on price direction
            match direction {
                d if d > 0.0 => {
                    // Price went up - add volume
                    self.state.cumulative_obv += input.volume;
                }
                d if d < 0.0 => {
                    // Price went down - subtract volume
                    self.state.cumulative_obv -= input.volume;
                }
                _ => {
                    // Price unchanged - OBV stays the same
                }
            }

            self.state.previous_close = Some(input.close);
            direction
        };

        Ok(OBVOutput {
            obv: self.state.cumulative_obv,
            flow_direction,
        })
    }

    /// Calculate OBV for a batch of inputs
    pub fn calculate_batch(&mut self, inputs: &[OBVInput]) -> Result<Vec<OBVOutput>, OBVError> {
        inputs.iter().map(|input| self.calculate(*input)).collect()
    }

    /// Reset the calculator state
    pub fn reset(&mut self) {
        self.state = OBVState::new(self.state.config);
    }

    /// Get current state (for serialization/debugging)
    pub fn get_state(&self) -> &OBVState {
        &self.state
    }

    /// Restore state (for deserialization)
    pub fn set_state(&mut self, state: OBVState) {
        self.state = state;
    }

    // Private helper methods

    fn validate_input(&self, input: &OBVInput) -> Result<(), OBVError> {
        if input.volume < 0.0 {
            return Err(OBVError::NegativeVolume);
        }

        if !input.close.is_finite() {
            return Err(OBVError::InvalidPrice);
        }

        Ok(())
    }

    fn determine_flow_direction(&self, current_close: f64, previous_close: f64) -> f64 {
        if current_close > previous_close {
            1.0 // Up
        } else if current_close < previous_close {
            -1.0 // Down
        } else {
            0.0 // Unchanged
        }
    }
}

/// Convenience function to calculate OBV for a single input without maintaining state
pub fn calculate_obv_simple(close_prices: &[f64], volumes: &[f64]) -> Result<Vec<f64>, OBVError> {
    if close_prices.len() != volumes.len() {
        return Err(OBVError::InvalidInput(
            "Close prices and volumes must have same length".to_string(),
        ));
    }

    if close_prices.is_empty() {
        return Ok(Vec::new());
    }

    let mut obv_calculator = OBV::new();
    let mut results = Vec::with_capacity(close_prices.len());

    for (close, volume) in close_prices.iter().zip(volumes.iter()) {
        let input = OBVInput {
            close: *close,
            volume: *volume,
        };
        let output = obv_calculator.calculate(input)?;
        results.push(output.obv);
    }

    Ok(results)
}
