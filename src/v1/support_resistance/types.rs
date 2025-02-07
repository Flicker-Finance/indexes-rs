#[derive(Debug, Clone)]
pub enum PricePosition {
    AboveResistance,
    BelowSupport,
    NearResistance,
    NearSupport,
    Middle,
    Unknown,
}

#[derive(Debug)]
pub struct SRResult {
    pub nearest_support: Option<f64>,
    pub nearest_resistance: Option<f64>,
    pub support_strength: f64,
    pub resistance_strength: f64,
    pub breakout_potential: f64,
    pub price_position: PricePosition,
}
