use crate::network::core::packet_data::PacketData;
use log::{error, warn};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};
use crate::network::types::Probability;

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
    reorder_probability: Probability,
    max_delay: Duration,
) {
    if max_delay.as_millis() == 0 {
        warn!("Max delay cannot be zero. Skipping packet reordering.");
        return;
    }

    let mut skipped_packets = Vec::new(); // Temporary storage for packets to be skipped

    for packet in packets.drain(..) {
        if rand::random::<f64>() >= reorder_probability.value() {
            skipped_packets.push(packet); // Store skipped packets
            continue;
        }

        let delay = Duration::from_millis((rand::random::<u128>() % max_delay.as_millis()) as u64);
        let delayed_packet = DelayedPacket::new(packet, delay);
        storage.push(delayed_packet);
    }

    packets.append(&mut skipped_packets); // Append skipped packets back to the original packets vector

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