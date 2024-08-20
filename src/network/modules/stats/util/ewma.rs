/// A structure that computes the Exponentially Weighted Moving Average (EWMA) of a sequence of values.
///
/// EWMA is a type of infinite impulse response filter that applies weighting factors which
/// decrease exponentially. The weighting for each older datum decreases exponentially, never reaching zero.
/// This is useful for smoothing out time series data and giving more weight to recent observations.
///
/// # Fields
///
/// * `alpha` - The smoothing factor, between 0 and 1. A higher value discounts older observations faster.
/// * `current_value` - The current value of the EWMA after processing the latest input.
///                     Initially, this will be `None` until the first value is processed.
///
/// # Example
///
/// ```rust
/// use fumble::network::modules::stats::util::ewma::Ewma;
/// let mut ewma = Ewma::new(0.5);
/// ewma.update(10.0);
/// assert_eq!(ewma.get(), Some(10.0));
/// ewma.update(20.0);
/// assert_eq!(ewma.get(), Some(15.0)); // 0.5 * 10.0 + 0.5 * 20.0 = 15.0
/// ```
pub struct Ewma {
    alpha: f64,
    current_value: Option<f64>,
}

impl Ewma {
    /// Creates a new `Ewma` instance with the specified smoothing factor `alpha`.
    ///
    /// # Parameters
    ///
    /// * `alpha` - A smoothing factor between 0.0 (exclusive) and 1.0 (inclusive).
    ///             Higher values give more weight to recent observations.
    ///
    /// # Panics
    ///
    /// This function will panic if `alpha` is not in the range `(0, 1]`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fumble::network::modules::stats::util::ewma::Ewma;
    /// let ewma = Ewma::new(0.3);
    /// ```
    pub fn new(alpha: f64) -> Self {
        assert!(
            alpha > 0.0 && alpha <= 1.0,
            "Alpha should be between 0 and 1"
        );
        Ewma {
            alpha,
            current_value: None,
        }
    }

    /// Updates the EWMA with a new value and returns the updated EWMA value.
    ///
    /// # Parameters
    ///
    /// * `new_value` - The new data point to be incorporated into the EWMA.
    ///
    /// # Returns
    ///
    /// The updated EWMA value after incorporating the `new_value`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fumble::network::modules::stats::util::ewma::Ewma;
    /// let mut ewma = Ewma::new(0.5);
    /// ewma.update(10.0);
    /// assert_eq!(ewma.get(), Some(10.0));
    /// ewma.update(20.0);
    /// assert_eq!(ewma.get(), Some(15.0)); // 0.5 * 10.0 + 0.5 * 20.0 = 15.0
    /// ```
    pub fn update(&mut self, new_value: f64) -> f64 {
        self.current_value = Some(match self.current_value {
            Some(current) => current * (1.0 - self.alpha) + new_value * self.alpha,
            None => new_value, // If no previous value exists, just set to new_value
        });
        self.current_value.unwrap()
    }

    /// Retrieves the current EWMA value.
    ///
    /// # Returns
    ///
    /// An `Option<f64>` representing the current EWMA value.
    /// This will be `None` if `update` has not yet been called.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fumble::network::modules::stats::util::ewma::Ewma;
    /// let mut ewma = Ewma::new(0.5);
    /// assert_eq!(ewma.get(), None);
    /// ewma.update(10.0);
    /// assert_eq!(ewma.get(), Some(10.0));
    /// ```
    pub fn get(&self) -> Option<f64> {
        self.current_value
    }
}
