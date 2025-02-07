#[cfg(test)]
mod tests {
    use crate::v1::{macd::main::MACD, types::TradingSignal};

    #[test]
    fn test_macd_signals() {
        let mut macd = MACD::new(12, 26, 9);
        let prices = vec![10.0, 12.0, 14.0, 13.0, 11.0, 9.0];

        for price in prices {
            let result = macd.calculate(price);
            if let Some(res) = result {
                assert!(res.histogram == res.macd_line - res.signal_line);
            }
        }
    }

    #[test]
    fn test_signal_determination() {
        let macd = MACD::new(12, 26, 9);
        assert_eq!(macd.determine_signal(1.0, 0.5), TradingSignal::Buy);
        assert_eq!(macd.determine_signal(0.5, 1.0), TradingSignal::Sell);
        assert_eq!(macd.determine_signal(1.0, 1.0), TradingSignal::Hold);
    }
}
