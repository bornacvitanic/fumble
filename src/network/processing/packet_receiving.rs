use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use log::{error, info, trace};
use tokio::sync::mpsc;
use tokio::time::sleep;
use windivert::error::WinDivertError;
use windivert::WinDivert;
use windivert::layer::NetworkLayer;
use windivert_sys::WinDivertFlags;
use crate::network::core::packet_data::PacketData;

pub async fn receive_packets(
    traffic_filter: String,
    packet_sender: mpsc::Sender<PacketData<'_>>,
    running: Arc<AtomicBool>,
) -> Result<(), WinDivertError> {
    let wd = Arc::new(WinDivert::<NetworkLayer>::network(&traffic_filter, 0, WinDivertFlags::new())
        .map_err(|e| {
            error!("Failed to initialize WinDivert: {}", e);
            e
        })?);

    while running.load(Ordering::SeqCst) {
        let wd = Arc::clone(&wd);
        let packet_fut = tokio::task::spawn_blocking(move || {
            let mut buf = vec![0u8; 1500];
            let result = wd.recv(Some(&mut buf));
            result.map(|packet| packet.into_owned()).ok()
        });

        tokio::select! {
            packet_result = packet_fut => {
                if let Ok(Some(packet)) = packet_result {
                    let packet_data = PacketData::from(packet);
                    if packet_sender.send(packet_data).await.is_err() {
                        if !running.load(Ordering::SeqCst) {
                            error!("Failed to send packet data to main thread");
                        }
                        break;
                    }
                } else {
                    error!("Failed to receive or process packet.");
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