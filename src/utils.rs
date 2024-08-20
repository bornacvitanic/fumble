use log::info;

pub fn log_statistics(received: usize, sent: usize) {
    let dropped = received.saturating_sub(sent); // Number of dropped packets
    let dropped_percentage = if received > 0 {
        (dropped as f64 / received as f64) * 100.0
    } else {
        0.0
    };
    info!(
        "Received Packets: {}, Sent Packets: {}, Skipped Packets: {} - {:.2}%",
        received, sent, dropped, dropped_percentage
    );
}