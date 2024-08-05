use crate::network::types::Probability;
use crate::network::utils::filter::validate_filter;
use clap::Parser;
use log::info;

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
    pub tamper: TamperOptions,

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
    #[arg(
        long = "throttle-duration",
        default_value_t = 30,
        id = "throttle-duration"
    )]
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
pub struct TamperOptions {
    /// Probability of tampering packets, ranging from 0.0 to 1.0
    #[arg(long = "tamper-probability", id = "tamper-probability")]
    pub probability: Option<Probability>,

    /// Amount of tampering that should be applied, ranging from 0.0 to 1.0
    #[arg(long = "tamper-amount", default_value_t = Probability::new(0.1).unwrap(), id = "tamper-amount")]
    pub amount: Probability,

    /// Whether tampered packets should have their checksums recalculated to mask the tampering and avoid the packets getting automatically dropped
    #[arg(
        long = "tamper-recalculate-checksums",
        id = "tamper-recalculate-checksums"
    )]
    pub recalculate_checksums: Option<bool>,
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

pub fn log_initialization_info(cli: &Cli) {
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
    let duplicate_probability = cli.duplicate.probability.unwrap_or_default();
    if cli.duplicate.count > 1usize && duplicate_probability.value() > 0.0 {
        info!(
            "Duplicating packets {} times with probability: {}",
            &cli.duplicate.count,
            duplicate_probability
        );
    }
    if let Some(bandwidth_limit) = &cli.bandwidth.limit {
        info!("Limiting bandwidth to: {} KB/s", bandwidth_limit)
    }
}