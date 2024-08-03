use std::collections::VecDeque;
use std::time::{Instant};
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

#[cfg(test)]
mod tests {
    use super::*;
    use windivert::packet::WinDivertPacket;
    use windivert::layer::NetworkLayer;
    use std::time::Duration;

    /// Safely creates a dummy packet with a specified length.
    /// Assumes the vector created with the specified length is valid for packet creation.
    fn create_dummy_packet<'a>(length: usize) -> WinDivertPacket<'a, NetworkLayer> {
        let data = vec![1; length];
        unsafe {
            WinDivertPacket::<NetworkLayer>::new(data)
        }
    }

    #[test]
    fn test_basic_bandwidth_limiting() {
        let mut packets = vec![
            PacketData::from(create_dummy_packet(1000)),
            PacketData::from(create_dummy_packet(1000)),
        ];
        let mut buffer = VecDeque::new();
        let mut last_send_time = Instant::now() - Duration::from_secs(1);
        let bandwidth_limit = 1; // 1 KB/s

        bandwidth_limiter(&mut packets, &mut buffer, &mut last_send_time, bandwidth_limit);

        assert!(packets.len() <= 1);
    }

    #[test]
    fn test_exceeding_buffer_size() {
        let mut packets = Vec::new();
        let mut buffer = VecDeque::with_capacity(MAX_BUFFER_SIZE + 10);
        for _ in 0..(MAX_BUFFER_SIZE + 10) {
            buffer.push_back(PacketData::from(create_dummy_packet(1000)));
        }
        let mut last_send_time = Instant::now();
        let bandwidth_limit = 100; // High enough to not limit the test

        bandwidth_limiter(&mut packets, &mut buffer, &mut last_send_time, bandwidth_limit);

        assert!(buffer.len() <= MAX_BUFFER_SIZE);
    }

    #[test]
    fn test_no_bandwidth_limiting() {
        let mut packets = vec![
            PacketData::from(create_dummy_packet(1000)),
            PacketData::from(create_dummy_packet(1000)),
        ];
        let mut buffer = VecDeque::new();
        let mut last_send_time = Instant::now() - Duration::from_secs(1);
        let bandwidth_limit = 10_000; // 10 MB/s

        bandwidth_limiter(&mut packets, &mut buffer, &mut last_send_time, bandwidth_limit);

        assert_eq!(packets.len(), 2);
    }

    #[test]
    fn test_zero_bandwidth() {
        let mut packets = vec![
            PacketData::from(create_dummy_packet(1000)),
            PacketData::from(create_dummy_packet(1000)),
        ];
        let mut buffer = VecDeque::new();
        let mut last_send_time = Instant::now();
        let bandwidth_limit = 0; // 0 KB/s

        bandwidth_limiter(&mut packets, &mut buffer, &mut last_send_time, bandwidth_limit);

        assert!(packets.is_empty());
        assert_eq!(buffer.len(), 2);
    }

    #[test]
    fn test_empty_packet_vector() {
        let mut packets = Vec::new();
        let mut buffer = VecDeque::new();
        let mut last_send_time = Instant::now();
        let bandwidth_limit = 10_000; // 10 MB/s

        bandwidth_limiter(&mut packets, &mut buffer, &mut last_send_time, bandwidth_limit);

        // Since the packets vector was empty, buffer should remain empty and nothing should be sent
        assert!(packets.is_empty());
        assert!(buffer.is_empty());
    }
}