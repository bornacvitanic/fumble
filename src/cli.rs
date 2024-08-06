use std::{fs, io};
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::network::types::Probability;
use crate::network::utils::filter::validate_filter;
use clap::Parser;
use log::{info};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Parser, Debug)]
#[command(
    name = "fumble",
    about = "A network manipulation tool for simulating various network conditions.",
    long_about = "fumble is a network manipulation tool that can introduce packet drops, delays, throttling, reordering, duplication, and bandwidth limitations.\n\n\
    ## Logging\n\
    The tool uses the `env_logger` crate for logging. By default, informational messages are displayed.\n\n\
    To customize the verbosity of logs, set the `RUST_LOG` environment variable before running `fumble`.\n\n\
    Example: RUST_LOG=debug fumble --filter 'tcp.DstPort == 80'"
)]
pub struct Cli {
    /// Filter expression for capturing packets
    #[arg(short, long, value_parser = validate_filter)]
    pub filter: Option<String>,

    #[command(flatten)]
    pub config: ConfigOptions,

    #[command(flatten)]
    pub packet_manipulation_settings: PacketManipulationSettings,
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct PacketManipulationSettings {
    #[command(flatten)]
    pub drop: DropOptions,

    #[command(flatten)]
    pub delay: DelayOptions,

    #[command(flatten)]
    pub throttle: ThrottleOptions,

    #[command(flatten)]
    pub reorder: ReorderOptions,

    #[command(flatten)]
    pub tamper: TamperOptions,

    #[command(flatten)]
    pub duplicate: DuplicateOptions,

    #[command(flatten)]
    pub bandwidth: BandwidthOptions,
}

impl Default for PacketManipulationSettings {
    fn default() -> Self {
        PacketManipulationSettings {
            drop: Default::default(),
            delay: Default::default(),
            throttle: Default::default(),
            reorder: Default::default(),
            tamper: Default::default(),
            duplicate: Default::default(),
            bandwidth: Default::default(),
        }
    }
}

impl PacketManipulationSettings {
    /// Load configuration from a TOML file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(config)
    }

    /// Save current configuration to a TOML file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    /// Create a default configuration file with all fields set to default values
    /// but commented out
    pub fn create_default_config_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
        let default_cli = Self::default();

        // Serialize the default configuration to TOML
        let serialized = toml::to_string_pretty(&default_cli)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // Comment out all lines
        let commented_out = serialized
            .lines()
            .map(|line| {
                if line.trim().is_empty() || line.starts_with("[") {
                    line.to_string()
                } else {
                    format!("# {}", line)
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        let mut file = fs::File::create(path)?;
        file.write_all(commented_out.as_bytes())?;
        Ok(())
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            config: Default::default(),
            filter: None,
            packet_manipulation_settings: Default::default()
        }
    }
}

/// Manage configurations for fumble.
#[derive(Parser, Debug)]
#[derive(Default)]
pub struct ConfigOptions {
    /// Command to create a default configuration file.
    #[arg(long, help_heading = "Configuration Management")]
    pub create_default: Option<PathBuf>,

    /// Command to use an existing configuration file.
    #[arg(long, help_heading = "Configuration Management")]
    pub use_config: Option<PathBuf>,

    /// Command to list all available configuration files.
    #[arg(long, help_heading = "Configuration Management")]
    pub list_configs: bool,
}

impl ConfigOptions {
    /// Create a default configuration file with all fields commented out.
    pub fn create_default_config<P: AsRef<Path>>(path: P) -> io::Result<()> {
        PacketManipulationSettings::create_default_config_file(path)
    }

    /// List all configuration files in the specified directory.
    pub fn list_all_configs<P: AsRef<Path>>(directory: P) -> io::Result<Vec<String>> {
        let mut config_files = Vec::new();
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(filename) = path.file_name() {
                    if let Some(filename_str) = filename.to_str() {
                        config_files.push(filename_str.to_string());
                    }
                }
            }
        }
        Ok(config_files)
    }

    /// Load an existing configuration file.
    pub fn load_existing_config<P: AsRef<Path>>(path: P) -> io::Result<PacketManipulationSettings> {
        PacketManipulationSettings::load_from_file(path)
    }
}

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

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct DelayOptions {
    /// Delay in milliseconds to introduce for each packet
    #[arg(long = "delay-duration", id = "delay-duration")]
    #[serde(default)]
    pub duration: Option<u64>,
}

