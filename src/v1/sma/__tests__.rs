#[cfg(test)]
mod tests {
    use crate::v1::sma::main::SimpleMovingAverage;

    #[test]
    fn test_sma_calculation() {
        let mut sma = SimpleMovingAverage::new(3);

        sma.add_value(2.0);
        assert_eq!(sma.calculate(), None);

        sma.add_value(4.0);
        assert_eq!(sma.calculate(), None);

        sma.add_value(6.0);
        assert_eq!(sma.calculate(), Some(4.0));

        sma.add_value(8.0);
        assert_eq!(sma.calculate(), Some(6.0));
    }

    #[test]
    fn test_period_overflow() {
        let mut sma = SimpleMovingAverage::new(2);
        sma.add_value(1.0);
        sma.add_value(2.0);
        sma.add_value(3.0);

        assert_eq!(sma.calculate(), Some(2.5));
        assert_eq!(sma.values.len(), 2);
    }
}
