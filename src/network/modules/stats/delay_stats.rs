pub struct DelayStats {
    pub(crate) delayed_package_count: usize,
}

impl Default for DelayStats {
    fn default() -> Self {
        Self::new()
    }
}

impl DelayStats {
    pub fn new() -> Self {
        DelayStats {
            delayed_package_count: 0,
        }
    }

    pub fn delayed_package_count(&mut self, value: usize) {
        self.delayed_package_count = value;
    }
}
