//! Types for the Simple Moving Average (SMA) calculator.
//!
use serde::Serialize;

use crate::v1::types::TrendDirection;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct SMAResult {
    pub value: f64,
    pub trend: TrendDirection,
}

/// An error type for the SimpleMovingAverage.
#[derive(Debug, PartialEq)]
pub enum SMAError {
    /// Indicates that the provided period is invalid (e.g., zero).
    InvalidPeriod,
}
