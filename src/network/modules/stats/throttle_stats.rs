pub struct ThrottleStats {
    pub(crate) is_throttling: bool,
    pub(crate) dropped_count: usize,
}

impl ThrottleStats {
    pub fn new() -> Self {
        ThrottleStats {
            is_throttling: false,
            dropped_count: 0,
        }
    }
}