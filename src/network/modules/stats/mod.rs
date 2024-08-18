use std::sync::{Arc, RwLock};
use std::time::Duration;
use crate::network::modules::stats::bandwidth_stats::BandwidthStats;
use crate::network::modules::stats::drop_stats::DropStats;
use crate::network::modules::stats::delay_stats::DelayStats;
use crate::network::modules::stats::duplicate_stats::DuplicateStats;
use crate::network::modules::stats::throttle_stats::ThrottleStats;
use crate::network::modules::stats::reorder_stats::ReorderStats;
use crate::network::modules::stats::tamper_stats::TamperStats;

pub mod util;
pub mod drop_stats;
pub mod delay_stats;
pub mod throttle_stats;
pub mod reorder_stats;
pub mod tamper_stats;
pub mod duplicate_stats;
pub mod bandwidth_stats;

pub struct PacketProcessingStatistics {
    pub drop_stats: DropStats,
    pub delay_stats: DelayStats,
    pub throttle_stats: ThrottleStats,
    pub reorder_stats: ReorderStats,
    pub tamper_stats: TamperStats,
    pub duplicate_stats: DuplicateStats,
    pub bandwidth_stats: BandwidthStats,
}

// Function to initialize the statistics
pub fn initialize_statistics() -> Arc<RwLock<PacketProcessingStatistics>> {
    Arc::new(RwLock::new(PacketProcessingStatistics {
        drop_stats: DropStats::new(0.005),
        delay_stats: DelayStats::new(),
        throttle_stats: ThrottleStats::new(),
        reorder_stats: ReorderStats::new(0.005),
        tamper_stats: TamperStats::new(Duration::from_millis(500)),
        duplicate_stats: DuplicateStats::new(0.005),
        bandwidth_stats: BandwidthStats::new(0.005),
    }))
}