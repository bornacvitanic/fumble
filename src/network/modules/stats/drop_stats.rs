use crate::network::modules::stats::ewma::Ewma;

pub struct DropStats {
    pub total_packets: usize,
    pub total_dropped: usize,
    ewma: Ewma,
}

impl DropStats {
    pub fn new(alpha: f64) -> Self {
        Self {
            total_packets: 0,
            total_dropped: 0,
            ewma: Ewma::new(alpha),
        }
    }

    pub fn record(&mut self, dropped: bool) {
        self.total_packets += 1;
        if dropped {
            self.total_dropped += 1;
        }

        // Update the EWMA with the new drop status (1.0 if dropped, 0.0 if not)
        let current_drop_rate = if dropped { 1.0 } else { 0.0 };
        self.ewma.update(current_drop_rate);
    }

    pub fn total_drop_rate(&self) -> f64 {
        if self.total_packets == 0 {
            0.0
        } else {
            self.total_dropped as f64 / self.total_packets as f64
        }
    }

    pub fn recent_drop_rate(&self) -> f64 {
        self.ewma.get().unwrap_or(0.0)
    }
}