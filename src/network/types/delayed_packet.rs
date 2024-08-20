use crate::network::core::packet_data::PacketData;
use std::cmp::Ordering;
use std::time::{Duration, Instant};

pub struct DelayedPacket<'a> {
    pub packet: PacketData<'a>,
    pub delay_until: Instant,
}

impl<'a> PartialEq for DelayedPacket<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.delay_until == other.delay_until
    }
}

impl<'a> Eq for DelayedPacket<'a> {}

impl<'a> PartialOrd for DelayedPacket<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Note: We flip the ordering here to turn BinaryHeap into a min-heap based on delay_until
        Some(other.delay_until.cmp(&self.delay_until))
    }
}

impl<'a> Ord for DelayedPacket<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Note: We flip the ordering here to turn BinaryHeap into a min-heap based on delay_until
        other.delay_until.cmp(&self.delay_until)
    }
}

impl<'a> DelayedPacket<'a> {
    pub(crate) fn new(packet: PacketData<'a>, delay: Duration) -> Self {
        DelayedPacket {
            packet,
            delay_until: Instant::now() + delay,
        }
    }
}
