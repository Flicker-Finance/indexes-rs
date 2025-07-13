use crate::v2::std_dev::types::{
    StandardDeviationConfig, StandardDeviationError, StandardDeviationInput,
    StandardDeviationOutput, StandardDeviationState, VolatilityLevel,
};

/// Standard Deviation Indicator
///
/// Standard deviation measures the amount of variation or dispersion in a set of values.
/// It's a fundamental statistical measure used in many other technical indicators.
///
/// Formulas:
/// - Population Standard Deviation: σ = √(Σ(x - μ)² / N)
/// - Sample Standard Deviation: s = √(Σ(x - μ)² / (N-1))
///
/// Where:
/// - x = individual values
/// - μ = mean of the values
/// - N = number of values
///
/// Uses:
/// - Measuring price volatility
/// - Bollinger Bands calculation
/// - Risk assessment
/// - Normalization of other indicators
/// - Z-score calculations
pub struct StandardDeviation {
    state: StandardDeviationState,
}

impl StandardDeviation {
    /// Create a new Standard Deviation calculator with default configuration (period=20, sample=true)
    pub fn new() -> Self {
        Self::with_config(StandardDeviationConfig::default())
    }

    /// Create a new Standard Deviation calculator with custom period
    pub fn with_period(period: usize) -> Result<Self, StandardDeviationError> {
        if period == 0 {
            return Err(StandardDeviationError::InvalidPeriod);
        }

        let config = StandardDeviationConfig {
            period,
            ..Default::default()
        };
        Ok(Self::with_config(config))
    }

    /// Create a new Standard Deviation calculator for population standard deviation
    pub fn population(period: usize) -> Result<Self, StandardDeviationError> {
        if period == 0 {
            return Err(StandardDeviationError::InvalidPeriod);
        }

        let config = StandardDeviationConfig {
            period,
            use_sample: false,
        };
        Ok(Self::with_config(config))
    }

    /// Create a new Standard Deviation calculator for sample standard deviation
    pub fn sample(period: usize) -> Result<Self, StandardDeviationError> {
        if period <= 1 {
            return Err(StandardDeviationError::InvalidPeriod);
        }

        let config = StandardDeviationConfig {
            period,
            use_sample: true,
        };
        Ok(Self::with_config(config))
    }

    /// Create a new Standard Deviation calculator with custom configuration
    pub fn with_config(config: StandardDeviationConfig) -> Self {
        Self {
            state: StandardDeviationState::new(config),
        }
    }

    /// Calculate Standard Deviation for the given input
    pub fn calculate(
        &mut self,
        input: StandardDeviationInput,
    ) -> Result<StandardDeviationOutput, StandardDeviationError> {
        // Validate input
        self.validate_input(&input)?;
        self.validate_config()?;

        // Update value history
        self.update_value_history(input.value);

        // Calculate standard deviation if we have enough data
        let (std_dev, variance, mean) = if self.state.has_sufficient_data {
            self.calculate_standard_deviation()?
        } else {
            (0.0, 0.0, input.value) // Default values when insufficient data
        };

        // Calculate derived metrics
        let z_score = if std_dev != 0.0 {
            (input.value - mean) / std_dev
        } else {
            0.0
        };

        let coefficient_of_variation = if mean != 0.0 {
            (std_dev / mean.abs()) * 100.0
        } else {
            0.0
        };

        // Classify volatility level
        let volatility_level = self.classify_volatility(std_dev, mean);

        Ok(StandardDeviationOutput {
            std_dev,
            variance,
            mean,
            current_value: input.value,
            z_score,
            coefficient_of_variation,
            volatility_level,
        })
    }

    /// Calculate Standard Deviation for a batch of inputs
    pub fn calculate_batch(
        &mut self,
        inputs: &[StandardDeviationInput],
    ) -> Result<Vec<StandardDeviationOutput>, StandardDeviationError> {
        inputs.iter().map(|input| self.calculate(*input)).collect()
    }

    /// Reset the calculator state
    pub fn reset(&mut self) {
        self.state = StandardDeviationState::new(self.state.config);
    }

    /// Get current state (for serialization/debugging)
    pub fn get_state(&self) -> &StandardDeviationState {
        &self.state
    }

    /// Restore state (for deserialization)
    pub fn set_state(&mut self, state: StandardDeviationState) {
        self.state = state;
    }

    /// Get current mean
    pub fn mean(&self) -> f64 {
        self.state.current_mean
    }

    /// Get current volatility level
    pub fn volatility_level(&self) -> VolatilityLevel {
        if !self.state.has_sufficient_data {
            VolatilityLevel::Insufficient
        } else {
            // Would need last std_dev to classify properly
            VolatilityLevel::Normal
        }
    }

    /// Check if value is outlier (beyond N standard deviations)
    pub fn is_outlier(&self, value: f64, threshold_std_devs: f64) -> bool {
        if !self.state.has_sufficient_data {
            return false;
        }

        // Use current state values instead of recalculating
        let mean = self.state.current_mean;

        // Calculate current std dev from state
        if let Ok((std_dev, _, _)) = self.calculate_standard_deviation() {
            if std_dev == 0.0 {
                return false;
            }
            let z_score = (value - mean) / std_dev;
            z_score.abs() > threshold_std_devs
        } else {
            false
        }
    }

