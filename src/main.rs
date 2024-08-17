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
use fumble::cli::tui::custom_logger::init_logger;
use fumble::cli::tui::state::AppState;
use fumble::cli::tui::terminal::TerminalManager;
use fumble::cli::tui::{input, ui};
use fumble::cli::tui::traits::IsActive;
use fumble::cli::tui::widgets::custom_widget::CustomWidget;
use fumble::cli::tui::widgets::utils::TextAreaExt;
use fumble::network::types::Probability;

fn main() -> Result<(), WinDivertError> {
    let mut cli = Cli::parse();

    let mut should_start_tui = false;
    if let Some(_tui) = &cli.tui {
        should_start_tui = true;
        init_logger();
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
    let shutdown_triggered = Arc::new(Mutex::new(false));
    setup_ctrlc_handler(running.clone(), shutdown_triggered.clone());

    let (packet_sender, packet_receiver) = mpsc::channel();
    let traffic_filter = cli.filter.clone().unwrap_or_default();

    // Start the packet receiving thread
    let packet_receiver_handle = thread::spawn({
        let running = running.clone();
        move || receive_packets(traffic_filter, packet_sender, running)
    });

    // Start packet processing thread
    let cli_thread_safe = Arc::new(Mutex::new(cli));

    // Clone the Arc for the packet processing thread
    let cli_for_processing = cli_thread_safe.clone();
    let packet_sender_handle = thread::spawn({
        let running = running.clone();
        move || start_packet_processing(cli_for_processing, packet_receiver, running)
    });

    if should_start_tui {
        tui(running, cli_thread_safe)?;
    }

    wait_for_thread(packet_sender_handle, "Packet sending");
    debug!("Awaiting packet receiving thread termination...");
    wait_for_thread(packet_receiver_handle, "Packet receiving");

    info!("Application shutdown complete.");
    Ok(())
}

fn tui(running: Arc<AtomicBool>, cli: Arc<Mutex<Cli>>) -> Result<(), WinDivertError> {
    let mut terminal_manager = TerminalManager::new()?;

    let mut state = AppState::new();
    match cli.lock() {
        Ok(cli) => {
            if let Some(filter) = &cli.filter {
                state.filter_widget.textarea.set_text(filter);
            }
            if let CustomWidget::Drop(ref mut drop_widget) = state.sections[0] {
                if let Some(probability) = cli.packet_manipulation_settings.drop.probability {
                    drop_widget.probability_text_area.set_text(&probability.value().to_string());
                    drop_widget.set_active(true);
                }
            }
            if let CustomWidget::Delay(ref mut delay_widget) = state.sections[1] {
                if let Some(duration) = cli.packet_manipulation_settings.delay.duration {
                    delay_widget.delay_duration.set_text(&duration.to_string());
                    delay_widget.set_active(true);
                }
            }
            if let CustomWidget::Throttle(ref mut throttle_widget) = state.sections[2] {
                if let Some(probability) = cli.packet_manipulation_settings.throttle.probability {
                    throttle_widget.probability_text_area.set_text(&probability.to_string());
                    throttle_widget.set_active(true);
                }
                throttle_widget.throttle_duration.set_text(&cli.packet_manipulation_settings.throttle.duration.to_string());
                throttle_widget.drop = cli.packet_manipulation_settings.throttle.drop;
            }
            if let CustomWidget::Reorder(ref mut reorder_widget) = state.sections[3] {
                if let Some(duration) = cli.packet_manipulation_settings.reorder.max_delay {
                    reorder_widget.delay_duration.set_text(&duration.to_string());
                    reorder_widget.set_active(true);
                }
            }
            if let CustomWidget::Tamper(ref mut tamper_widget) = state.sections[4] {
                if let Some(probability) = cli.packet_manipulation_settings.tamper.probability {
                    tamper_widget.probability_text_area.set_text(&probability.to_string());
                    tamper_widget.set_active(true);
                }
                tamper_widget.tamper_amount.set_text(&cli.packet_manipulation_settings.tamper.amount.to_string());
                if let Some(recalculate_checksums) = cli.packet_manipulation_settings.tamper.recalculate_checksums {
                    tamper_widget.recalculate_checksums = recalculate_checksums;
                }
            }
            if let CustomWidget::Duplicate(ref mut duplicate_widget) = state.sections[5] {
                if let Some(probability) = cli.packet_manipulation_settings.duplicate.probability {
                    duplicate_widget.probability_text_area.set_text(&probability.to_string());
                    duplicate_widget.set_active(true);
                }
                duplicate_widget.duplicate_count.set_text(&cli.packet_manipulation_settings.duplicate.count.to_string());
            }
            if let CustomWidget::Bandwidth(ref mut bandwidth_widget) = state.sections[6] {
                if let Some(duration) = cli.packet_manipulation_settings.bandwidth.limit {
                    bandwidth_widget.limit.set_text(&duration.to_string());
                    bandwidth_widget.set_active(true);
                }
            }
        }
        Err(_) => {
            eprintln!("Failed to lock CLI mutex.");
        }
    }

    while running.load(Ordering::SeqCst) {
        terminal_manager.draw(|f| ui::ui(f, &mut state))?;
        let should_quit = input::handle_input(&mut state)?;
        if should_quit { running.store(false, Ordering::SeqCst); }
        update_cli(&state, &cli)
    }
    Ok(())
}

fn update_cli(state: &AppState, cli: &Arc<Mutex<Cli>>) {
    if let Ok(mut cli) = cli.lock() {
        if let CustomWidget::Drop(ref drop_widget) = state.sections[0] {
            if !drop_widget.is_active() { cli.packet_manipulation_settings.drop.probability = None }
            else if let Some(Ok(parsed_value)) = drop_widget.probability_text_area.lines().get(0).map(|line| line.parse::<f64>()) {
                cli.packet_manipulation_settings.drop.probability = Probability::new(parsed_value).ok();
            }
        }
        if let CustomWidget::Delay(ref delay_widget) = state.sections[1] {
            if !delay_widget.is_active() { cli.packet_manipulation_settings.delay.duration = None }
            else if let Some(Ok(parsed_value)) = delay_widget.delay_duration.lines().get(0).map(|line| line.parse::<u64>()) {
                cli.packet_manipulation_settings.delay.duration = Some(parsed_value);
            }
        }
        if let CustomWidget::Throttle(ref throttle_widget) = state.sections[2] {
            if !throttle_widget.is_active() { cli.packet_manipulation_settings.throttle.probability = None }
            else {
                if let Some(Ok(parsed_value)) = throttle_widget.probability_text_area.lines().get(0).map(|line| line.parse::<f64>()) {
                    cli.packet_manipulation_settings.throttle.probability = Probability::new(parsed_value).ok();
                }
                if let Some(Ok(parsed_value)) = throttle_widget.throttle_duration.lines().get(0).map(|line| line.parse::<u64>()) {
                    cli.packet_manipulation_settings.throttle.duration = parsed_value;
                }
                cli.packet_manipulation_settings.throttle.drop = throttle_widget.drop;
            }
        }
        if let CustomWidget::Reorder(ref reorder_widget) = state.sections[3] {
            if !reorder_widget.is_active() { cli.packet_manipulation_settings.reorder.max_delay = None }
            else if let Some(Ok(parsed_value)) = reorder_widget.delay_duration.lines().get(0).map(|line| line.parse::<u64>()) {
                cli.packet_manipulation_settings.reorder.max_delay = Some(parsed_value);
            }
        }
        if let CustomWidget::Tamper(ref tamper_widget) = state.sections[4] {
            if !tamper_widget.is_active() { cli.packet_manipulation_settings.tamper.probability = None }
            else {
                if let Some(Ok(parsed_value)) = tamper_widget.probability_text_area.lines().get(0).map(|line| line.parse::<f64>()) {
                    cli.packet_manipulation_settings.tamper.probability = Probability::new(parsed_value).ok();
                }
                if let Some(Ok(parsed_value)) = tamper_widget.tamper_amount.lines().get(0).map(|line| line.parse::<f64>()) {
                    cli.packet_manipulation_settings.tamper.amount = Probability::new(parsed_value).unwrap();
                }
                cli.packet_manipulation_settings.tamper.recalculate_checksums = Some(tamper_widget.recalculate_checksums);
            }
        }
        if let CustomWidget::Duplicate(ref duplicate_widget) = state.sections[5] {
            if !duplicate_widget.is_active() { cli.packet_manipulation_settings.duplicate.probability = None }
            else {
                if let Some(Ok(parsed_value)) = duplicate_widget.probability_text_area.lines().get(0).map(|line| line.parse::<f64>()) {
                    cli.packet_manipulation_settings.duplicate.probability = Probability::new(parsed_value).ok();
                }
                if let Some(Ok(parsed_value)) = duplicate_widget.duplicate_count.lines().get(0).map(|line| line.parse::<usize>()) {
                    cli.packet_manipulation_settings.duplicate.count = parsed_value;
                }
            }
        }
        if let CustomWidget::Bandwidth(ref bandwidth_widget) = state.sections[6] {
            if !bandwidth_widget.is_active() { cli.packet_manipulation_settings.bandwidth.limit = None }
            else if let Some(Ok(parsed_value)) = bandwidth_widget.limit.lines().get(0).map(|line| line.parse::<usize>()) {
                cli.packet_manipulation_settings.bandwidth.limit = Some(parsed_value);
            }
        }
    } else {
        // Handle the case where the mutex lock failed
        eprintln!("Failed to lock CLI mutex.");
    }
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