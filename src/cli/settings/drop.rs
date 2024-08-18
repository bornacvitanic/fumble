use crate::network::types::Probability;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct DropOptions {
    /// Probability of dropping packets, ranging from 0.0 to 1.0
    #[arg(long = "drop-probability", id = "drop-probability", default_value_t = Probability::default())]
    #[serde(default)]
    pub probability: Probability,
}

impl Default for DropOptions {
    fn default() -> Self {
        DropOptions {
            probability: Probability::default(),
        }
    }
}