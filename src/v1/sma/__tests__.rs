//! Tests for the Simple Moving Average (SMA) calculator.

#[cfg(test)]
mod tests {
    use crate::v1::{
        sma::main::{SMAError, SMAResult, SimpleMovingAverage},
        types::TrendDirection,
    };

    /// Test creating an SMA with a valid period.
    #[test]
    fn test_create_sma_valid() {
        let sma = SimpleMovingAverage::new(3);
        assert!(sma.is_ok());
    }

    /// Test creating an SMA with an invalid period (zero).
    #[test]
    fn test_create_sma_invalid() {
        let sma = SimpleMovingAverage::new(0);
        assert_eq!(sma, Err(SMAError::InvalidPeriod));
    }

    /// Test that the SMA returns `None` until enough values are provided.
    #[test]
    fn test_insufficient_values() {
        let mut sma = SimpleMovingAverage::new(3).unwrap();
        sma.add_value(1.0);
        sma.add_value(2.0);
        assert_eq!(sma.calculate(), None);
    }

    /// Test the SMA calculation when enough values are provided and check the trend.
    #[test]
    fn test_sma_calculation_trend() {
        let mut sma = SimpleMovingAverage::new(3).unwrap();
        sma.add_value(2.0);
        sma.add_value(4.0);
        sma.add_value(6.0);
        // First calculation: average is 4.0, trend is Neutral (no previous value)
        let result1 = sma.calculate().unwrap();
        assert_eq!(
            result1,
            SMAResult {
                value: 4.0,
                trend: TrendDirection::Sideways
            }
        );

        // Now add a new value and recalculate.
        sma.add_value(8.0);
        // The window now is [4.0, 6.0, 8.0] → average = 6.0, which is greater than 4.0.
        let result2 = sma.calculate().unwrap();
        assert_eq!(
            result2,
            SMAResult {
                value: 6.0,
                trend: TrendDirection::Up
            }
        );

        // Add a lower value to reverse the trend.
        sma.add_value(2.0);
        // The window becomes [6.0, 8.0, 2.0] → average = 5.33...
        let result3 = sma.calculate().unwrap();
        // Since 5.33 is less than 6.0, the trend should be Down.
        assert_eq!(result3.trend, TrendDirection::Down);
    }

    /// Test the SMA calculation with a larger window.
    #[test]
    fn test_sma_calculation_large() {
        let mut sma = SimpleMovingAverage::new(5).unwrap();
        let values = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        for v in values {
            sma.add_value(v);
        }
        let result = sma.calculate().unwrap();
        let expected_value = (10.0 + 20.0 + 30.0 + 40.0 + 50.0) / 5.0;
        // Since this is the first calculation, trend should be Neutral.
        assert_eq!(
            result,
            SMAResult {
                value: expected_value,
                trend: TrendDirection::Sideways
            }
        );
    }
}
