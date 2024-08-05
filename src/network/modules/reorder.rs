use crate::network::core::packet_data::PacketData;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};
use log::error;

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
    fn new(packet: PacketData<'a>, delay: Duration) -> Self {
        DelayedPacket {
            packet,
            delay_until: Instant::now() + delay,
        }
    }
}

pub fn reorder_packets<'a>(
    packets: &mut Vec<PacketData<'a>>,
    storage: &mut BinaryHeap<DelayedPacket<'a>>,
    max_delay: Duration,
) {
    for packet in packets.drain(..) {
        let delay = Duration::from_millis((rand::random::<u128>() % max_delay.as_millis()) as u64);
        let delayed_packet = DelayedPacket::new(packet, delay);
        storage.push(delayed_packet);
    }

    let now = Instant::now();
    while let Some(delayed_packet) = storage.peek() {
        if delayed_packet.delay_until <= now {
            if let Some(delayed_packet) = storage.pop() {
                packets.push(delayed_packet.packet);
            } else {
                error!("Expected a delayed packet, but none was found in storage.");
                break;
            }
        } else {
            break;
        }
    }
}