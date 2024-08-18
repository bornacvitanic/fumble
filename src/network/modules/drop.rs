use rand::Rng;
use crate::network::core::packet_data::PacketData;
use crate::network::modules::stats::drop_stats::DropStats;
use crate::network::types::probability::Probability;

pub fn drop_packets(packets: &mut Vec<PacketData>, drop_probability: Probability, stats: &mut DropStats) {
    let mut rng = rand::thread_rng();

    // We use retain with a side effect: recording the drop stats
    packets.retain(|_| {
        let drop = rng.random::<f64>() < drop_probability.value();
        stats.record(drop);
        !drop
    });
}

#[cfg(test)]
mod tests {
    use crate::network::core::packet_data::PacketData;
    use crate::network::modules::drop::drop_packets;
    use crate::network::types::probability::Probability;
    use windivert::layer::NetworkLayer;
    use windivert::packet::WinDivertPacket;
    use crate::network::modules::stats::drop_stats::DropStats;

    #[test]
    fn test_drop_packets() {
        unsafe {
            let mut packets = vec![PacketData::from(WinDivertPacket::<NetworkLayer>::new(
                vec![1, 2, 3],
            ))];
            let mut drop_stats = DropStats::new(0.3);
            drop_packets(&mut packets, Probability::new(1.0).unwrap(), &mut drop_stats);
            assert!(packets.is_empty())
        }
    }
}