use std::collections::VecDeque;
use std::time::{Instant};
use log::{trace};
use crate::network::capture::PacketData;

const MAX_BUFFER_SIZE: usize = 10 * 1024 * 1024; // 10 MB in bytes

pub fn bandwidth_limiter<'a>(
    packets: &mut Vec<PacketData<'a>>,
    buffer: &mut VecDeque<PacketData<'a>>,
    total_buffer_size: &mut usize,
    last_send_time: &mut Instant,
    bandwidth_limit_kbps: usize,
) {
    let incoming_packet_count = packets.len();
    add_packets_to_buffer(buffer, packets, total_buffer_size);
    maintain_buffer_size(buffer, total_buffer_size);

    let now = Instant::now();
    let elapsed = now.duration_since(*last_send_time).as_secs_f64();
    let bytes_allowed = (((bandwidth_limit_kbps * 1024) as f64) * elapsed) as usize;

    let mut bytes_sent = 0;
    let mut to_send = Vec::new();

    while let Some(packet_data) = buffer.front() {
        let packet_size = packet_data.packet.data.len();
        if bytes_sent + packet_size > bytes_allowed {
            break;
        }
        bytes_sent += packet_size;
        match remove_packet_from_buffer(buffer, total_buffer_size) {
            Some(packet) => to_send.push(packet),
            None => {},
        }
    }

    packets.extend(to_send);

    trace!("Limit: {}, Bytes Allowed {}, Incoming Packets: {}, Packets Sent: {}, Buffer Element Count: {}, Total Buffer Size: {}, Bytes Sent: {}",
           bandwidth_limit_kbps, bytes_allowed, incoming_packet_count, packets.len(), buffer.len(), total_buffer_size, bytes_sent);

    if bytes_sent > 0 {
        *last_send_time = now;
    }
}

fn add_packet_to_buffer<'a>(buffer: &mut VecDeque<PacketData<'a>>, packet: PacketData<'a>, total_size: &mut usize) {
    *total_size += packet.packet.data.len();
    buffer.push_back(packet);
}

fn add_packets_to_buffer<'a>(buffer: &mut VecDeque<PacketData<'a>>, packets: &mut Vec<PacketData<'a>>, total_size: &mut usize) {
    while let Some(packet) = packets.pop() {
        add_packet_to_buffer(buffer, packet, total_size);
    }
}

fn remove_packet_from_buffer<'a>(buffer: &mut VecDeque<PacketData<'a>>, total_size: &mut usize) -> Option<PacketData<'a>> {
    if let Some(packet) = buffer.pop_front() {
        *total_size -= packet.packet.data.len();
        Some(packet)
    } else {
        None
    }
}

