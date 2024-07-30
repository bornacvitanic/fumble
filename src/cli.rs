use clap::{Parser};

#[derive(Parser)]
#[command(name = "clumsy", about="A network manipulation tool")]
pub struct Cli {
    /// Filter expression for capturing packets
    #[arg(short, long)]
    pub filter: Option<String>,

    /// Probability of dropping packets in the range 0.0 to 1.0
    #[arg(long, value_parser = parse_probability)]
    pub drop: Option<f64>,

    /// Number of times to duplicate packets
    #[arg(long, default_value_t = 1)]
    pub duplicate_count: usize,

    /// Probability of duplicating packets, must be between 0.0 and 1.0
    #[arg(long, value_parser = parse_probability)]
    pub duplicate_probability: Option<f64>,
}

fn parse_probability(s: &str) -> Result<f64, String> {
    let value: f64 = s.parse().map_err(|_| format!("`{}` isn't a valid number", s))?;
    if (0.0..=1.0).contains(&value) {
        Ok(value)
    } else {
        Err(format!("`{}` is not in the range 0.0 to 1.0", value))
    }
}