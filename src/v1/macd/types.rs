use crate::v1::types::TradingSignal;

#[derive(Debug)]
pub struct MACDResult {
    pub macd_line: f64,
    pub signal_line: f64,
    pub histogram: f64,
    pub signal: TradingSignal,
}
