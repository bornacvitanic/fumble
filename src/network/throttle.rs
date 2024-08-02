use std::collections::VecDeque;
use std::time::{Duration, Instant};
use rand::{Rng, thread_rng};
use crate::network::capture::PacketData;

pub fn throttle_packages<'a>(
    packets: &mut Vec<PacketData<'a>>,
    storage: &mut VecDeque<PacketData<'a>>,
    throttle_probability: f64,
    throttle_duration: Duration,
    drop: bool,
    throttled_start_time: &mut Instant) {
    if is_throttled(throttle_duration, throttled_start_time) {
        if drop {
            packets.clear();
        } else {
            storage.extend(packets.drain(..));
        }
    } else {
        packets.extend(storage.drain(..));
        if thread_rng().gen_bool(throttle_probability)  {
            *throttled_start_time = Instant::now();
        }
    }
}

fn is_throttled(throttle_duration: Duration, throttled_start_time: &mut Instant) -> bool {
    throttled_start_time.elapsed() <= throttle_duration
}