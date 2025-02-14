#[cfg(test)]
mod tests {
    use crate::v1::{roc::main::ROC, types::TradingSignal};

    #[test]
    fn test_insufficient_data() {
        let mut roc = ROC::new(12);
        // Feed fewer than period + 1 prices.
        for price in [100.0, 101.0, 102.0, 103.0] {
            assert!(roc.calculate(price).is_none());
        }
    }

    #[test]
    fn test_roc_calculation() {
        let mut roc = ROC::new(3);
        // Provide 4 prices so that we have enough data.
        let prices = vec![100.0, 102.0, 104.0, 106.0];
        let mut result = None;
        for price in prices {
            result = roc.calculate(price);
        }
        // With these values, old_price should be 100.0 and current price 106.0:
        // ROC = ((106 - 100) / 100) * 100 = 6%
        let res = result.unwrap();
        assert!((res.value - 6.0).abs() < 1e-6);
        // Check that the normalized momentum is approximately 60 (6/10 * 100)
        assert!((res.momentum - 60.0).abs() < 1e-6);
    }

    #[test]
    fn test_trading_signal() {
        // Test Buy signal: feed a series of prices that yield a positive ROC.
        let mut roc = ROC::new(3);
        let prices = vec![100.0, 100.0, 100.0, 103.0];
        let mut result = None;
        for price in prices {
            result = roc.calculate(price);
        }
        let res = result.unwrap();
        assert_eq!(res.signal, TradingSignal::Buy);

        // Test Sell signal: feed a series of prices that yield a negative ROC.
        let mut roc = ROC::new(3);
        let prices = vec![100.0, 100.0, 100.0, 97.0];
        let mut result = None;
        for price in prices {
            result = roc.calculate(price);
        }
        let res = result.unwrap();
        assert_eq!(res.signal, TradingSignal::Sell);
    }
}
