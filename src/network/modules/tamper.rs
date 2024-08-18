use crate::network::core::packet_data::PacketData;
use crate::network::types::probability::Probability;
use log::error;
use std::collections::HashSet;
use rand::Rng;
use windivert_sys::ChecksumFlags;
use crate::network::modules::stats::tamper_stats::TamperStats;

pub fn tamper_packets(
    packets: &mut [PacketData],
    tamper_probability: Probability,
    tamper_amount: Probability,
    recalculate_checksums: bool,
    stats: &mut TamperStats,
) {
    let should_update_stats = stats.should_update();
    for packet_data in packets.iter_mut() {
        let should_skip = rand::random::<f64>() >= tamper_probability.value();

        if should_skip && !should_update_stats {
            continue;
        }

        let data = packet_data.packet.data.to_mut();

        let (ip_header_len, protocol) = match get_ip_version(data) {
            Some((4, data)) => parse_ipv4_header(data),
            Some((6, data)) => parse_ipv6_header(data),
            _ => {
                error!("Unsupported IP version");
                continue;
            }
        };

        let total_header_len = match protocol {
            17 => parse_udp_header(data, ip_header_len), // UDP
            6 => parse_tcp_header(data, ip_header_len),  // TCP
            _ => ip_header_len,                          // Unsupported protocols
        };

        let payload_offset = total_header_len;
        let payload_length = data.len() - payload_offset;

        if should_skip {
            if should_update_stats {
                stats.data = data[payload_offset..].to_owned();
                stats.tamper_flags = vec![false; stats.data.len()];
                stats.checksum_valid = true;
                stats.updated();
            }
            continue;
        }

        if payload_length > 0 {
            let bytes_to_tamper = (payload_length as f64 * tamper_amount.value()).ceil() as usize;
            let tampered_indices = apply_tampering(&mut data[payload_offset..], bytes_to_tamper);

            if should_update_stats {
                let tampered_flags = calculate_tampered_flags(data.len(), &tampered_indices);
                stats.tamper_flags = tampered_flags;
                stats.data = data[payload_offset..].to_owned();
                stats.updated();
            }
        }

        if recalculate_checksums {
            if let Err(e) = packet_data
                .packet
                .recalculate_checksums(ChecksumFlags::new())
            {
                error!("Error recalculating checksums: {}", e);
            }
        }

        if should_update_stats {
            stats.checksum_valid = packet_data.packet.address.ip_checksum()
                && packet_data.packet.address.tcp_checksum()
                && packet_data.packet.address.udp_checksum();
            stats.updated();
        }
    }
}

fn apply_tampering(data: &mut [u8], bytes_to_tamper: usize) -> HashSet<usize> {
    let mut tampered_indices = HashSet::new();
    let mut tampered_count = 0;
    let data_len = data.len();
    let mut rng = rand::thread_rng();

    while tampered_count < bytes_to_tamper && tampered_count < data_len {
        let index = rng.gen_range(0..data.len());
        if tampered_indices.insert(index) {
            tampered_count += 1;
            let tamper_type = rng.gen_range(0..3);
            let modified_indices = match tamper_type {
                0 => bit_manipulation(data, index, rng.gen_range(0..8), true),
                1 => bit_flipping(data, index, rng.gen_range(0..8)),
                2 => value_adjustment(data, index, rng.gen_range(-64..64)),
                _ => vec![],
            };
            tampered_indices.extend(modified_indices);
        }
    }

    tampered_indices
}

fn calculate_tampered_flags(data_len: usize, tampered_indices: &HashSet<usize>) -> Vec<bool> {
    let mut tampered_flags = vec![false; data_len];
    for &index in tampered_indices.iter() {
        if index < data_len {
            tampered_flags[index] = true;
        }
    }
    tampered_flags
}

fn get_ip_version(data: &[u8]) -> Option<(u8, &[u8])> {
    if data.is_empty() {
        return None;
    }
    let version = data[0] >> 4;
    Some((version, data))
}

fn parse_ipv4_header(data: &[u8]) -> (usize, u8) {
    let header_length = ((data[0] & 0x0F) * 4) as usize;
    let protocol = data[9]; // Protocol field
    (header_length, protocol)
}

fn parse_ipv6_header(data: &[u8]) -> (usize, u8) {
    let header_length = 40; // IPv6 header is always 40 bytes
    let next_header = data[6]; // Next header field
    (header_length, next_header)
}

fn parse_udp_header(_data: &[u8], ip_header_len: usize) -> usize {
    let udp_header_len = 8; // UDP header is always 8 bytes
    ip_header_len + udp_header_len
}

fn parse_tcp_header(data: &[u8], ip_header_len: usize) -> usize {
    let tcp_data_offset = (data[ip_header_len + 12] >> 4) * 4;
    ip_header_len + tcp_data_offset as usize
}

fn bit_manipulation(data: &mut [u8], byte_index: usize, bit_position: usize, new_bit: bool) -> Vec<usize> {
    if byte_index < data.len() && bit_position < 8 {
        if new_bit {
            data[byte_index] |= 1 << bit_position; // Set the bit
        } else {
            data[byte_index] &= !(1 << bit_position); // Clear the bit
        }
        vec![byte_index] // Return the modified index
    } else {
        vec![] // No modification
    }
}

fn bit_flipping(data: &mut [u8], byte_index: usize, bit_position: usize) -> Vec<usize> {
    if byte_index < data.len() && bit_position < 8 {
        data[byte_index] ^= 1 << bit_position; // Flip the bit
        vec![byte_index] // Return the modified index
    } else {
        vec![] // No modification
    }
}

fn value_adjustment(data: &mut [u8], offset: usize, value: i8) -> Vec<usize> {
    if offset < data.len() {
        let adjusted_value = data[offset].wrapping_add(value as u8);
        data[offset] = adjusted_value;
        vec![offset] // Return the modified index
    } else {
        vec![] // No modification
    }
}