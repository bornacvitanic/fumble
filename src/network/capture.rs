use std::time::Instant;
use windivert::layer::NetworkLayer;
use windivert::packet::WinDivertPacket;

pub struct PacketData<'a> {
    pub packet: WinDivertPacket<'a, NetworkLayer>,
    pub arrival_time: Instant,

impl<'a> From<WinDivertPacket<'a, NetworkLayer>> for PacketData<'a>{
    fn from(packet: WinDivertPacket<'a, NetworkLayer>) -> Self {
        PacketData {
            packet,
            arrival_time: Instant::now(),
        }
    }
}
}