use crate::network::capture::PacketData;
use crate::network::types::Probability;
use log::error;
use std::collections::HashSet;
use windivert_sys::ChecksumFlags;

pub fn tamper_packets(
    packets: &mut [PacketData],
    tamper_probability: Probability,
    tamper_amount: Probability,
    recalculate_checksums: bool,
) {
    for packet_data in packets.iter_mut() {
        if rand::random::<f64>() >= tamper_probability.value() {
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

        if payload_length > 0 {
            let bytes_to_tamper = (payload_length as f64 * tamper_amount.value()).ceil() as usize;
            apply_tampering(&mut data[payload_offset..], bytes_to_tamper);
        }

        if recalculate_checksums {
            if let Err(e) = packet_data
                .packet
                .recalculate_checksums(ChecksumFlags::new())
            {
                error!("Error recalculating checksums: {}", e);
            }
        }
    }
}

fn apply_tampering(data: &mut [u8], bytes_to_tamper: usize) {
    let mut tampered_indices = HashSet::new();
    let mut tampered_count = 0;
    let data_len = data.len();
    let mut rng = rand::thread_rng();

    while tampered_count < bytes_to_tamper && tampered_count < data_len {
        let index = rng.gen_range(0..data.len());
        if tampered_indices.insert(index) {
            tampered_count += 1;
            let tamper_type = rng.gen_range(0..5);
            match tamper_type {
                0 => bit_manipulation(&mut data[index..], 3, true),
                1 => bit_flipping(&mut data[index..], 5),
                2 => value_adjustment(&mut data[index..], 10, 3),
                3 => data_substitution(&mut data[index..], 15, b"newdata"),
                4 => random_data_injection(&mut data[index..], 20, 5),
                _ => (),
            }
        }
    }
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

fn bit_manipulation(data: &mut [u8], bit_index: usize, new_bit: bool) {
    let byte_index = bit_index / 8;
    let bit_position = bit_index % 8;
    if byte_index < data.len() {
        if new_bit {
            data[byte_index] |= 1 << bit_position; // Set the bit
        } else {
            data[byte_index] &= !(1 << bit_position); // Clear the bit
        }
    }
}

fn bit_flipping(data: &mut [u8], bit_index: usize) {
    let byte_index = bit_index / 8;
    let bit_position = bit_index % 8;
    if byte_index < data.len() {
        data[byte_index] ^= 1 << bit_position; // Flip the bit
    }
}

fn value_adjustment(data: &mut [u8], offset: usize, value: i8) {
    if offset < data.len() {
        let adjusted_value = data[offset].wrapping_add(value as u8);
        data[offset] = adjusted_value;
    }
}

fn data_substitution(data: &mut [u8], offset: usize, new_data: &[u8]) {
    let end = std::cmp::min(data.len(), offset + new_data.len());
    if offset < end {
        data[offset..end].copy_from_slice(&new_data[..end - offset]);
    }
}

use rand::Rng;

fn random_data_injection(data: &mut [u8], offset: usize, length: usize) {
    let mut rng = rand::thread_rng();
    for i in offset..std::cmp::min(data.len(), offset + length) {
        data[i] = rng.random();
    }
}