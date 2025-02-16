#[cfg(test)]
mod tests {
    use crate::v1::bollinger::{main::BollingerBands, types::BBResult};

    #[test]
    fn test_insufficient_data() {
        let mut bb = BollingerBands::new(5, 2.0).unwrap();
        // Feed fewer than 5 prices.
        for price in [100.0, 101.0, 102.0, 103.0] {
            assert!(bb.calculate(price).is_none());
        }
    }

    #[test]
    fn test_bollinger_bands_calculation() {
        let mut bb = BollingerBands::new(3, 2.0).unwrap();
        // Provide exactly 3 prices.
        let prices = vec![100.0, 102.0, 104.0];
        let mut result = None;
        for price in prices {
            result = bb.calculate(price);
        }
        let res: BBResult = result.unwrap();
        // The middle band should be the SMA of [100, 102, 104] which is 102.0.
        // Standard deviation = sqrt(((100-102)² + (102-102)² + (104-102)²)/3)
        //                  = sqrt((4 + 0 + 4) / 3) = sqrt(8/3) ≈ 1.633
        // Band width = 1.633 * 2.0 ≈ 3.266
        // Upper band ≈ 102 + 3.266 = 105.266, Lower band ≈ 102 - 3.266 = 98.734.
        assert!((res.middle - 102.0).abs() < 1e-6);
        assert!((res.upper - 105.266).abs() < 0.01);
        assert!((res.lower - 98.734).abs() < 0.01);
    }
}
