mod cli;

use fumble::cli::Cli;
use clap::Parser;
use env_logger::Env;
use fumble::network::capture::{packet_receiving_thread, start_packet_processing};
use log::{debug, info};
use windivert::error::WinDivertError;
use std::sync::mpsc::{channel};
use std::thread;

fn main() -> Result<(), WinDivertError> {
    initialize_logging();
    let cli = Cli::parse();
    debug!("Parsed CLI arguments: {:?}", &cli);
    log_initialization_info(&cli);

    let (packet_sender, packet_receiver) = channel();
    let traffic_filter_clone = cli.filter.clone().unwrap_or_default();
    thread::spawn(move || packet_receiving_thread(traffic_filter_clone, packet_sender));

    start_packet_processing(cli, packet_receiver)
}

fn initialize_logging() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
}

fn log_initialization_info(cli: &Cli){
    if let Some(traffic_filter) = &cli.filter {
        info!("Traffic filer: {}", traffic_filter);
    }
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
}