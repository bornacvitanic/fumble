mod cli;

use crate::cli::Cli;
use windivert::error::WinDivertError;
use windivert::layer::NetworkLayer;
use windivert::prelude::{WinDivertFlags};
use windivert::WinDivert;
use std::time::{Duration, Instant};
use clap::Parser;
use env_logger::Env;
use fumble::network::capture::PacketData;
use fumble::network::delay::delay_packets;
use fumble::network::drop::drop_packets;
use fumble::network::duplicate::duplicate_packets;
use fumble::utils::log_statistics;
use log::{debug, error, info};

fn main() -> Result<(), WinDivertError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let cli = Cli::parse();
    debug!("Parsed CLI arguments: {:?}", cli);

    let traffic_filter = cli.filter.unwrap_or_else(|| String::new());
    info!("Traffic filer: {}", traffic_filter);
    if let Some(drop_probability) = &cli.drop {
        info!("Dropping packets with probability: {}", drop_probability);
    }
    if let Some(delay) = &cli.delay {
        info!("Delaying packets for: {} ms", delay)
    }
    if cli.duplicate_count > 1usize && cli.duplicate_probability.unwrap_or(0.0) > 0.0 {
        info!("Duplicating packets {} times with probability: {}", &cli.duplicate_count, &cli.duplicate_probability.unwrap());
    }

    let log_interval = Duration::from_secs(5);
    let mut last_log_time = Instant::now();

    let wd = WinDivert::<NetworkLayer>::network(traffic_filter, 0, WinDivertFlags::new())
        .map_err(|e| {
            error!("Failed to initialize WinDiver: {}", e);
            e
        })?;

    let mut total_packets = 0;
    let mut sent_packets = 0;
    let mut delay_storage = Vec::new();
    let mut buffer = vec![0u8; 1500];

    info!("Starting packet interception.");
    loop {
        if let Ok(packet) = wd.recv(Some(&mut buffer)) {
            total_packets += 1;

            let packet_data = PacketData::from(packet.into_owned());
            let mut packets = vec![packet_data];

            if let Some(drop_probability) = cli.drop {
                drop_packets(&mut packets, drop_probability);
            }

            if let Some(delay) = cli.delay {
                delay_packets(&mut packets, &mut delay_storage, Duration::from_millis(delay));
            }

            if cli.duplicate_count > 1 && cli.duplicate_probability.unwrap_or(0.0) > 0.0 {
                duplicate_packets(&mut packets, cli.duplicate_count, cli.duplicate_probability.unwrap_or(0.0));
            }

            for packet_data in packets {
                wd.send(&packet_data.packet)?; // Send the packet data
                sent_packets += 1;
            }
        }

        // Periodically log the statistics
        if last_log_time.elapsed() >= log_interval {
            log_statistics(total_packets, sent_packets);
            last_log_time = Instant::now(); // Reset the timer
        }
    }
}