use clap::Parser;
use env_logger::Env;
use fumble::cli::config::config_options::ConfigOptions;
use fumble::cli::utils::logging::log_initialization_info;
use fumble::cli::Cli;
use fumble::network::processing::packet_processing::start_packet_processing;
use fumble::network::processing::packet_receiving::receive_packets;
use log::{debug, error, info};
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use windivert::error::WinDivertError;

fn main() -> Result<(), WinDivertError> {
    initialize_logging();
    let mut cli = Cli::parse();
    debug!("Parsed CLI arguments: {:?}", &cli);

    if let Some(file_name) = &cli.config.create_default {
        // Create a default config file and exit
        ConfigOptions::create_default_config(file_name)?;
        info!(
            "Default configuration file created with name {:?}",
            file_name
        );
        return Ok(());
    }

    if cli.config.list_configs {
        // List all config files in the current directory
        match ConfigOptions::list_all_configs() {
            Ok(configs) => {
                for config in configs {
                    println!("{}", config);
                }
            }
            Err(e) => error!("Failed to list configs: {}", e),
        }
        return Ok(());
    }

    // Load configuration from file if specified
    if let Some(file_name) = &cli.config.use_config {
        let loaded_settings = ConfigOptions::load_existing_config(file_name)?;
        cli.packet_manipulation_settings = loaded_settings;
        info!("Loaded configuration from {:?}", file_name);
    }

    log_initialization_info(&cli.filter, &cli.packet_manipulation_settings);

    let running = Arc::new(AtomicBool::new(true));
    let shutdown_triggered = Arc::new(Mutex::new(false));
    setup_ctrlc_handler(running.clone(), shutdown_triggered.clone());

    let (packet_sender, packet_receiver) = mpsc::channel();
    let traffic_filter = cli.filter.clone().unwrap_or_default();

    // Start the packet receiving thread
    let packet_receiver_handle = thread::spawn({
        let running = running.clone();
        move || receive_packets(traffic_filter, packet_sender, running)
    });

    // Start packet processing
    start_packet_processing(cli, packet_receiver, running.clone())?;
    info!("Packet processing stopped. Awaiting packet receiving thread termination...");

    wait_for_receiving_thread(packet_receiver_handle);

    info!("Application shutdown complete.");
    Ok(())
}

fn wait_for_receiving_thread(packet_receiver_handle: JoinHandle<Result<(), WinDivertError>>) {
    match packet_receiver_handle.join() {
        Ok(Ok(())) => {
            info!("Packet receiving thread completed successfully.");
        }
        Ok(Err(e)) => {
            error!("Packet receiving thread encountered an error: {:?}", e);
        }
        Err(e) => {
            error!("Failed to join packet receiving thread: {:?}", e);
        }
    }
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
