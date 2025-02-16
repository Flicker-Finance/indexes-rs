use serde::Serialize;

/// The result of a Bollinger Bands calculation.
#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct BBResult {
    /// The upper Bollinger Band.
    pub upper: f64,
    /// The middle Bollinger Band (SMA).
    pub middle: f64,
    /// The lower Bollinger Band.
    pub lower: f64,
}
