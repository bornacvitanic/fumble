mod cli;

use fumble::cli::Cli;
use clap::Parser;
use env_logger::Env;
use fumble::network::capture::{packet_receiving_thread, start_packet_processing};
use log::{debug, info};
use windivert::error::WinDivertError;
use std::sync::mpsc::{channel};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() -> Result<(), WinDivertError> {
    initialize_logging();
    let cli = Cli::parse();
    debug!("Parsed CLI arguments: {:?}", &cli);
    log_initialization_info(&cli);

    let running = Arc::new(AtomicBool::new(true));
    setup_ctrlc_handler(running.clone());

    let (packet_sender, packet_receiver) = channel();
    let traffic_filter = cli.filter.clone().unwrap_or_default();

    let handle = thread::spawn({
        let running = running.clone();
        move || packet_receiving_thread(traffic_filter, packet_sender, running)
    });

    start_packet_processing(cli, packet_receiver, running)?;
    info!("Packet processing stopped. Awaiting packet receiving thread termination...");

    handle.join().expect("Thread panicked")?;
    info!("Application shutdown complete.");

    Ok(())
}

fn setup_ctrlc_handler(running: Arc<AtomicBool>) {
    ctrlc::set_handler(move || {
        info!("Ctrl+C pressed; initiating shutdown...");
        running.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
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
    if let Some(throttle_probability) = &cli.throttle_probability {
        info!("Throttling packets with probability of {} ms with a throttle duration of {}. \
        Throttle packet dropping: {}", throttle_probability, &cli.throttle_duration, &cli.throttle_drop)
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
    if let Some(bandwidth_limit) = &cli.bandwidth_limit {
        info!("Limiting bandwidth to: {} KB/s", bandwidth_limit)
    }
}