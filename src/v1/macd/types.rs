use serde::Serialize;

use crate::v1::types::TradingSignal;

#[derive(Debug, Clone, Serialize)]
pub struct MACDResult {
    pub macd_line: f64,
    pub signal_line: f64,
    pub histogram: f64,
    pub signal: TradingSignal,
}
