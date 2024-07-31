use std::time::Duration;
use crate::network::capture::PacketData;

pub fn delay_packets<'a>(packets: &mut Vec<PacketData<'a>>, storage: &mut Vec<PacketData<'a>>, delay: Duration){
    storage.append(packets);

    storage.retain(|packet_data| {
        if packet_data.arrival_time.elapsed() >= delay {
            packets.push(packet_data.clone());
            false
        } else {
            true
        }
    });
}