use std::sync::{Arc, RwLock};
use crate::network::modules::stats::drop_stats::DropStats;
use crate::network::modules::stats::delay_stats::DelayStats;
use crate::network::modules::stats::throttle_stats::ThrottleStats;
use crate::network::modules::stats::reorder_stats::ReorderStats;

pub mod util;
pub mod drop_stats;
pub mod delay_stats;
pub mod throttle_stats;
pub mod reorder_stats;

pub struct PacketProcessingStatistics {
    pub drop_stats: DropStats,
    pub delay_stats: DelayStats,
    pub throttle_stats: ThrottleStats,
    pub reorder_stats: ReorderStats,
}

// Function to initialize the statistics
pub fn initialize_statistics() -> Arc<RwLock<PacketProcessingStatistics>> {
    Arc::new(RwLock::new(PacketProcessingStatistics {
        drop_stats: DropStats::new(0.005),
        delay_stats: DelayStats::new(),
        throttle_stats: ThrottleStats::new(),
        reorder_stats: ReorderStats::new(0.005),
    }))
}