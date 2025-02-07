#[cfg(test)]
mod tests {
    use crate::v1::rsi::{main::RSI, types::MarketCondition};

    #[test]
    fn test_rsi_calculation() {
        let mut rsi = RSI::new(14);
        let prices = vec![44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03, 45.61, 46.28, 46.28];

        let mut result = None;
        for price in prices {
            result = rsi.calculate(price);
        }

        if let Some(r) = result {
            assert!(r.value >= 0.0 && r.value <= 100.0);
        }
    }

    #[test]
    fn test_market_conditions() {
        let rsi = RSI::new(2);
        assert_eq!(rsi.determine_condition(75.0), MarketCondition::Overbought);
        assert_eq!(rsi.determine_condition(25.0), MarketCondition::Oversold);
        assert_eq!(rsi.determine_condition(50.0), MarketCondition::Neutral);
    }
}
