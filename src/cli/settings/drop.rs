use crate::network::types::Probability;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct DropOptions {
    /// Probability of dropping packets, ranging from 0.0 to 1.0
    #[arg(long = "drop-probability", id = "drop-probability")]
    #[serde(default)]
    pub probability: Option<Probability>,
}

impl Default for DropOptions {
    fn default() -> Self {
        DropOptions {
            probability: Some(Probability::new(0.0).unwrap()),
        }
    }
}
