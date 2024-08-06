use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct ReorderOptions {
    /// Maximum random delay in milliseconds to apply when reordering packets
    #[arg(long = "reorder-max-delay", id = "reorder-max-delay")]
    #[serde(default)]
    pub max_delay: Option<u64>,
}

impl Default for ReorderOptions {
    fn default() -> Self {
        ReorderOptions {
            max_delay: Some(100),
        }
    }
}