use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize, Default)]
pub struct BandwidthOptions {
    /// Maximum bandwidth limit in KB/s
    #[arg(long = "bandwidth-limit", id = "bandwidth-limit", default_value_t = 0)]
    #[serde(default)]
    pub limit: usize,
}
