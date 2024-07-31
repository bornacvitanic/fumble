use crate::network::capture::PacketData;

pub fn drop_packets(packets: &mut Vec<PacketData>, drop_probability: f64) {
    packets.retain(|_| rand::random::<f64>() >= drop_probability)
}

#[cfg(test)]
mod tests {
    use crate::network::capture::PacketData;
    use crate::network::drop::drop_packets;
    use windivert::layer::NetworkLayer;
    use windivert::packet::WinDivertPacket;

    #[test]
    fn test_drop_packets() {
        unsafe {
            let mut packets = vec![PacketData::from(WinDivertPacket::<NetworkLayer>::new(
                vec![1, 2, 3],
            ))];
            drop_packets(&mut packets, 1.0);
            assert!(packets.is_empty())
        }
    }
}
