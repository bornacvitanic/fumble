use crate::network::core::packet_data::PacketData;
use log::{error, info};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use windivert::error::WinDivertError;
use windivert::layer::NetworkLayer;
use windivert::WinDivert;
use windivert_sys::WinDivertFlags;

pub fn receive_packets(
    traffic_filter: String,
    packet_sender: mpsc::Sender<PacketData<'_>>,
    running: Arc<AtomicBool>,
) -> Result<(), WinDivertError> {
    let wd = WinDivert::<NetworkLayer>::network(traffic_filter, 0, WinDivertFlags::new()).map_err(
        |e| {
            error!("Failed to initialize WinDivert: {}", e);
            e
        },
    )?;

    let mut buffer = vec![0u8; 1500];

    while running.load(Ordering::SeqCst) {
        match wd.recv(Some(&mut buffer)) {
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
    }

    info!("Shutting down packet receiving thread");
    Ok(())
}

fn should_shutdown(running: &Arc<AtomicBool>) -> bool {
    if !running.load(Ordering::SeqCst) {
        info!("Packet receiving thread exiting due to shutdown signal.");
        return true;
    }
    false
}
