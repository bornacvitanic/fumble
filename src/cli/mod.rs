use crate::cli::config::config_options::ConfigOptions;
use crate::cli::settings::packet_manipulation::PacketManipulationSettings;
use crate::network::utils::filter::validate_filter_with_docs;
use clap::Parser;

pub mod config;
pub mod settings;
pub mod tui;
pub mod utils;

#[derive(Parser, Debug)]
#[command(
    name = "fumble",
    about = "A network manipulation tool for simulating various network conditions.",
    long_about = "fumble is a network manipulation tool that can introduce packet drops, delays, throttling, reordering, duplication, and bandwidth limitations.\n\n\
    ## Logging\n\
    The tool uses the `env_logger` crate for logging. By default, informational messages are displayed.\n\n\
    To customize the verbosity of logs, set the `RUST_LOG` environment variable before running `fumble`.\n\n\
    Example: RUST_LOG=debug fumble --filter 'tcp.DstPort == 80'"
)]
#[derive(Default)]
pub struct Cli {
    /// Filter expression for capturing packets
    #[arg(short, long, value_parser = validate_filter_with_docs)]
    pub filter: Option<String>,

    #[command(flatten)]
    pub config: ConfigOptions,

    #[command(flatten)]
    pub packet_manipulation_settings: PacketManipulationSettings,

    #[arg(short, long, default_value_t = false)]
    pub tui: bool,
}