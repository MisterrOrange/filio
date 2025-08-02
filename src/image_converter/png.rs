use super::DecodedFile;
use std::fs::File;
use std::io::{Bytes, BufReader};


pub fn decode_png(it: Bytes<BufReader<File>>) {
    for (index, byte) in it.enumerate() {

    }
}

fn verify