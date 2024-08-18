use crate::network::types::Probability;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct DuplicateOptions {
    /// Probability of duplicating packets, ranging from 0.0 to 1.0
    #[arg(long = "duplicate-probability", id = "duplicate-probability", default_value_t = Probability::default())]
    #[serde(default)]
    pub probability: Probability,

    /// Number of times to duplicate each packet
    #[arg(long = "duplicate-count", default_value_t = 1, id = "duplicate-count")]
    #[serde(default)]
    pub count: usize,
}

impl Default for DuplicateOptions {
    fn default() -> Self {
        DuplicateOptions {
            count: 1,
            probability: Probability::default(),
        }
    }
}