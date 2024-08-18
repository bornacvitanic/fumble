use crate::network::types::probability::Probability;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct ThrottleOptions {
    /// Probability of triggering a throttle event, ranging from 0.0 to 1.0
    #[arg(long = "throttle-probability", id = "throttle-probability", default_value_t = Probability::default())]
    #[serde(default)]
    pub probability: Probability,

    /// Duration in milliseconds for which throttling should be applied
    #[arg(
        long = "throttle-duration",
        default_value_t = 30,
        id = "throttle-duration"
    )]
    #[serde(default)]
    pub duration: u64,

    /// Indicates whether throttled packets should be dropped
    #[arg(long = "throttle-drop", default_value_t = false, id = "throttle-drop")]
    #[serde(default)]
    pub drop: bool,
}

impl Default for ThrottleOptions {
    fn default() -> Self {
        ThrottleOptions {
            probability: Probability::default(),
            duration: 30,
            drop: false,
        }
    }
}