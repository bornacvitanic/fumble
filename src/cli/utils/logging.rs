use log::info;
use crate::cli::settings::packet_manipulation::PacketManipulationSettings;

pub fn log_initialization_info(filter: &Option<String>, settings: &PacketManipulationSettings) {
    if let Some(traffic_filter) = &filter {
        info!("Traffic filer: {}", traffic_filter);
    }
    if let Some(drop_probability) = &settings.drop.probability {
        info!("Dropping packets with probability: {}", drop_probability);
    }
    if let Some(delay) = &settings.delay.duration {
        info!("Delaying packets for: {} ms", delay)
    }
    if let Some(throttle_probability) = &settings.throttle.probability {
        info!(
            "Throttling packets with probability of {} ms with a throttle duration of {}. \
        Throttle packet dropping: {}",
            throttle_probability, &settings.throttle.duration, &settings.throttle.drop
        )
    }
    if let Some(max_delay) = &settings.reorder.max_delay {
        info!(
            "Reordering packets with maximum random delay of: {} ms",
            max_delay
        )
    }
    if let Some(tamper_probability) = &settings.tamper.probability {
        info!(
            "Tampering packets with probability {} and amount {}. Recalculating checksums: {}",
            tamper_probability,
            &settings.tamper.amount,
            &settings.tamper.recalculate_checksums.unwrap_or(true)
        )
    }
    let duplicate_probability = settings.duplicate.probability.unwrap_or_default();
    if settings.duplicate.count > 1usize && duplicate_probability.value() > 0.0 {
        info!(
            "Duplicating packets {} times with probability: {}",
            &settings.duplicate.count, duplicate_probability
        );
    }
    if let Some(bandwidth_limit) = &settings.bandwidth.limit {
        info!("Limiting bandwidth to: {} KB/s", bandwidth_limit)
    }
}