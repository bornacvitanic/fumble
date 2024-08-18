use crate::network::modules::stats::util::ewma::Ewma;

pub struct ReorderStats {
    pub(crate) total_packets: usize,
    pub(crate) reordered_packets: usize,
    pub(crate) delayed_packets: usize,
    ewma: Ewma,
}

impl ReorderStats {
    pub fn new(alpha: f64) -> Self {
        ReorderStats {
            total_packets: 0,
            reordered_packets: 0,
            delayed_packets: 0,
            ewma: Ewma::new(alpha),
        }
    }

    pub fn record(&mut self, reordered: bool) {
        self.total_packets += 1;
        if reordered {
            self.reordered_packets += 1;
        }

        let current_reorder_rate = if reordered { 1.0 } else { 0.0 };
        self.ewma.update(current_reorder_rate);
    }

    pub fn total_reorder_rate(&self) -> f64 {
        if self.total_packets == 0 {
            0.0
        } else {
            self.reordered_packets as f64 / self.total_packets as f64
        }
    }

    pub fn recent_reorder_rate(&self) -> f64 {
        self.ewma.get().unwrap_or(0.0)
    }
}