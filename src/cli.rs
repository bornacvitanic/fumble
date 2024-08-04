use crate::network::types::Probability;
use clap::Parser;
use windivert::layer::NetworkLayer;
use windivert::prelude::WinDivertFlags;
use windivert::{CloseAction, WinDivert};

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
pub struct Cli {
    /// Filter expression for capturing packets
    #[arg(short, long, value_parser = validate_filter)]
    pub filter: Option<String>,

    #[command(flatten)]
    pub drop: DropOptions,

    #[command(flatten)]
    pub delay: DelayOptions,

    #[command(flatten)]
    pub throttle: ThrottleOptions,

    #[command(flatten)]
    pub reorder: ReorderOptions,

    #[command(flatten)]
    pub duplicate: DuplicateOptions,

    #[command(flatten)]
    pub bandwidth: BandwidthOptions,
}

#[derive(Parser, Debug, Default)]
pub struct DropOptions {
    /// Probability of dropping packets, ranging from 0.0 to 1.0
    #[arg(long = "drop-probability", id = "drop-probability")]
    pub probability: Option<Probability>,
}

#[derive(Parser, Debug, Default)]
pub struct DelayOptions {
    /// Delay in milliseconds to introduce for each packet
    #[arg(long = "delay-duration", id = "delay-duration")]
    pub duration: Option<u64>,
}

#[derive(Parser, Debug, Default)]
pub struct ThrottleOptions {
    /// Probability of triggering a throttle event, ranging from 0.0 to 1.0
    #[arg(long = "throttle-probability", id = "throttle-probability")]
    pub probability: Option<Probability>,

    /// Duration in milliseconds for which throttling should be applied
    #[arg(long = "throttle-duration", default_value_t = 30, id = "throttle-duration")]
    pub duration: u64,

    /// Indicates whether throttled packets should be dropped
    #[arg(long = "throttle-drop", default_value_t = false, id = "throttle-drop")]
    pub drop: bool,
}

#[derive(Parser, Debug, Default)]
pub struct ReorderOptions {
    /// Maximum random delay in milliseconds to apply when reordering packets
    #[arg(long = "reorder-max-delay", id = "reorder-max-delay")]
    pub max_delay: Option<u64>,
}

#[derive(Parser, Debug, Default)]
pub struct DuplicateOptions {
    /// Number of times to duplicate each packet
    #[arg(long = "duplicate-count", default_value_t = 1, id = "duplicate-count")]
    pub count: usize,

    /// Probability of duplicating packets, ranging from 0.0 to 1.0
    #[arg(long = "duplicate-probability", id = "duplicate-probability")]
    pub probability: Option<Probability>,
}

#[derive(Parser, Debug, Default)]
pub struct BandwidthOptions {
    /// Maximum bandwidth limit in KB/s
    #[arg(long = "bandwidth-limit", id = "bandwidth-limit")]
    pub limit: Option<usize>,
}

fn validate_filter(filter: &str) -> Result<String, String> {
    // Attempt to open a handle to validate the filter string syntax
    let handle = WinDivert::<NetworkLayer>::network(filter, 0, WinDivertFlags::new());
    match handle {
        Ok(mut wd) => { wd.close(CloseAction::Nothing).expect("Failed to close filter validation WinDivert handle."); }
        Err(e) => { return Err(e.to_string()); }
    }

    // Additional check: ensure any provided port numbers are valid
    let port_pattern = regex::Regex::new(r"(tcp|udp)\.(SrcPort|DstPort)\s*==\s*(\d+)(?:$|\s)").unwrap();
    for cap in port_pattern.captures_iter(filter) {
        if let Some(port_str) = cap.get(3) {
            if let Err(e) = port_str.as_str().parse::<u16>() {
                println!("Invalid port number detected.");
                return Err(format!("Invalid port number detected. Port number {} is out of range (0-65535). Error: {}", port_str.as_str(), e));
            }
        }
    }

    Ok(filter.to_string())
}