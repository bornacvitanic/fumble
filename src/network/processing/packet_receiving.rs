use crate::cli::Cli;
use crate::network::core::packet_data::PacketData;
use log::{debug, error};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use windivert::error::WinDivertError;
use windivert::layer::NetworkLayer;
use windivert::{CloseAction, WinDivert};
use windivert_sys::WinDivertFlags;

pub fn receive_packets(
    packet_sender: mpsc::Sender<PacketData<'_>>,
    running: Arc<AtomicBool>,
    cli: Arc<Mutex<Cli>>,
) -> Result<(), WinDivertError> {
    let mut buffer = vec![0u8; 1500];
    let mut last_filter = String::new();
    let mut wd: Option<WinDivert<NetworkLayer>> = None;
    let mut logged_missing_handle = false;

    while running.load(Ordering::SeqCst) {
        // Check for filter updates
        let current_filter = match cli.lock() {
            Ok(cli) => cli.filter.clone().unwrap_or_default(),
            Err(_) => {
                error!("Failed to lock CLI for reading");
                continue;
            }
        };

        if current_filter != last_filter {
            // Filter changed, close the existing handle if it exists
            if let Some(ref mut wd_handle) = wd {
                if let Err(e) = wd_handle.close(CloseAction::Nothing) {
                    error!("Failed to close existing WinDivert handle: {}", e);
                } else {
                    debug!("Closed existing WinDivert handle");
                }
            }

            // Open a new WinDivert handle with the new filter
            last_filter = current_filter.clone();
            wd = match WinDivert::<NetworkLayer>::network(
                &current_filter,
                1,
                WinDivertFlags::set_recv_only(WinDivertFlags::new()),
            ) {
                Ok(handle) => {
                    debug!(
                        "WinDivert handle re-opened with new filter: {}",
                        current_filter
                    );
                    Some(handle)
                }
                Err(e) => {
                    error!("Failed to initialize WinDivert: {}", e);
                    None
                }
            };
        }

        if let Some(ref wd_handle) = wd {
            logged_missing_handle = false;
            match wd_handle.recv(Some(&mut buffer)) {
                Ok(packet) => {
                    let packet_data = PacketData::from(packet.into_owned());
                    if packet_sender.send(packet_data).is_err() {
                        if should_shutdown(&running) {
                            break;
                        } else {
                            error!("Failed to send packet data to main thread");
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to receive packet: {}", e);
                    if should_shutdown(&running) {
                        break;
                    }
                }
            }
        } else {
            if !logged_missing_handle {
                error!("WinDivert handle is not initialized. Skipping packet reception.");
                logged_missing_handle = true;
            }
        }
    }

    debug!("Shutting down packet receiving thread");
    Ok(())
}

fn should_shutdown(running: &Arc<AtomicBool>) -> bool {
    if !running.load(Ordering::SeqCst) {
        debug!("Packet receiving thread exiting due to shutdown signal.");
        return true;
    }
    false
}