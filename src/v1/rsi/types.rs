#[derive(Debug, Clone, PartialEq)]
pub enum MarketCondition {
    Overbought,
    Oversold,
    Neutral,
}

#[derive(Debug)]
pub struct RSIResult {
    pub value: f64,
    pub condition: MarketCondition,
}
