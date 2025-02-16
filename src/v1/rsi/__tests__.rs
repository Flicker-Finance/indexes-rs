//! Tests for the RSI calculator.

#[cfg(test)]
mod tests {
    use crate::v1::rsi::{
        main::RSI,
        types::{MarketCondition, RSIResult},
    };

    /// Test that the calculator returns `None` until sufficient data has been provided.
    #[test]
    fn test_insufficient_data() {
        let mut rsi = RSI::new(3, None, None);
        // First price point: no calculation possible.
        assert_eq!(rsi.calculate(100.0), None);
        // Second price point: still insufficient.
        assert_eq!(rsi.calculate(101.0), None);
    }

    /// Test that the RSI calculation eventually produces a result and that the market condition is valid.
    #[test]
    fn test_rsi_calculation() {
        let mut rsi = RSI::new(3, None, None);
        let prices = vec![44.34, 44.09, 44.15, 43.61, 44.33];
        let mut result: Option<RSIResult> = None;
        for price in prices {
            result = rsi.calculate(price);
        }
        // Expect a result since we have processed at least 3 changes.
        assert!(result.is_some());
        let rsi_result = result.unwrap();
        // Check that the market condition is one of the defined variants.
        match rsi_result.condition {
            MarketCondition::Overbought | MarketCondition::Oversold | MarketCondition::Neutral => {}
        }
    }

    /// Test the market condition determination for default thresholds.
    #[test]
    fn test_determine_condition_default() {
        let rsi = RSI::new(14, None, None);
        assert_eq!(rsi.determine_condition(75.0), MarketCondition::Overbought);
        assert_eq!(rsi.determine_condition(25.0), MarketCondition::Oversold);
        assert_eq!(rsi.determine_condition(50.0), MarketCondition::Neutral);
    }

    /// Test the market condition determination for custom thresholds.
    #[test]
    fn test_determine_condition_custom() {
        // Use custom thresholds: 80 for overbought, 20 for oversold.
        let rsi = RSI::new(14, Some(80.0), Some(20.0));
        assert_eq!(rsi.determine_condition(82.0), MarketCondition::Overbought);
        assert_eq!(rsi.determine_condition(18.0), MarketCondition::Oversold);
        assert_eq!(rsi.determine_condition(50.0), MarketCondition::Neutral);
    }
}
