use crate::cli::settings::packet_manipulation::PacketManipulationSettings;
use clap::Parser;
use dirs::config_dir;
use std::path::PathBuf;
use std::{fs, io};

/// Manage configurations for fumble.
#[derive(Parser, Debug, Default)]
pub struct ConfigOptions {
    /// Command to create a default configuration file with the specified name.
    #[arg(long, help_heading = "Configuration Management")]
    pub create_default: Option<String>,

    /// Command to use an existing configuration file based on specified name.
    #[arg(long, help_heading = "Configuration Management")]
    pub use_config: Option<String>,

    /// Command to list all available configuration files.
    #[arg(long, help_heading = "Configuration Management")]
    pub list_configs: bool,
}

impl ConfigOptions {
    /// Create a default configuration file with all fields commented out.
    pub fn create_default_config(file_name: &str) -> io::Result<()> {
        ensure_config_dir_exists().unwrap();
        PacketManipulationSettings::create_default_config_file(
            get_config_dir().join(ensure_toml_extension(file_name)),
        )
    }

    /// List all configuration files in the specified directory.
    pub fn list_all_configs() -> io::Result<Vec<String>> {
        let mut config_files = Vec::new();
        ensure_config_dir_exists().unwrap();
        for entry in fs::read_dir(get_config_dir())? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(filename) = path.file_name() {
                    if let Some(filename_str) = filename.to_str() {
                        config_files.push(
                            filename_str
                                .strip_suffix(".toml")
                                .unwrap_or(filename_str)
                                .to_string(),
                        );
                    }
                }
            }
        }
        Ok(config_files)
    }

    /// Load an existing configuration file.
    pub fn load_existing_config(file_name: &str) -> io::Result<PacketManipulationSettings> {
        ensure_config_dir_exists().unwrap();
        PacketManipulationSettings::load_from_file(
            get_config_dir().join(ensure_toml_extension(file_name)),
        )
    }
}

pub fn get_config_dir() -> PathBuf {
    let mut config_path = config_dir().unwrap_or_else(|| {
        // Fallback to home directory if config_dir() fails
        let mut home_dir = dirs::home_dir().expect("Could not find home directory");
        home_dir.push(".fumble");
        home_dir
    });

    config_path.push("fumble");
    config_path
}

pub fn ensure_config_dir_exists() -> io::Result<()> {
    let config_path = get_config_dir();
    if !config_path.exists() {
        fs::create_dir_all(&config_path)?;
    }
    Ok(())
}

pub fn ensure_toml_extension(file_name: &str) -> String {
    let mut path = PathBuf::from(file_name);
    if path.extension().map_or(true, |ext| ext != "toml") {
        path.set_extension("toml");
    }
    path.to_string_lossy().to_string()
}
