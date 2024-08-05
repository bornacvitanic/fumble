use crate::cli::Cli;
use crate::network::bandwidth::bandwidth_limiter;
use crate::network::delay::delay_packets;
use crate::network::drop::drop_packets;
use crate::network::duplicate::duplicate_packets;
use crate::network::reorder::{reorder_packets, DelayedPacket};
use crate::network::tamper::tamper_packets;
use crate::network::throttle::throttle_packages;
use crate::utils::log_statistics;
use log::{error, info, trace};
use std::collections::{BinaryHeap, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::time::sleep;
use windivert::error::WinDivertError;
use windivert::layer::NetworkLayer;
use windivert::packet::WinDivertPacket;
use windivert::prelude::WinDivertFlags;
use windivert::WinDivert;

#[derive(Clone)]
pub struct PacketData<'a> {
    pub packet: WinDivertPacket<'a, NetworkLayer>,
    pub arrival_time: Instant,
}

impl<'a> From<WinDivertPacket<'a, NetworkLayer>> for PacketData<'a> {
    fn from(packet: WinDivertPacket<'a, NetworkLayer>) -> Self {
        PacketData {
            packet,
            arrival_time: Instant::now(),
        }
    }
}

pub struct PacketProcessingState<'a> {
    pub delay_storage: VecDeque<PacketData<'a>>,
    pub reorder_storage: BinaryHeap<DelayedPacket<'a>>,
    pub bandwidth_limit_storage: VecDeque<PacketData<'a>>,
    pub bandwidth_storage_total_size: usize,
    pub throttle_storage: VecDeque<PacketData<'a>>,
    pub throttled_start_time: Instant,
    pub last_sent_package_time: Instant,
}

pub async fn packet_receiving_thread(
    traffic_filter: String,
    packet_sender: mpsc::Sender<PacketData<'_>>,
    running: Arc<AtomicBool>,
) -> Result<(), WinDivertError> {
    let wd = WinDivert::<NetworkLayer>::network(&traffic_filter, 0, WinDivertFlags::new())
        .map_err(|e| {
            error!("Failed to initialize WinDivert: {}", e);
            e
        })?;

    let wd = Arc::new(Mutex::new(wd));

    while running.load(Ordering::SeqCst) {
        let wd_clone = Arc::clone(&wd);
        let packet_fut = tokio::task::spawn_blocking(move || {
            let mut buf = vec![0u8; 1500];
            let result = {
                let wd_guard = wd_clone.lock().unwrap();
                wd_guard.recv(Some(&mut buf))
            };
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
            _ = sleep(Duration::from_millis(250)) => {
                trace!("No packets received. Checking shutdown signal after timeout");
                if should_shutdown(&running) {
                    info!("Shutting down packet receiving thread");
                    break;
                }
            }
        }
    }
    Ok(())
}

fn should_shutdown(running: &Arc<AtomicBool>) -> bool {
    if !running.load(Ordering::SeqCst) {
        info!("Packet receiving thread exiting due to shutdown signal.");
        return true;
    }
    false
}

pub fn start_packet_processing(
    cli: Cli,
    mut packet_receiver: Receiver<PacketData>,
    running: Arc<AtomicBool>,
) -> Result<(), WinDivertError> {
    let wd = WinDivert::<NetworkLayer>::network(
        cli.filter.clone().unwrap_or_default(),
        0,
        WinDivertFlags::new(),
    )
    .map_err(|e| {
        error!("Failed to initialize WinDiver: {}", e);
        e
    })?;

    let log_interval = Duration::from_secs(5);
    let mut last_log_time = Instant::now();

    let mut total_packets = 0;
    let mut sent_packets = 0;

    let mut state = PacketProcessingState {
        delay_storage: VecDeque::new(),
        throttle_storage: VecDeque::new(),
        bandwidth_limit_storage: VecDeque::new(),
        bandwidth_storage_total_size: 0,
        reorder_storage: BinaryHeap::new(),
        throttled_start_time: Instant::now(),
        last_sent_package_time: Instant::now(),
    };

    info!("Starting packet interception.");
    while running.load(Ordering::SeqCst) {
        let mut packets = Vec::new();
        // Try to receive packets from the channel
        while let Ok(packet_data) = packet_receiver.try_recv() {
            packets.push(packet_data);
            total_packets += 1;
        }

        process_packets(&cli, &mut packets, &mut state);

        for packet_data in &packets {
            wd.send(&packet_data.packet).map_err(|e| {
                error!("Failed to send packet: {}", e);
                e
            })?;
            sent_packets += 1;
        }

        // Periodically log the statistics
        if last_log_time.elapsed() >= log_interval {
            log_statistics(total_packets, sent_packets);
            last_log_time = Instant::now(); // Reset the timer
        }
    }

    Ok(())
}

fn process_packets<'a>(
    cli: &Cli,
    packets: &mut Vec<PacketData<'a>>,
    state: &mut PacketProcessingState<'a>,
) {
    if let Some(drop_probability) = cli.drop.probability {
        drop_packets(packets, drop_probability);
    }

    if let Some(delay) = cli.delay.duration {
        delay_packets(
            packets,
            &mut state.delay_storage,
            Duration::from_millis(delay),
        );
    }

    if let Some(throttle_probability) = cli.throttle.probability {
        throttle_packages(
            packets,
            &mut state.throttle_storage,
            &mut state.throttled_start_time,
            throttle_probability,
            Duration::from_millis(cli.throttle.duration),
            cli.throttle.drop,
        );
    }

    if let Some(delay) = cli.reorder.max_delay {
        reorder_packets(
            packets,
            &mut state.reorder_storage,
            Duration::from_millis(delay),
        );
    }

    if let Some(tamper_probability) = cli.tamper.probability {
        tamper_packets(
            packets,
            tamper_probability,
            cli.tamper.amount,
            cli.tamper.recalculate_checksums.unwrap_or(true),
        );
    }

    if cli.duplicate.count > 1 && cli.duplicate.probability.unwrap_or_default().value() > 0.0 {
        duplicate_packets(
            packets,
            cli.duplicate.count,
            cli.duplicate.probability.unwrap_or_default(),
        );
    }

    if let Some(bandwidth_limit) = cli.bandwidth.limit {
        bandwidth_limiter(
            packets,
            &mut state.bandwidth_limit_storage,
            &mut state.bandwidth_storage_total_size,
            &mut state.last_sent_package_time,
            bandwidth_limit,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::network::capture::PacketData;
    use windivert::layer::NetworkLayer;
    use windivert::packet::WinDivertPacket;

    #[test]
    fn test_packet_data_creation() {
        unsafe {
            let dummy_packet = WinDivertPacket::<NetworkLayer>::new(vec![1, 2, 3, 4]);
            let packet_data = PacketData::from(dummy_packet);
            // Assert that the packet data is correctly assigned
            assert_eq!(packet_data.packet.data.len(), 4);
            assert_eq!(packet_data.packet.data[..], [1, 2, 3, 4]);

            // Optionally, check if the arrival time is set (not empty, but correctness might need specific methods)
            assert!(packet_data.arrival_time.elapsed().as_secs() < 1);
        }
    }
}
