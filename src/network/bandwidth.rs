use std::collections::VecDeque;
use std::time::Instant;
use crate::network::capture::PacketData;

const MAX_BUFFER_SIZE: usize = 100_000;

pub fn bandwidth_limiter<'a>(
    packets: &mut Vec<PacketData<'a>>,
    buffer: &mut VecDeque<PacketData<'a>>,
    last_send_time: &mut Instant,
    bandwidth_limit: usize,
) {
    buffer.extend(packets.drain(..));

    if buffer.len() > MAX_BUFFER_SIZE {
        let excess = buffer.len() - MAX_BUFFER_SIZE;
        for _ in 0..excess {
            buffer.pop_front();
        }
    }

    let now = Instant::now();
    let elapsed = now.duration_since(*last_send_time).as_secs_f64();
    let bytes_allowed = ((bandwidth_limit*1024) as f64 * elapsed) as usize;

    let mut bytes_sent = 0;
    let mut to_send = Vec::new();

    while let Some(packet_data) = buffer.front() {
        let packet_size = packet_data.packet.data.len();
        if bytes_sent + packet_size > bytes_allowed {
            break;
        }
        bytes_sent += packet_size;
        to_send.push(buffer.pop_front().unwrap());
    }

    packets.extend(to_send);

    if bytes_sent > 0 {
        *last_send_time = now;
    }
}