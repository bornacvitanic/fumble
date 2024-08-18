use std::time::{Duration, Instant};
use crate::network::modules::stats::util::ewma::Ewma;

pub struct BandwidthStats {
    pub(crate) storage_packet_count: usize,
    pub(crate) total_byte_count: usize,
    ewma: Ewma,
    recent_byte_sent: usize,
    recent_timer: Instant,
    update_interval: Duration,
}

impl BandwidthStats {
    pub fn new(alpha: f64) -> Self {
        BandwidthStats {
            storage_packet_count: 0,
            total_byte_count: 0,
            ewma: Ewma::new(alpha),
            recent_byte_sent: 0,
            recent_timer: Instant::now(),
            update_interval: Duration::from_millis(100),
        }
    }

    pub fn record(&mut self, bytes_sent: usize) {
        self.total_byte_count += bytes_sent;
        self.recent_byte_sent += bytes_sent;
        if self.recent_timer.elapsed() >= self.update_interval {
            self.ewma.update((self.recent_byte_sent as f64 / 1024f64) / self.update_interval.as_secs_f64());
            self.recent_byte_sent = 0;
            self.recent_timer = Instant::now();
        }
    }

    pub fn recent_throughput(&self) -> f64 {
        self.ewma.get().unwrap_or(0.0)
    }
}