use std::ops::Sub;
use std::time::{Duration, Instant};

pub struct TamperStats {
    pub(crate) data: Vec<u8>,
    pub(crate) tamper_flags: Vec<bool>,
    pub(crate) checksum_valid: bool,
    pub last_update: Instant,
    pub update_interval: Duration,
}

impl TamperStats {
    pub fn new(refresh_interval: Duration) -> Self {
        TamperStats {
            data: vec![],
            tamper_flags: vec![],
            checksum_valid: true,
            last_update: Instant::now().sub(refresh_interval),
            update_interval: refresh_interval,
        }
    }

    pub fn should_update(&mut self) -> bool {
        if self.last_update.elapsed() >= self.update_interval {
            true
        } else {
            false
        }
    }

    pub fn updated(&mut self) {
        self.last_update = Instant::now();
    }
}