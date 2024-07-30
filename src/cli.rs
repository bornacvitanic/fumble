use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "clumsy", about="")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    Drop {
        #[arg(short, long)]
        probability: f64
    },
}