use serde::Serialize;

/// The result of a Momentum calculation.
#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct MomentumResult {
    /// The momentum value (current price minus past price).
    pub value: f64,
    /// The momentum ratio (current price as a percentage of the past price).
    pub ratio: f64,
}
