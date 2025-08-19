use super::types::{ATRError, ATRResult, OHLCData};

/// Average True Range (ATR) Calculator
///
/// ATR measures market volatility by calculating the average of true ranges over a period.
/// True Range is the greatest of:
/// - Current High - Current Low
/// - |Current High - Previous Close|
/// - |Current Low - Previous Close|
pub struct ATR {
    period: usize,
    atr_value: Option<f64>,
    true_ranges: Vec<f64>,
    prev_close: Option<f64>,
    initialized: bool,
}

impl ATR {
    /// Creates a new ATR calculator with the specified period
    pub fn new(period: usize) -> Result<Self, ATRError> {
        if period == 0 {
            return Err(ATRError::InvalidPeriod);
        }

        Ok(Self {
            period,
            atr_value: None,
            true_ranges: Vec::with_capacity(period),
            prev_close: None,
            initialized: false,
        })
    }

    /// Updates ATR with OHLC data
    pub fn update(&mut self, data: OHLCData) -> Option<ATRResult> {
        // Validate input
        if data.high.is_nan() || data.low.is_nan() || data.close.is_nan() {
            return None;
        }
        if data.high.is_infinite() || data.low.is_infinite() || data.close.is_infinite() {
            return None;
        }
        if data.high < data.low {
            return None; // Invalid OHLC data
        }

        // Calculate True Range
        let true_range = self.calculate_true_range(data.high, data.low);

        // Store current close as previous close for next calculation
        self.prev_close = Some(data.close);

        // Calculate ATR
        self.calculate_atr(true_range)
    }

    /// Updates ATR with separate high, low, close values
    pub fn update_hlc(&mut self, high: f64, low: f64, close: f64) -> Option<ATRResult> {
        self.update(OHLCData::new(high, low, close))
    }

    /// Calculate with just closing price (simplified - less accurate)
    pub fn update_close_only(&mut self, close: f64) -> Option<ATRResult> {
        // Estimate high/low using close with a volatility factor
        let estimated_range = close * 0.01; // 1% estimated daily range
        let high = close + estimated_range / 2.0;
        let low = close - estimated_range / 2.0;

        self.update(OHLCData::new(high, low, close))
    }

    /// Returns the current ATR value
    pub fn value(&self) -> Option<f64> {
        self.atr_value
    }

    /// Returns the current ATR result with true range
    pub fn result(&self) -> Option<ATRResult> {
        self.atr_value.map(|atr| ATRResult {
            atr,
            true_range: self.true_ranges.last().copied().unwrap_or(0.0),
        })
    }

    /// Resets the calculator
    pub fn reset(&mut self) {
        self.atr_value = None;
        self.true_ranges.clear();
        self.prev_close = None;
        self.initialized = false;
    }

    /// Calculate True Range
    fn calculate_true_range(&self, high: f64, low: f64) -> f64 {
        if let Some(prev_close) = self.prev_close {
            // True Range = max of:
            // 1. Current High - Current Low
            // 2. |Current High - Previous Close|
            // 3. |Current Low - Previous Close|
            let hl = high - low;
            let hc = (high - prev_close).abs();
            let lc = (low - prev_close).abs();

            hl.max(hc).max(lc)
        } else {
            // First candle: use High - Low
            high - low
        }
    }

    /// Calculate ATR using Wilder's smoothing method
    fn calculate_atr(&mut self, true_range: f64) -> Option<ATRResult> {
        if !self.initialized {
            // Initial ATR calculation: Simple average of first N true ranges
            self.true_ranges.push(true_range);

            if self.true_ranges.len() >= self.period {
                let initial_atr = self.true_ranges.iter().sum::<f64>() / self.period as f64;
                self.atr_value = Some(initial_atr);
                self.initialized = true;

                return Some(ATRResult { atr: initial_atr, true_range });
            }

            None
        } else {
            // Wilder's smoothing: ATR = ((ATR_prev * (n-1)) + TR) / n
            if let Some(prev_atr) = self.atr_value {
                let new_atr = (prev_atr * (self.period - 1) as f64 + true_range) / self.period as f64;
                self.atr_value = Some(new_atr);

                // Keep only the last true range for reference
                self.true_ranges.clear();
                self.true_ranges.push(true_range);

                Some(ATRResult { atr: new_atr, true_range })
            } else {
                None
            }
        }
    }

    /// Get the period
    pub fn period(&self) -> usize {
        self.period
    }

    /// Check if ATR is initialized
    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Batch calculation for historical data
    pub fn calculate_batch(period: usize, data: &[OHLCData]) -> Result<Vec<Option<ATRResult>>, ATRError> {
        let mut atr = Self::new(period)?;
        let mut results = Vec::with_capacity(data.len());

        for ohlc in data {
            results.push(atr.update(*ohlc));
        }

        Ok(results)
    }
}
