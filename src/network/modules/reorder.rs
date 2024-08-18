use crate::network::core::packet_data::PacketData;
use log::{error, warn};
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};
use crate::network::modules::stats::reorder_stats::ReorderStats;
use crate::network::types::delayed_packet::DelayedPacket;
use crate::network::types::probability::Probability;

pub fn reorder_packets<'a>(
    packets: &mut Vec<PacketData<'a>>,
    storage: &mut BinaryHeap<DelayedPacket<'a>>,
    reorder_probability: Probability,
    max_delay: Duration,
    stats: &mut ReorderStats,
) {
    if max_delay.as_millis() == 0 {
        warn!("Max delay cannot be zero. Skipping packet reordering.");
        return;
    }

    let mut skipped_packets = Vec::new(); // Temporary storage for packets to be skipped

    for packet in packets.drain(..) {
        if rand::random::<f64>() >= reorder_probability.value() {
            skipped_packets.push(packet); // Store skipped packets
            stats.record(false);
            continue;
        }

        let delay = Duration::from_millis((rand::random::<u128>() % max_delay.as_millis()) as u64);
        let delayed_packet = DelayedPacket::new(packet, delay);
        storage.push(delayed_packet);
        stats.record(true);
    }
    stats.delayed_packets = storage.len();

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