impl Default for DelayOptions {
    fn default() -> Self {
        DelayOptions {
            duration: Some(0),
        }
    }
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct ThrottleOptions {
    /// Probability of triggering a throttle event, ranging from 0.0 to 1.0
    #[arg(long = "throttle-probability", id = "throttle-probability")]
    #[serde(default)]
    pub probability: Option<Probability>,

    /// Duration in milliseconds for which throttling should be applied
    #[arg(
        long = "throttle-duration",
        default_value_t = 30,
        id = "throttle-duration",
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
            probability: Some(Probability::new(0.0).unwrap()),
            duration: 30,
            drop: false,
        }
    }
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct ReorderOptions {
    /// Maximum random delay in milliseconds to apply when reordering packets
    #[arg(long = "reorder-max-delay", id = "reorder-max-delay")]
    #[serde(default)]
    pub max_delay: Option<u64>,
}

impl Default for ReorderOptions {
    fn default() -> Self {
        ReorderOptions {
            max_delay: Some(100),
        }
    }
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct TamperOptions {
    /// Probability of tampering packets, ranging from 0.0 to 1.0
    #[arg(long = "tamper-probability", id = "tamper-probability")]
    #[serde(default)]
    pub probability: Option<Probability>,

    /// Amount of tampering that should be applied, ranging from 0.0 to 1.0
    #[arg(long = "tamper-amount", default_value_t = Probability::new(0.1).unwrap(), id = "tamper-amount")]
    #[serde(default)]
    pub amount: Probability,

    /// Whether tampered packets should have their checksums recalculated to mask the tampering and avoid the packets getting automatically dropped
    #[arg(
        long = "tamper-recalculate-checksums",
        id = "tamper-recalculate-checksums"
    )]
    #[serde(default)]
    pub recalculate_checksums: Option<bool>,
}

impl Default for TamperOptions {
    fn default() -> Self {
        TamperOptions {
            probability: Some(Probability::new(0.0).unwrap()),
            amount: Probability::new(0.1).unwrap(),
            recalculate_checksums: Some(true),
        }
    }
}

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

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct BandwidthOptions {
    /// Maximum bandwidth limit in KB/s
    #[arg(long = "bandwidth-limit", id = "bandwidth-limit")]
    #[serde(default, serialize_with = "serialize_option")]
    pub limit: Option<usize>,
}

impl Default for BandwidthOptions {
    fn default() -> Self {
        BandwidthOptions {
            limit: None,
        }
    }
}

pub fn log_initialization_info(filter: &Option<String>, settings: &PacketManipulationSettings) {
    if let Some(traffic_filter) = &filter {
        info!("Traffic filer: {}", traffic_filter);
    }
    if let Some(drop_probability) = &settings.drop.probability {
        info!("Dropping packets with probability: {}", drop_probability);
    }
    if let Some(delay) = &settings.delay.duration {
        info!("Delaying packets for: {} ms", delay)
    }
    if let Some(throttle_probability) = &settings.throttle.probability {
        info!(
            "Throttling packets with probability of {} ms with a throttle duration of {}. \
        Throttle packet dropping: {}",
            throttle_probability, &settings.throttle.duration, &settings.throttle.drop
        )
    }
    if let Some(max_delay) = &settings.reorder.max_delay {
        info!(
            "Reordering packets with maximum random delay of: {} ms",
            max_delay
        )
    }
    if let Some(tamper_probability) = &settings.tamper.probability {
        info!(
            "Tampering packets with probability {} and amount {}. Recalculating checksums: {}",
            tamper_probability,
            &settings.tamper.amount,
            &settings.tamper.recalculate_checksums.unwrap_or(true)
        )
    }
    let duplicate_probability = settings.duplicate.probability.unwrap_or_default();
    if settings.duplicate.count > 1usize && duplicate_probability.value() > 0.0 {
        info!(
            "Duplicating packets {} times with probability: {}",
            &settings.duplicate.count, duplicate_probability
        );
    }
    if let Some(bandwidth_limit) = &settings.bandwidth.limit {
        info!("Limiting bandwidth to: {} KB/s", bandwidth_limit)
    }
}

fn serialize_option<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize + Default,
{
    match value {
        Some(v) => serializer.serialize_some(v),
        None => serializer.serialize_some(&T::default()),
    }
}