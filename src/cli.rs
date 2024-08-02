use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "clumsy",
    about = "A network manipulation tool",
    long_about = "clumsy is a network manipulation tool that can drop, duplicate, and delay packets.\n\n\
    ## Logging\n\
    The tool uses the `env_logger` crate for logging. By default, info messages are shown.\n\n\
    To see more or less detailed logs, set the `RUST_LOG` environment variable before running `clumsy`.\n\n\
    "
)]
pub struct Cli {
    /// Filter expression for capturing packets
    #[arg(short, long)]
    pub filter: Option<String>,

    /// Probability of dropping packets in the range 0.0 to 1.0
    #[arg(long, value_parser = parse_probability)]
    pub drop: Option<f64>,

    /// Delay to introduce for each packet in milliseconds
    #[arg(long)]
    pub delay: Option<u64>,

    /// Probability of triggering a throttle event
    #[arg(long, value_parser = parse_probability)]
    pub throttle_probability: Option<f64>,

    /// Throttle duration in milliseconds
    #[arg(long, default_value_t = 30)]
    pub throttle_duration: u64,

    /// Makes throttled packets be dropped
    #[arg(long, default_value_t = false)]
    pub throttle_drop: bool,

    /// Reorder packets by applying random delay in milliseconds
    #[arg(short, long)]
    pub reorder: Option<u64>,

    /// Number of times to duplicate packets
    #[arg(long, default_value_t = 1)]
    pub duplicate_count: usize,

    /// Probability of duplicating packets, must be between 0.0 and 1.0
    #[arg(long, value_parser = parse_probability)]
    pub duplicate_probability: Option<f64>,

    /// Set bandwidth limit in KB/s
    #[arg(short, long)]
    pub bandwidth_limit: Option<usize>
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