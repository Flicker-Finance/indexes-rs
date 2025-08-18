use serde::Serialize;

/// Represents the essential price points needed for fractal analysis.
///
/// The `PartialEq` trait is added to simplify comparisons in unit tests.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HighLow {
    pub high: f64,
    pub low: f64,
}

/// The final output struct containing calculated support and resistance levels.
///
/// The `Serialize` trait allows this struct to be easily converted to formats like JSON.
#[derive(Serialize, Debug, PartialEq)]
pub struct SupportResistanceLevels {
    /// A vector of identified support levels, sorted in descending order.
    pub support: Vec<f64>,
    /// A vector of identified resistance levels, sorted in ascending order.
    pub resistance: Vec<f64>,
}
