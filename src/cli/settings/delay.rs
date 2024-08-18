use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct DelayOptions {
    /// Delay in milliseconds to introduce for each packet
    #[arg(long = "delay-duration", id = "delay-duration", default_value_t = 0)]
    #[serde(default)]
    pub duration: u64,
}

impl Default for DelayOptions {
    fn default() -> Self {
        DelayOptions { duration: 0 }
    }
}