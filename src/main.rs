mod cli;

use crate::cli::Cli;
use windivert::error::WinDivertError;
use windivert::layer::NetworkLayer;
use windivert::prelude::{WinDivertFlags};
use windivert::WinDivert;
use std::time::{Duration, Instant};
use clap::Parser;
use clumsy::network::capture::PacketData;
use clumsy::network::drop::drop_packets;
use clumsy::utils::log_statistics;
use log::info;
use env_logger;

fn main() -> Result<(), WinDivertError> {
    env_logger::init();
    let cli = Cli::parse();

    let traffic_filter = cli.filter.unwrap_or_else(|| String::new());
    info!("Traffic filer: {}", traffic_filter);
    if let Some(drop_probability) = &cli.drop {
        info!("Dropping packets with probability: {}", drop_probability);
    }

    let log_interval = Duration::from_secs(5);
    let mut last_log_time = Instant::now();

    let wd = WinDivert::<NetworkLayer>::network(traffic_filter, 0, WinDivertFlags::new())
        .map_err(|e| {
            eprintln!("Failed to initialize WinDiver: {}", e);
            e
        })?;
    let mut buffer = vec![0u8; 1500];

    let mut total_packets = 0;
    let mut sent_packets = 0;

    info!("Starting packet interception.");
    loop {
        let mut packets = Vec::new();

        if let Ok(packet) = wd.recv(Some(&mut buffer)) {
            total_packets += 1;
            packets.push(PacketData::from(packet));
        }

        if let Some(drop_probability) = &cli.drop{
            drop_packets(&mut packets, *drop_probability);
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
}