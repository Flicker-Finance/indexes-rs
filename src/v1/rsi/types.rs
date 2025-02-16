use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum MarketCondition {
    Overbought,
    Oversold,
    Neutral,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct RSIResult {
    pub value: f64,
    pub condition: MarketCondition,
}
