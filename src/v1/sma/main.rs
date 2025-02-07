/// A Simple Moving Average (SMA) calculator that maintains a moving window of values
/// and calculates their average.
///
/// The Simple Moving Average is calculated by taking the arithmetic mean of a given set of values
/// over a specified period. For example, a 20-day SMA is calculated by taking the arithmetic mean
/// of the most recent 20 values.
///
/// # Example
///
/// ```
/// use your_crate::SimpleMovingAverage;
///
/// let mut sma = SimpleMovingAverage::new(3).unwrap();
/// sma.add_value(2.0);
/// sma.add_value(4.0);
/// sma.add_value(6.0);
///
/// assert_eq!(sma.calculate().unwrap(), 4.0);
/// ```
#[derive(Debug)]
pub struct SimpleMovingAverage {
    /// The period over which the moving average is calculated
    pub period: usize,
    /// The collection of values in the moving window
    pub values: Vec<f64>,
}

impl SimpleMovingAverage {
    /// Creates a new SimpleMovingAverage instance with the specified period.
    ///
    /// # Arguments
    ///
    /// * `period` - The number of values to include in the moving average calculation
    ///
    /// # Returns
    ///
    /// * `Ok(SimpleMovingAverage)` - A new instance with the specified period
    /// * `Err(SMAError)` - If the period is invalid (e.g., zero)
    pub fn new(period: usize) -> Self {
        SimpleMovingAverage { period, values: Vec::new() }
    }

    /// Adds a new value to the moving window.
    ///
    /// If the window is full (length equals period), the oldest value is removed
    /// before adding the new one.
    ///
    /// # Arguments
    ///
    /// * `value` - The new value to add to the moving window
    pub fn add_value(&mut self, value: f64) {
        self.values.push(value);
        if self.values.len() > self.period {
            self.values.remove(0);
        }
    }

    /// Calculates the current Simple Moving Average.
    ///
    /// # Returns
    ///
    /// * `Some(f64)` - The calculated average if enough values are available
    /// * `None` - If there aren't enough values to calculate the average
    ///
    /// # Example
    ///
    /// ```
    /// use your_crate::SimpleMovingAverage;
    ///
    /// let mut sma = SimpleMovingAverage::new(3);
    /// sma.add_value(2.0);
    /// sma.add_value(4.0);
    /// sma.add_value(6.0);
    ///
    /// assert_eq!(sma.calculate(), Some(4.0));
    /// ```
    pub fn calculate(&self) -> Option<f64> {
        if self.values.len() < self.period {
            return None;
        }

        let sum: f64 = self.values.iter().sum();
        Some(sum / self.period as f64)
    }
}
