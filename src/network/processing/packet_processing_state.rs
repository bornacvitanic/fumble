use crate::network::core::packet_data::PacketData;
use crate::network::types::delayed_packet::DelayedPacket;
use std::collections::{BinaryHeap, VecDeque};
use std::time::Instant;

pub struct PacketProcessingState<'a> {
    pub delay_storage: VecDeque<PacketData<'a>>,
    pub reorder_storage: BinaryHeap<DelayedPacket<'a>>,
    pub bandwidth_limit_storage: VecDeque<PacketData<'a>>,
    pub bandwidth_storage_total_size: usize,
    pub throttle_storage: VecDeque<PacketData<'a>>,
    pub throttled_start_time: Instant,
    pub last_sent_package_time: Instant,
}