use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct DelayOptions {
    /// Delay in milliseconds to introduce for each packet
    #[arg(long = "delay-duration", id = "delay-duration")]
    #[serde(default)]
    pub duration: Option<u64>,
}

impl Default for DelayOptions {
    fn default() -> Self {
        DelayOptions {
            duration: Some(0),
        }
    }
}