use crate::network::modules::stats::util::ewma::Ewma;

pub struct DuplicateStats {
    pub(crate) incoming_packet_count: usize,
    pub(crate) outgoing_packet_count: usize,
    ewma: Ewma,
}

impl DuplicateStats {
    pub fn new(alpha: f64) -> Self {
        DuplicateStats {
            incoming_packet_count: 0,
            outgoing_packet_count: 0,
            ewma: Ewma::new(alpha),
        }
    }

    pub fn record(&mut self, outgoing_count: usize) {
        self.incoming_packet_count += 1;
        self.outgoing_packet_count += outgoing_count;

        let current_duplication_multiplier = outgoing_count as f64;
        self.ewma.update(current_duplication_multiplier);
    }

    pub fn total_duplication_multiplier(&self) -> f64 {
        if self.outgoing_packet_count == 0 {
            1.0
        } else {
            self.outgoing_packet_count as f64 / self.incoming_packet_count as f64
        }
    }

    pub fn recent_duplication_multiplier(&self) -> f64 {
        self.ewma.get().unwrap_or(1.0)
    }
}
