use crate::cli::utils::serialization::serialize_option;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize, Default)]
pub struct BandwidthOptions {
    /// Maximum bandwidth limit in KB/s
    #[arg(long = "bandwidth-limit", id = "bandwidth-limit")]
    #[serde(default, serialize_with = "serialize_option")]
    pub limit: Option<usize>,
}
