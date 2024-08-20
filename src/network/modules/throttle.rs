use crate::network::core::packet_data::PacketData;
use crate::network::modules::stats::throttle_stats::ThrottleStats;
use crate::network::types::probability::Probability;
use rand::{thread_rng, Rng};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub fn throttle_packages<'a>(
    packets: &mut Vec<PacketData<'a>>,
    storage: &mut VecDeque<PacketData<'a>>,
    throttled_start_time: &mut Instant,
    throttle_probability: Probability,
    throttle_duration: Duration,
    drop: bool,
    stats: &mut ThrottleStats,
) {
    if is_throttled(throttle_duration, throttled_start_time) {
        if drop {
            stats.dropped_count += packets.len();
            packets.clear();
        } else {
            storage.extend(packets.drain(..));
        }
        stats.is_throttling = true;
    } else {
        packets.extend(storage.drain(..));
        if thread_rng().gen_bool(throttle_probability.value()) {
            *throttled_start_time = Instant::now();
        }
        stats.is_throttling = false;
    }
}

fn is_throttled(throttle_duration: Duration, throttled_start_time: &mut Instant) -> bool {
    throttled_start_time.elapsed() <= throttle_duration
}
