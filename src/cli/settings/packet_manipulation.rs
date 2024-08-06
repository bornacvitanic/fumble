use std::path::Path;
use std::{fs, io};
use std::io::Write;
use clap::Parser;
use serde::{Deserialize, Serialize};
use crate::cli::settings::bandwidth::BandwidthOptions;
use crate::cli::settings::delay::DelayOptions;
use crate::cli::settings::drop::DropOptions;
use crate::cli::settings::duplicate::DuplicateOptions;
use crate::cli::settings::reorder::ReorderOptions;
use crate::cli::settings::tamper::TamperOptions;
use crate::cli::settings::throttle::ThrottleOptions;

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