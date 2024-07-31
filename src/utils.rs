use log::info;

pub fn log_statistics(total: usize, sent: usize) {
    let dropped = total.saturating_sub(sent); // Number of dropped packets
    let dropped_percentage = if total > 0 {
        (dropped as f64 / total as f64) * 100.0
    } else {
        0.0
    };
    info!(
        "Total Packets: {}, Sent Packets: {}, Dropped Packets: {}, Dropped Percentage: {:.2}%",
        total, sent, dropped, dropped_percentage
    );
}
