use crate::cli::settings::packet_manipulation::PacketManipulationSettings;
use crate::cli::Cli;
use crate::network::core::packet_data::PacketData;
use crate::network::modules::bandwidth::bandwidth_limiter;
use crate::network::modules::delay::delay_packets;
use crate::network::modules::drop::drop_packets;
use crate::network::modules::duplicate::duplicate_packets;
use crate::network::modules::reorder::reorder_packets;
use crate::network::modules::stats::PacketProcessingStatistics;
use crate::network::modules::tamper::tamper_packets;
use crate::network::modules::throttle::throttle_packages;
use crate::network::processing::packet_processing_state::PacketProcessingState;
use crate::utils::log_statistics;
use log::{error, info};
use std::collections::{BinaryHeap, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use windivert::error::WinDivertError;
use windivert::layer::NetworkLayer;
use windivert::WinDivert;
use windivert_sys::WinDivertFlags;

pub fn start_packet_processing(
    cli: Arc<Mutex<Cli>>,
    packet_receiver: Receiver<PacketData>,
    running: Arc<AtomicBool>,
    statistics: Arc<RwLock<PacketProcessingStatistics>>,
) -> Result<(), WinDivertError> {
    let wd = WinDivert::<NetworkLayer>::network(
        "false",
        0,
        WinDivertFlags::set_send_only(WinDivertFlags::new()),
    )
    .map_err(|e| {
        error!("Failed to initialize WinDiver: {}", e);
        e
    })?;

    let log_interval = Duration::from_secs(2);
    let mut last_log_time = Instant::now();

    let mut received_packet_count = 0;
    let mut sent_packet_count = 0;

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
            received_packet_count += 1;
        }

        if let Ok(cli) = cli.lock() {
            process_packets(
                &cli.packet_manipulation_settings,
                &mut packets,
                &mut state,
                &statistics,
            );
        }

        for packet_data in &packets {
            wd.send(&packet_data.packet).map_err(|e| {
                error!("Failed to send packet: {}", e);
                e
            })?;
            sent_packet_count += 1;
        }

        // Periodically log the statistics
        if last_log_time.elapsed() >= log_interval {
            log_statistics(received_packet_count, sent_packet_count);
            received_packet_count = 0;
            sent_packet_count = 0;
            last_log_time = Instant::now(); // Reset the timer
        }
    }

    Ok(())
}

pub fn process_packets<'a>(
    settings: &PacketManipulationSettings,
    packets: &mut Vec<PacketData<'a>>,
    state: &mut PacketProcessingState<'a>,
    statistics: &Arc<RwLock<PacketProcessingStatistics>>,
) {
    if let Some(drop) = &settings.drop {
        drop_packets(
            packets,
            drop.probability,
            &mut statistics.write().unwrap().drop_stats,
        );
    }

    if let Some(delay) = &settings.delay {
        delay_packets(
            packets,
            &mut state.delay_storage,
            Duration::from_millis(delay.duration),
            &mut statistics.write().unwrap().delay_stats,
        );
    }

    if let Some(throttle) = &settings.throttle {
        throttle_packages(
            packets,
            &mut state.throttle_storage,
            &mut state.throttled_start_time,
            throttle.probability,
            Duration::from_millis(throttle.duration),
            throttle.drop,
            &mut statistics.write().unwrap().throttle_stats,
        );
    }

    if let Some(reorder) = &settings.reorder {
        reorder_packets(
            packets,
            &mut state.reorder_storage,
            reorder.probability,
            Duration::from_millis(reorder.max_delay),
            &mut statistics.write().unwrap().reorder_stats,
        );
    }

    if let Some(tamper) = &settings.tamper {
        tamper_packets(
            packets,
            tamper.probability,
            tamper.amount,
            tamper.recalculate_checksums.unwrap_or(true),
            &mut statistics.write().unwrap().tamper_stats,
        );
    }

    if let Some(duplicate) = &settings.duplicate {
        if duplicate.count > 1 && duplicate.probability.value() > 0.0 {
            duplicate_packets(
                packets,
                duplicate.count,
                duplicate.probability,
                &mut statistics.write().unwrap().duplicate_stats,
            );
        }
    }

    if let Some(bandwidth) = &settings.bandwidth {
        bandwidth_limiter(
            packets,
            &mut state.bandwidth_limit_storage,
            &mut state.bandwidth_storage_total_size,
            &mut state.last_sent_package_time,
            bandwidth.limit,
            &mut statistics.write().unwrap().bandwidth_stats,
        );
    }
}
