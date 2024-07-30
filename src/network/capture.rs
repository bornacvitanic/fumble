use std::time::Instant;
use windivert::layer::NetworkLayer;
use windivert::packet::WinDivertPacket;

pub struct PacketData<'a> {
    pub packet: WinDivertPacket<'a, NetworkLayer>,
    pub arrival_time: Instant,
}