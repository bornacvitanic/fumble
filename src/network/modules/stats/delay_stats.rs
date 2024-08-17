pub struct DelayStats {
    pub(crate) delayed_package_count: usize
}

impl DelayStats {
    pub fn new() -> Self {
        DelayStats {
            delayed_package_count: 0
        }
    }

    pub fn delayed_package_count(&mut self, value: usize) {
        self.delayed_package_count = value;
    }
}