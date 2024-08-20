use clap::Parser;
use env_logger::Env;
use fumble::cli::config::config_options::ConfigOptions;
use fumble::cli::tui::cli_ext::{CliExt, TuiStateExt};
use fumble::cli::tui::custom_logger::{init_logger, set_logger_console_state};
use fumble::cli::tui::state::TuiState;
use fumble::cli::tui::terminal::TerminalManager;
use fumble::cli::tui::{input, ui};
use fumble::cli::utils::logging::log_initialization_info;
use fumble::cli::Cli;
use fumble::network::modules::stats::{initialize_statistics, PacketProcessingStatistics};
use fumble::network::processing::packet_processing::start_packet_processing;
use fumble::network::processing::packet_receiving::receive_packets;
use log::{debug, error, info};
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;
use std::thread::JoinHandle;
use windivert::error::WinDivertError;

fn main() -> Result<(), WinDivertError> {
    let mut cli = Cli::parse();

    let mut should_start_tui = false;
    if let Some(_tui) = &cli.tui {
        should_start_tui = true;
        init_logger().expect("Failed to init logger.")
    } else {
        initialize_logging();
    }

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
    let shutdown_triggered = Arc::new(AtomicBool::new(false));
    setup_ctrlc_handler(running.clone(), shutdown_triggered.clone());

    let (packet_sender, packet_receiver) = mpsc::channel();

    let cli_thread_safe = Arc::new(Mutex::new(cli));

    // Start the packet receiving thread
    let cli_for_reading = cli_thread_safe.clone();
    let packet_receiver_handle = thread::spawn({
        let running = running.clone();
        move || receive_packets(packet_sender, running, cli_for_reading)
    });

    // Start packet processing thread
    let statistics = initialize_statistics();

    // Clone the Arc for the packet processing thread
    let cli_for_processing = cli_thread_safe.clone();
    let statistics_for_processing = statistics.clone();
    let packet_sender_handle = thread::spawn({
        let running = running.clone();
        move || {
            start_packet_processing(
                cli_for_processing,
                packet_receiver,
                running,
                statistics_for_processing,
            )
        }
    });

    if should_start_tui {
        tui(cli_thread_safe, statistics, running, shutdown_triggered)?;
    }

    wait_for_thread(packet_sender_handle, "Packet sending");
    debug!("Awaiting packet receiving thread termination...");
    wait_for_thread(packet_receiver_handle, "Packet receiving");

    info!("Application shutdown complete.");
    Ok(())
}

fn tui(
    cli: Arc<Mutex<Cli>>,
    statistics: Arc<RwLock<PacketProcessingStatistics>>,
    running: Arc<AtomicBool>,
    shutdown_triggered: Arc<AtomicBool>,
) -> Result<(), WinDivertError> {
    {
        let mut terminal_manager = TerminalManager::new()?;

        let mut tui_state = TuiState::from_cli(&cli);

        while running.load(Ordering::SeqCst) {
            terminal_manager.draw(|f| ui::ui(f, &mut tui_state))?;
            let should_quit = input::handle_input(&mut tui_state)?;
            if should_quit {
                shutdown_triggered.store(true, Ordering::SeqCst);
                break;
            } else if tui_state.processing {
                cli.update_from(&mut tui_state);
                tui_state.update_from(&statistics);
            } else {
                cli.clear_state();
            }
        }
    }
    set_logger_console_state(true);
    running.store(false, Ordering::SeqCst);
    info!("Initiating shutdown...");
    Ok(())
}

fn wait_for_thread(thread_handle: JoinHandle<Result<(), WinDivertError>>, thread_name: &str) {
    match thread_handle.join() {
        Ok(Ok(())) => {
            debug!("{} thread completed successfully.", thread_name);
        }
        Ok(Err(e)) => {
            error!("{} thread encountered an error: {:?}", thread_name, e);
        }
        Err(e) => {
            error!("Failed to join {} thread: {:?}", thread_name, e);
        }
    }
}

fn setup_ctrlc_handler(running: Arc<AtomicBool>, shutdown_triggered: Arc<AtomicBool>) {
    ctrlc::set_handler(move || {
        if !shutdown_triggered.load(Ordering::SeqCst) {
            shutdown_triggered.store(true, Ordering::SeqCst);
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
