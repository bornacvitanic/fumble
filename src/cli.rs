use clap::{Parser};

#[derive(Parser)]
#[command(name = "clumsy", about="A network manipulation tool")]
pub struct Cli {
    /// Filter expression for capturing packets
    #[arg(short, long)]
    pub filter: Option<String>,

    /// Probability of dropping packets
    #[arg(short, long)]
    pub drop: Option<f64>,
}