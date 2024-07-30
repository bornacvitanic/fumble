use crate::network::capture::PacketData;

pub fn drop_packets(packets: &mut Vec<PacketData>, drop_probability: f64){
    packets.retain(|_| rand::random::<f64>() >= drop_probability)
}