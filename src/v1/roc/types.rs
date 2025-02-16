use serde::Serialize;

use crate::v1::types::TradingSignal;

/// The result of an ROC calculation.
#[derive(Debug, Clone, Serialize)]
pub struct ROCResult {
    /// The calculated ROC value as a percentage.
    pub value: f64,
    /// The normalized momentum (0-100 scale).
    pub momentum: f64,
    /// The acceleration (change in ROC from the previous value).
    pub acceleration: Option<f64>,
    /// A trading signal based on the ROC.
    pub signal: TradingSignal,
}
