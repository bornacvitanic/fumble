use clap::Parser;
use serde::{Deserialize, Serialize};
use crate::network::types::Probability;

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct DuplicateOptions {
    /// Probability of duplicating packets, ranging from 0.0 to 1.0
    #[arg(long = "duplicate-probability", id = "duplicate-probability")]
    #[serde(default)]
    pub probability: Option<Probability>,

    /// Number of times to duplicate each packet
    #[arg(long = "duplicate-count", default_value_t = 1, id = "duplicate-count")]
    #[serde(default)]
    pub count: usize,
}

impl Default for DuplicateOptions {
    fn default() -> Self {
        DuplicateOptions {
            count: 1,
            probability: Some(Probability::new(0.0).unwrap()),
        }
    }
}