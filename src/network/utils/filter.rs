use regex::Regex;
use thiserror::Error;
use windivert::layer::NetworkLayer;
use windivert::prelude::WinDivertFlags;
use windivert::{CloseAction, WinDivert};

#[derive(Debug, Error, Clone)]
pub enum FilterError {
    #[error("Invalid filter syntax: {0}")]
    InvalidSyntax(String),
    #[error("Invalid port number detected in filter: {0}")]
    InvalidPort(String),
}

pub fn validate_filter(filter: &str) -> Result<String, FilterError> {
    // Attempt to open a handle to validate the filter string syntax
    let mut win_divert = WinDivert::<NetworkLayer>::network(filter, 0, WinDivertFlags::new())
        .map_err(|e| FilterError::InvalidSyntax(e.to_string()))?;

    win_divert
        .close(CloseAction::Nothing)
        .map_err(|_| FilterError::InvalidSyntax("Failed to close handle.".into()))?;

    // Additional check: ensure any provided port numbers are valid
    let port_pattern = Regex::new(r"(tcp|udp)\.(SrcPort|DstPort)\s*==\s*(\d+)(?:$|\s)").unwrap();
    for cap in port_pattern.captures_iter(filter) {
        if let Some(port_str) = cap.get(3) {
            port_str.as_str().parse::<u16>().map_err(|_| {
                FilterError::InvalidPort(format!(
                    "Port number {} is out of range (0-65535)",
                    port_str.as_str()
                ))
            })?;
        }
    }

    Ok(filter.to_string())
}