fn maintain_buffer_size(buffer: &mut VecDeque<PacketData<'_>>, total_size: &mut usize) {
    while *total_size > MAX_BUFFER_SIZE {
        if remove_packet_from_buffer(buffer, total_size).is_some() {
            // Packet removed from buffer to maintain size limit
        } else {
            break; // No more packets to remove
        }
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
        let mut total_buffer_size: &mut usize = &mut 0usize;
        let mut last_send_time = Instant::now() - Duration::from_secs(1);
        let bandwidth_limit = 1; // 1 KB/s

        bandwidth_limiter(&mut packets, &mut buffer, total_buffer_size, &mut last_send_time, bandwidth_limit);

        assert!(packets.len() <= 1);
    }

    #[test]
    fn test_exceeding_buffer_size() {
        let mut packets = Vec::new();
        let mut buffer = VecDeque::new();
        let mut total_buffer_size = 0;

        // Fill the buffer with packets to exceed the max total size
        while total_buffer_size < MAX_BUFFER_SIZE + 10_000 {
            let packet = PacketData::from(create_dummy_packet(1000));
            total_buffer_size += packet.packet.data.len();
            buffer.push_back(packet);
        }
        let mut last_send_time = Instant::now();
        let bandwidth_limit = 100; // High enough to not limit the test

        bandwidth_limiter(&mut packets, &mut buffer, &mut total_buffer_size, &mut last_send_time, bandwidth_limit);

        let actual_total_size: usize = buffer.iter().map(|p| p.packet.data.len()).sum();
        assert!(actual_total_size <= MAX_BUFFER_SIZE);
    }

    #[test]
    fn test_no_bandwidth_limiting() {
        let mut packets = vec![
            PacketData::from(create_dummy_packet(1000)),
            PacketData::from(create_dummy_packet(1000)),
        ];
        let mut buffer = VecDeque::new();
        let mut total_buffer_size = 0;
        let mut last_send_time = Instant::now() - Duration::from_secs(1);
        let bandwidth_limit = 10_000; // 10 MB/s

        bandwidth_limiter(&mut packets, &mut buffer, &mut total_buffer_size, &mut last_send_time, bandwidth_limit);

        assert_eq!(packets.len(), 2);
    }

    #[test]
    fn test_zero_bandwidth() {
        let mut packets = vec![
            PacketData::from(create_dummy_packet(1000)),
            PacketData::from(create_dummy_packet(1000)),
        ];
        let mut buffer = VecDeque::new();
        let mut total_buffer_size = 0;
        let mut last_send_time = Instant::now();
        let bandwidth_limit = 0; // 0 KB/s

        bandwidth_limiter(&mut packets, &mut buffer, &mut total_buffer_size, &mut last_send_time, bandwidth_limit);

        assert!(packets.is_empty());
        assert_eq!(buffer.len(), 2);
    }

    #[test]
    fn test_empty_packet_vector() {
        let mut packets = Vec::new();
        let mut buffer = VecDeque::new();
        let mut total_buffer_size = 0;
        let mut last_send_time = Instant::now();
        let bandwidth_limit = 10_000; // 10 MB/s

        bandwidth_limiter(&mut packets, &mut buffer, &mut total_buffer_size, &mut last_send_time, bandwidth_limit);

        // Since the packets vector was empty, buffer should remain empty and nothing should be sent
        assert!(packets.is_empty());
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_add_packet_to_buffer() {
        let mut buffer = VecDeque::new();
        let mut total_size = 0;
        let packet = PacketData::from(create_dummy_packet(1000));

        add_packet_to_buffer(&mut buffer, packet.clone(), &mut total_size);

        assert_eq!(buffer.len(), 1);
        assert_eq!(total_size, 1000);
        assert_eq!(buffer.front().unwrap().packet.data.len(), 1000);
    }

    #[test]
    fn test_add_packets_to_buffer() {
        let mut buffer = VecDeque::new();
        let mut total_size = 0;
        let mut packets = vec![
            PacketData::from(create_dummy_packet(1000)),
            PacketData::from(create_dummy_packet(2000)),
        ];

        add_packets_to_buffer(&mut buffer, &mut packets, &mut total_size);

        assert_eq!(buffer.len(), 2);
        assert_eq!(total_size, 3000);
        assert_eq!(buffer.pop_front().unwrap().packet.data.len(), 2000);
        assert_eq!(buffer.pop_front().unwrap().packet.data.len(), 1000);
    }

    #[test]
    fn test_remove_packet_from_buffer() {
        let mut buffer = VecDeque::new();
        let mut total_size = 0;
        let packet = PacketData::from(create_dummy_packet(1000));
        add_packet_to_buffer(&mut buffer, packet.clone(), &mut total_size);

        let removed_packet = remove_packet_from_buffer(&mut buffer, &mut total_size);

        assert_eq!(removed_packet.unwrap().packet.data.len(), 1000);
        assert_eq!(buffer.len(), 0);
        assert_eq!(total_size, 0);
    }

    #[test]
    fn test_remove_packet_from_empty_buffer() {
        let mut buffer = VecDeque::new();
        let mut total_size = 0;

        let removed_packet = remove_packet_from_buffer(&mut buffer, &mut total_size);

        assert!(removed_packet.is_none());
        assert_eq!(buffer.len(), 0);
        assert_eq!(total_size, 0);
    }
}