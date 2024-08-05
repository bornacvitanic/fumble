use crate::network::capture::PacketData;
use crate::network::types::Probability;
use rand::Rng;
use std::vec::Vec;

pub fn duplicate_packets(packets: &mut Vec<PacketData>, count: usize, probability: Probability) {
    let mut rng = rand::thread_rng();
    let mut duplicate_packets = Vec::with_capacity(packets.len() * (count - 1));

    for packet_data in packets.iter() {
        if rng.random::<f64>() < probability.value() {
            for _ in 1..count {
                duplicate_packets.push(PacketData::from(packet_data.packet.clone()));
            }
        }
    }
    packets.extend(duplicate_packets);
}

#[cfg(test)]
mod tests {
    use crate::network::capture::PacketData;
    use crate::network::duplicate::duplicate_packets;
    use crate::network::types::Probability;
    use windivert::layer::NetworkLayer;
    use windivert::packet::WinDivertPacket;

    #[test]
    fn test_packet_duplication() {
        unsafe {
            let original_packets = vec![PacketData::from(WinDivertPacket::<NetworkLayer>::new(
                vec![1, 2, 3],
            ))];
            let original_len = original_packets.len();
            let mut packets = original_packets.clone();

            duplicate_packets(&mut packets, 3, Probability::new(1.0).unwrap());

            // Ensure three times as many packets
            assert_eq!(packets.len(), original_len * 3);

            // Ensure data consistency
            for chunk in packets.chunks(original_len) {
                for packet_data in chunk.iter() {
                    assert_eq!(packet_data.packet.data[..], [1, 2, 3]);
                }
            }
        }
    }
}
