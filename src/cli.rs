use clap::{Parser};

#[derive(Parser)]
#[command(name = "clumsy", about="A network manipulation tool")]
pub struct Cli {
    /// Filter expression for capturing packets
    #[arg(short, long)]
    pub filter: Option<String>,

    /// Probability of dropping packets in the range 0.0 to 1.0
    #[arg(short, long, value_parser = parse_drop_probability)]
    pub drop: Option<f64>,
}

fn parse_drop_probability(s: &str) -> Result<f64, String> {
    let value: f64 = s.parse().map_err(|_| format!("`{}` isn't a valid number", s))?;
    if (0.0..=1.0).contains(&value) {
        Ok(value)
    } else {
        Err(format!("`{}` is not in the range 0.0 to 1.0", value))
    }
}