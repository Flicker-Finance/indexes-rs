use serde::Serialize;

use super::{
    bollinger::types::BBResult, ma::main::MovingAverageResults, momentum::types::MomentumResult, roc::types::ROCResult, rsi::types::RSIResult, stochastic::types::StochResult,
    support_resistance::types::SRResult,
};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum TradingSignal {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum TrendDirection {
    Up,
    Down,
    Sideways,
}

pub struct BasicIndexes {
    pub ma: MovingAverageResults,
    pub rsi: RSIResult,
    pub bb: BBResult,
    pub atr: f64,
    pub roc: ROCResult,
    pub momentum: MomentumResult,
    pub stochastic: StochResult,
    pub support_resistance: SRResult,
}