    // Private helper methods

    fn validate_input(&self, input: &StandardDeviationInput) -> Result<(), StandardDeviationError> {
        if !input.value.is_finite() {
            return Err(StandardDeviationError::InvalidValue);
        }
        Ok(())
    }

    fn validate_config(&self) -> Result<(), StandardDeviationError> {
        if self.state.config.period == 0 {
            return Err(StandardDeviationError::InvalidPeriod);
        }

        if self.state.config.use_sample && self.state.config.period <= 1 {
            return Err(StandardDeviationError::InvalidPeriod);
        }

        Ok(())
    }

    fn update_value_history(&mut self, value: f64) {
        // Remove oldest if at capacity
        if self.state.values.len() >= self.state.config.period {
            if let Some(oldest) = self.state.values.pop_front() {
                self.state.sum -= oldest;
                self.state.sum_squared -= oldest * oldest;
            }
        }

        // Add new value
        self.state.values.push_back(value);
        self.state.sum += value;
        self.state.sum_squared += value * value;

        // Update mean
        if !self.state.values.is_empty() {
            self.state.current_mean = self.state.sum / self.state.values.len() as f64;
        }

        // Check if we have sufficient data
        let min_required = if self.state.config.use_sample { 2 } else { 1 };
        self.state.has_sufficient_data =
            self.state.values.len() >= self.state.config.period.max(min_required);
    }

    fn calculate_standard_deviation(&self) -> Result<(f64, f64, f64), StandardDeviationError> {
        if !self.state.has_sufficient_data {
            return Ok((0.0, 0.0, self.state.current_mean));
        }

        let n = self.state.values.len() as f64;
        let mean = self.state.sum / n;

        // Calculate variance using the computational formula
        // Var = (Σx²)/n - (Σx/n)²
        let variance_raw = (self.state.sum_squared / n) - (mean * mean);

        // Apply sample vs population correction
        let variance = if self.state.config.use_sample && n > 1.0 {
            // Sample variance: multiply by n/(n-1)
            variance_raw * n / (n - 1.0)
        } else {
            // Population variance
            variance_raw
        };

        // Ensure variance is non-negative (due to floating point precision)
        let variance = variance.max(0.0);

        let std_dev = variance.sqrt();

        if !std_dev.is_finite() {
            return Err(StandardDeviationError::DivisionByZero);
        }

        Ok((std_dev, variance, mean))
    }

    fn classify_volatility(&self, std_dev: f64, mean: f64) -> VolatilityLevel {
        if !self.state.has_sufficient_data {
            return VolatilityLevel::Insufficient;
        }

        // Use coefficient of variation for relative volatility measurement
        let cv = if mean != 0.0 {
            (std_dev / mean.abs()) * 100.0
        } else {
            0.0
        };

        // Classification based on coefficient of variation
        match cv {
            cv if cv < 5.0 => VolatilityLevel::VeryLow,
            cv if cv < 15.0 => VolatilityLevel::Low,
            cv if cv < 25.0 => VolatilityLevel::Normal,
            cv if cv < 50.0 => VolatilityLevel::High,
            _ => VolatilityLevel::VeryHigh,
        }
    }
}

impl Default for StandardDeviation {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to calculate standard deviation for a series of values
pub fn calculate_standard_deviation_simple(
    values: &[f64],
    period: usize,
    use_sample: bool,
) -> Result<Vec<f64>, StandardDeviationError> {
    if values.is_empty() {
        return Ok(Vec::new());
    }

    let config = StandardDeviationConfig { period, use_sample };

    let mut std_dev_calculator = StandardDeviation::with_config(config);
    let mut results = Vec::with_capacity(values.len());

    for &value in values {
        let input = StandardDeviationInput { value };
        let output = std_dev_calculator.calculate(input)?;
        results.push(output.std_dev);
    }

    Ok(results)
}

/// Calculate rolling standard deviation over a window
pub fn rolling_standard_deviation(
    values: &[f64],
    window: usize,
    use_sample: bool,
) -> Result<Vec<f64>, StandardDeviationError> {
    if values.is_empty() || window == 0 {
        return Ok(Vec::new());
    }

    if values.len() < window {
        return Ok(vec![0.0; values.len()]);
    }

    let mut results = Vec::with_capacity(values.len());

    for i in 0..values.len() {
        let start = if i + 1 >= window { i + 1 - window } else { 0 };
        let end = i + 1;
        let window_values = &values[start..end];

        if window_values.len() >= if use_sample { 2 } else { 1 } {
            let mean = window_values.iter().sum::<f64>() / window_values.len() as f64;
            let variance = window_values
                .iter()
                .map(|x| (x - mean).powi(2))
                .sum::<f64>()
                / if use_sample && window_values.len() > 1 {
                    window_values.len() - 1
                } else {
                    window_values.len()
                } as f64;

            results.push(variance.sqrt());
        } else {
            results.push(0.0);
        }
    }

    Ok(results)
}
