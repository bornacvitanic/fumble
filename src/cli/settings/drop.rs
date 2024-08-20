use crate::network::types::probability::Probability;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize, Default)]
pub struct DropOptions {
    /// Probability of dropping packets, ranging from 0.0 to 1.0
    #[arg(long = "drop-probability", id = "drop-probability", default_value_t = Probability::default())]
    #[serde(default)]
    pub probability: Probability,
}
