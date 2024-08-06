use clap::Parser;
use serde::{Deserialize, Serialize};
use crate::cli::utils::serialization::serialize_option;

#[derive(Parser, Debug, Serialize, Deserialize)]
#[derive(Default)]
pub struct BandwidthOptions {
    /// Maximum bandwidth limit in KB/s
    #[arg(long = "bandwidth-limit", id = "bandwidth-limit")]
    #[serde(default, serialize_with = "serialize_option")]
    pub limit: Option<usize>,
}

