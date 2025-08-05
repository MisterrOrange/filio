// Good crc resource (worth a read): https://www.sunshine2k.de/articles/coding/crc/understanding_crc.html

use crate::vec_extension::VecExtU8;

/// Returns true if crc matches data
pub fn validate_crc_32(chunk_data: &Vec<u8>, supplied_crc: Vec<u8>) -> bool {
    let mut bits: String = String::new();
    for byte in chunk_data {
        for bit_pos in (0..8).rev() {
            bits.push(get_bit(*byte, bit_pos));
        }
    }
    let correct_crc = calculate_crc_32(chunk_data);
    if correct_crc == supplied_crc.to_u32() {
        true
    }
    else {
        false
    }
}

// Calculates 32-bit CRCs
fn calculate_crc_32(data: &Vec<u8>) -> u32 {
    let mut crc: u32 = 0xFFFFFFFF;
    let polynom = 0xEDB88320;

    for byte in data {
        // Shift byte into MSByte of crc
        crc ^= *byte as u32;

        for _ in 0..8 {
            // Check if MSBit is set
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ polynom;
            }
            else {
                crc = crc >> 1;
            }
        }
    }
    crc ^ 0xFFFFFFFF
}

fn get_bit(number: u8, position: u32) -> char {
     // Some magic bit masking going on here 
    if (number >> position) & 1 == 1 {
        '1'
    }
    else {
        '0'
    }
}