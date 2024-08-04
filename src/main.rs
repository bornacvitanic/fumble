use clap::Parser;
use env_logger::Env;
use fumble::cli::Cli;
use fumble::network::capture::{packet_receiving_thread, start_packet_processing};
use log::{debug, error, info};
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use windivert::error::WinDivertError;

fn main() -> Result<(), WinDivertError> {
    initialize_logging();
    let cli = Cli::parse();
    debug!("Parsed CLI arguments: {:?}", &cli);
    log_initialization_info(&cli);

    let running = Arc::new(AtomicBool::new(true));
    let shutdown_triggered = Arc::new(Mutex::new(false));
    setup_ctrlc_handler(running.clone(), shutdown_triggered.clone());

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

fn setup_ctrlc_handler(running: Arc<AtomicBool>, shutdown_triggered: Arc<Mutex<bool>>) {
    ctrlc::set_handler(move || {
        let mut shutdown_initiated = shutdown_triggered.lock().unwrap();
        if !*shutdown_initiated {
            *shutdown_initiated = true;
            info!("Ctrl+C pressed; initiating shutdown...");
            running.store(false, Ordering::SeqCst);
        } else {
            error!("Ctrl+C pressed again; forcing immediate exit.");
            exit(1); // Exit immediately without waiting for cleanup
        }
    })
    .expect("Error setting Ctrl-C handler");
}

fn initialize_logging() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
}

fn log_initialization_info(cli: &Cli) {
    if let Some(traffic_filter) = &cli.filter {
        info!("Traffic filer: {}", traffic_filter);
    }
    if let Some(drop_probability) = &cli.drop.probability {
        info!("Dropping packets with probability: {}", drop_probability);
    }
    if let Some(delay) = &cli.delay.duration {
        info!("Delaying packets for: {} ms", delay)
    }
    if let Some(throttle_probability) = &cli.throttle.probability {
        info!(
            "Throttling packets with probability of {} ms with a throttle duration of {}. \
        Throttle packet dropping: {}",
            throttle_probability, &cli.throttle.duration, &cli.throttle.drop
        )
    }
    if let Some(max_delay) = &cli.reorder.max_delay {
        info!(
            "Reordering packets with maximum random delay of: {} ms",
            max_delay
        )
    }
    if let Some(tamper_probability) = &cli.tamper.probability {
        info!(
            "Tampering packets with probability {} and amount {}. Recalculating checksums: {}",
            tamper_probability,
            &cli.tamper.amount,
            &cli.tamper.recalculate_checksums.unwrap_or(true)
        )
    }
    if cli.duplicate.count > 1usize && cli.duplicate.probability.unwrap_or_default().value() > 0.0 {
        info!(
            "Duplicating packets {} times with probability: {}",
            &cli.duplicate.count,
            &cli.duplicate.probability.unwrap()
        );
    }
    if let Some(bandwidth_limit) = &cli.bandwidth.limit {
        info!("Limiting bandwidth to: {} KB/s", bandwidth_limit)
    }
}
