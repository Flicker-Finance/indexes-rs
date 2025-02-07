#[derive(Debug, Clone, PartialEq)]
pub enum StochSignal {
    Buy,
    Sell,
    Overbought,
    Oversold,
    Neutral,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StochCondition {
    Overbought,
    Oversold,
    Strong,
    Weak,
    Neutral,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StochCrossover {
    Bullish,
    Bearish,
    None,
}

pub struct StochResult {
    pub k_value: f64,
    pub d_value: f64,
    pub signal: StochSignal,
    pub condition: StochCondition,
    pub crossover: StochCrossover,
    pub strength: f64,
}
