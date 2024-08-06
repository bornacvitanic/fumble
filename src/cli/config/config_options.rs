use crate::cli::settings::packet_manipulation::PacketManipulationSettings;
use clap::Parser;
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Manage configurations for fumble.
#[derive(Parser, Debug, Default)]
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
