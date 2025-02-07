use crate::v1::types::TrendDirection;

#[derive(Debug)]
pub struct SMAResult {
    pub value: f64,
    pub trend: TrendDirection,
}
