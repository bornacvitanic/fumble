mod cli;

use std::collections::{BinaryHeap, VecDeque};
use crate::cli::Cli;
use clap::Parser;
use env_logger::Env;
use fumble::network::capture::PacketData;
use fumble::network::delay::delay_packets;
use fumble::network::drop::drop_packets;
use fumble::network::duplicate::duplicate_packets;
use fumble::utils::log_statistics;
use log::{debug, error, info};
use std::time::{Duration, Instant};
use windivert::error::WinDivertError;
use windivert::layer::NetworkLayer;
use windivert::prelude::WinDivertFlags;
use windivert::WinDivert;
use fumble::network::reorder::{reorder_packets};
use std::sync::mpsc::{channel, Receiver};
use std::thread;

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
    if let Some(delay) = &cli.reorder {
        info!("Reordering packets with maximum random delay of: {} ms", delay)
    }
    if cli.duplicate_count > 1usize && cli.duplicate_probability.unwrap_or(0.0) > 0.0 {
        info!(
            "Duplicating packets {} times with probability: {}",
            &cli.duplicate_count,
            &cli.duplicate_probability.unwrap()
        );
    }

    let wd = WinDivert::<NetworkLayer>::network(&traffic_filter, 0, WinDivertFlags::new()).map_err(
        |e| {
            error!("Failed to initialize WinDiver: {}", e);
            e
        },
    )?;

    let (packet_sender, packet_receiver) = channel();
    let traffic_filter_clone = traffic_filter.clone();
    thread::spawn(move || packet_receiving_thread(traffic_filter_clone, packet_sender));

    let log_interval = Duration::from_secs(5);
    let mut last_log_time = Instant::now();

    let mut total_packets = 0;
    let mut sent_packets = 0;
    let mut delay_storage = VecDeque::new();
    let mut reorder_storage= BinaryHeap::new();
    let mut buffer = vec![0u8; 1500];

    info!("Starting packet interception.");
    loop {
        // Try to receive packets from the channel
        let mut packets = Vec::new();
        while let Ok(packet_data) = packet_receiver.try_recv() {
            packets.push(packet_data);
            total_packets += 1;
        }

        if let Some(drop_probability) = cli.drop {
            drop_packets(&mut packets, drop_probability);
        }

        if let Some(delay) = cli.delay {
            delay_packets(
                &mut packets,
                &mut delay_storage,
                Duration::from_millis(delay),
            );
        }

        if let Some(delay) = cli.reorder {
            reorder_packets(&mut packets, &mut reorder_storage, Duration::from_millis(delay));
        }

        if cli.duplicate_count > 1 && cli.duplicate_probability.unwrap_or(0.0) > 0.0 {
            duplicate_packets(
                &mut packets,
                cli.duplicate_count,
                cli.duplicate_probability.unwrap_or(0.0),
            );
        }

        for packet_data in packets {
            wd.send(&packet_data.packet)?; // Send the packet data
            sent_packets += 1;
        }

        // Periodically log the statistics
        if last_log_time.elapsed() >= log_interval {
            log_statistics(total_packets, sent_packets);
            last_log_time = Instant::now(); // Reset the timer
        }
    }
}

fn packet_receiving_thread(
    traffic_filter: String,
    packet_sender: std::sync::mpsc::Sender<PacketData>,
    ) -> Result<(), WinDivertError> {

    let wd = WinDivert::<NetworkLayer>::network(&traffic_filter, 0, WinDivertFlags::new()).map_err(
        |e| {
            error!("Failed to initialize WinDiver: {}", e);
            e
        },
    )?;

    let mut buffer = vec![0u8; 1500];
    loop {
        match wd.recv(Some(&mut buffer)) {
            Ok(packet) => {
                let packet_data = PacketData::from(packet.into_owned());
                if packet_sender.send(packet_data).is_err() {
                    error!("Failed to send packet data to main thread");
                    break;
                }
            }
            Err(e) => {
                error!("Failed to receive packet: {}", e);
                break;
            }
        }
    }
    Ok(())
}