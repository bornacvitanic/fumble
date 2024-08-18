use crate::cli::settings::packet_manipulation::PacketManipulationSettings;
use log::info;

pub fn log_initialization_info(filter: &Option<String>, settings: &PacketManipulationSettings) {
    if let Some(traffic_filter) = &filter {
        info!("Traffic filer: {}", traffic_filter);
    }
    if let Some(drop) = &settings.drop {
        info!("Dropping packets with probability: {}", drop.probability);
    }
    if let Some(delay) = &settings.delay {
        info!("Delaying packets for: {} ms", delay.duration)
    }
    if let Some(throttle) = &settings.throttle {
        info!(
            "Throttling packets with probability of {} ms with a throttle duration of {}. \
        Throttle packet dropping: {}",
            throttle.probability, throttle.duration, throttle.drop
        )
    }
    if let Some(reorder) = &settings.reorder {
        info!(
            "Reordering packets with probability {} and maximum random delay of: {} ms",
            reorder.probability, reorder.max_delay
        )
    }
    if let Some(tamper) = &settings.tamper {
        info!(
            "Tampering packets with probability {} and amount {}. Recalculating checksums: {}",
            tamper.probability,
            tamper.amount,
            tamper.recalculate_checksums.unwrap_or(true)
        )
    }
    if let Some(duplicate) = &settings.duplicate {
        if duplicate.count > 1usize && duplicate.probability.value() > 0.0 {
            info!(
            "Duplicating packets {} times with probability: {}",
            duplicate.count, duplicate.probability
        );
        }
    }
    if let Some(bandwidth) = &settings.bandwidth {
        info!("Limiting bandwidth to: {} KB/s", bandwidth.limit)
    }
}