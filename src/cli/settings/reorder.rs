use clap::Parser;
use serde::{Deserialize, Serialize};
use crate::network::types::Probability;

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct ReorderOptions {
    /// Probability of reordering packets, ranging from 0.0 to 1.0
    #[arg(long = "reorder-probability", id = "reorder-probability", default_value_t = Probability::default())]
    #[serde(default)]
    pub probability: Probability,
    /// Maximum random delay in milliseconds to apply when reordering packets
    #[arg(long = "reorder-max-delay", id = "reorder-max-delay", default_value_t = 100)]
    #[serde(default)]
    pub max_delay: u64,
}

impl Default for ReorderOptions {
    fn default() -> Self {
        ReorderOptions {
            probability: Probability::default(),
            max_delay: 100,
        }
    }
}