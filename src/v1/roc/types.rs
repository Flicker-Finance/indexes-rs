use crate::v1::types::TradingSignal;

pub struct ROCResult {
    pub value: f64,                // Raw ROC value
    pub momentum: f64,             // Normalized momentum (-100 to 100)
    pub acceleration: Option<f64>, // Rate of change of ROC
    pub signal: TradingSignal,
}
