pub fn validate_crc(chunk_data: &Vec<u8>, supplied_crc: Vec<u8>) -> bool {
    let mut bits: String = String::new();
    for byte in chunk_data {
        for bit_pos in (0..8).rev() {
            bits.push(get_bit(*byte, bit_pos));
        }
    }
    let correct_crc = calculate_crc(bits);
    println!("{correct_crc}");
    true
}

// Calculates 32-bit CRCs
fn calculate_crc(mut data: String) -> u32 {
    let nulls = "0".repeat(32);
    // Add padding
    data.push_str(&nulls);
    let data_length = data.len();

    let divisor: &str = "100000100110000010001110110110111";

    'outer: loop {
        // Represents index of first occurence of a 1 in data
        let mut start_index: usize = 0;
        for (index, bit) in data.chars().enumerate() {
            if bit == '1' {
                start_index = index;
                break;
            }
            // If index has reached the padding of data the crc is complete
            if data_length - 32 <= index {
                break 'outer;
            }
        }

        for (index, bit) in divisor.chars().enumerate() {
            let current_index = start_index + index;
            let data_bit = data.chars().nth(current_index).expect("index went out of range of crc + padding, this should never happen");

            let replaced_bit = String::from(xor(bit, data_bit));
            data.replace_range(current_index..current_index+1, &replaced_bit);
        }
    }
    let crc = &data[data.len()-32..];
    // Convert crc to u32
    let crc: u32 = u32::from_str_radix(crc, 2).unwrap();
    crc
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

fn xor(char1: char, char2: char) -> char {
    if char1 == '1' && char2 == '1' {
        '0'
    }
    else if char1 == '1' || char2 == '1' {
        '1'
    }
    else {
        '0'
    }
}