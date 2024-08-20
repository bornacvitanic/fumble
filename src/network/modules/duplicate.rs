use crate::network::core::packet_data::PacketData;
use crate::network::modules::stats::duplicate_stats::DuplicateStats;
use crate::network::types::probability::Probability;
use rand::Rng;
use std::vec::Vec;

pub fn duplicate_packets(
    packets: &mut Vec<PacketData>,
    count: usize,
    probability: Probability,
    stats: &mut DuplicateStats,
) {
    let mut rng = rand::thread_rng();
    let mut duplicate_packets = Vec::with_capacity(packets.len() * count);

    for packet_data in packets.iter() {
        if rng.random::<f64>() < probability.value() {
            for _ in 1..=count {
                duplicate_packets.push(PacketData::from(packet_data.packet.clone()));
            }
            stats.record(1 + count);
        } else {
            stats.record(1);
        }
    }
    packets.extend(duplicate_packets);
}

#[cfg(test)]
mod tests {
    use crate::network::core::packet_data::PacketData;
    use crate::network::modules::duplicate::duplicate_packets;
    use crate::network::modules::stats::duplicate_stats::DuplicateStats;
    use crate::network::types::probability::Probability;
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
            let mut stats = DuplicateStats::new(0.05);

            duplicate_packets(&mut packets, 3, Probability::new(1.0).unwrap(), &mut stats);

            // Ensure three times as many packets
            assert_eq!(packets.len(), original_len * 4);

            // Ensure data consistency
            for chunk in packets.chunks(original_len) {
                for packet_data in chunk.iter() {
                    assert_eq!(packet_data.packet.data[..], [1, 2, 3]);
                }
            }
        }
    }
}
