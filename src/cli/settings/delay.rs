use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize, Default)]
pub struct DelayOptions {
    /// Delay in milliseconds to introduce for each packet
    #[arg(long = "delay-duration", id = "delay-duration", default_value_t = 0)]
    #[serde(default)]
    pub duration: u64,
}
