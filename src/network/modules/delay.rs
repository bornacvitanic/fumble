use crate::network::core::packet_data::PacketData;
use std::collections::VecDeque;
use std::time::Duration;

pub fn delay_packets<'a>(
    packets: &mut Vec<PacketData<'a>>,
    storage: &mut VecDeque<PacketData<'a>>,
    delay: Duration,
) {
    storage.extend(packets.drain(..));

    while let Some(packet_data) = storage.pop_front() {
        if packet_data.arrival_time.elapsed() >= delay {
            packets.push(packet_data);
        } else {
            storage.push_front(packet_data);
            break;
        }
    }
}
