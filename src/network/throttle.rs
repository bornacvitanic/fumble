use crate::network::capture::PacketData;
use rand::{thread_rng, Rng};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub fn throttle_packages<'a>(
    packets: &mut Vec<PacketData<'a>>,
    storage: &mut VecDeque<PacketData<'a>>,
    throttled_start_time: &mut Instant,
    throttle_probability: f64,
    throttle_duration: Duration,
    drop: bool,
) {
    if is_throttled(throttle_duration, throttled_start_time) {
        if drop {
            packets.clear();
        } else {
            storage.extend(packets.drain(..));
        }
    } else {
        packets.extend(storage.drain(..));
        if thread_rng().gen_bool(throttle_probability) {
            *throttled_start_time = Instant::now();
        }
    }
}

fn is_throttled(throttle_duration: Duration, throttled_start_time: &mut Instant) -> bool {
    throttled_start_time.elapsed() <= throttle_duration
}
