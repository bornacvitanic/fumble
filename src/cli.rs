use clap::Parser;

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
    #[arg(short, long)]
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
    #[arg(long = "drop-probability", value_parser = parse_probability, id = "drop-probability")]
    pub probability: Option<f64>,
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
    #[arg(long = "throttle-probability", value_parser = parse_probability, id = "throttle-probability")]
    pub probability: Option<f64>,

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
    #[arg(long = "duplicate-probability", value_parser = parse_probability, id = "duplicate-probability")]
    pub probability: Option<f64>,
}

#[derive(Parser, Debug, Default)]
pub struct BandwidthOptions {
    /// Maximum bandwidth limit in KB/s
    #[arg(long = "bandwidth-limit", id = "bandwidth-limit")]
    pub limit: Option<usize>,
}

fn parse_probability(s: &str) -> Result<f64, String> {
    let value: f64 = s
        .parse()
        .map_err(|_| format!("`{}` isn't a valid number", s))?;
    if (0.0..=1.0).contains(&value) {
        Ok(value)
    } else {
        Err(format!("`{}` is not in the range 0.0 to 1.0", value))
    }
}