mod cli;

use windivert::error::WinDivertError;
use windivert::layer::NetworkLayer;
use windivert::prelude::{WinDivertFlags, WinDivertPacket};
use windivert::WinDivert;
use std::time::{Duration, Instant};
use clap::Parser;
use crate::cli::{Cli, Commands};

pub struct PacketData<'a> {
    pub packet: WinDivertPacket<'a, NetworkLayer>,
    pub arrival_time: Instant,
}

fn main() -> Result<(), WinDivertError> {
    let cli = Cli::parse();
    let log_interval = Duration::from_secs(5);
    let mut last_log_time = Instant::now();

    let wd = WinDivert::<NetworkLayer>::network("inbound", 0, WinDivertFlags::new())?;
    let mut buffer = vec![0u8; 1500];

    let mut total_packets = 0;
    let mut sent_packets = 0;

    match &cli.command {
        Some(Commands::Drop { probability}) => {
            println!("Dropping packets with probability: {}", probability);
        }
        None => {}
    }

    loop {
        let mut packets = Vec::new();

        if let Ok(packet) = wd.recv(Some(&mut buffer)) {
            total_packets += 1;
            packets.push(PacketData {
                packet,
                arrival_time: Instant::now()
            })
        }

        match &cli.command {
            Some(Commands::Drop { probability}) => {
                drop_packets(&mut packets, *probability);
            }
            None => {}
        }

        for packet_data in packets {
            wd.send(&packet_data.packet)?;
            sent_packets += 1;
        }

        // Periodically log the statistics
        if last_log_time.elapsed() >= log_interval {
            log_statistics(total_packets, sent_packets);
            last_log_time = Instant::now(); // Reset the timer
        }
    }

    pub fn drop_packets(packets: &mut Vec<PacketData>, drop_probability: f64){
        packets.retain(|_| rand::random::<f64>() >= drop_probability)
    }

    fn log_statistics(total: usize, sent: usize) {
        let dropped = total.saturating_sub(sent); // Number of dropped packets
        let dropped_percentage = if total > 0 {
            (dropped as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        println!(
            "Total Packets: {}, Sent Packets: {}, Dropped Packets: {}, Dropped Percentage: {:.2}%",
            total, sent, dropped, dropped_percentage
        );
    }
}