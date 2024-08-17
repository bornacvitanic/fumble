use std::sync::{Arc, RwLock};
use crate::network::modules::stats::drop_stats::DropStats;

pub mod ewma;
pub mod drop_stats;

pub struct PacketProcessingStatistics {
    pub drop_stats: DropStats
}

// Function to initialize the statistics
pub fn initialize_statistics() -> Arc<RwLock<PacketProcessingStatistics>> {
    Arc::new(RwLock::new(PacketProcessingStatistics {
        drop_stats: DropStats::new(0.005),
    }))
}