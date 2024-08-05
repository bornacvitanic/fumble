use crate::network::core::packet_data::PacketData;
use crate::network::types::Probability;

pub fn drop_packets(packets: &mut Vec<PacketData>, drop_probability: Probability) {
    packets.retain(|_| rand::random::<f64>() >= drop_probability.value())
}

#[cfg(test)]
mod tests {
    use crate::network::core::packet_data::PacketData;
    use crate::network::modules::drop::drop_packets;
    use crate::network::types::Probability;
    use windivert::layer::NetworkLayer;
    use windivert::packet::WinDivertPacket;
    use crate::network::core::packet_data::PacketData;

    #[test]
    fn test_drop_packets() {
        unsafe {
            let mut packets = vec![PacketData::from(WinDivertPacket::<NetworkLayer>::new(
                vec![1, 2, 3],
            ))];
            drop_packets(&mut packets, Probability::new(1.0).unwrap());
            assert!(packets.is_empty())
        }
    }
}