use std::io::{Bytes, BufReader};
use std::fs::File;
use thiserror::Error;

use crate::algorithms::crc;

pub mod png;



pub struct RawImage {
    width: u32,
    height: u32,
    image_data: Vec<Vec<i32>>,
}

enum ImageTypes {
    PNG,
    JPG
}

pub trait FileConverter {
    type Error;
    fn decode(&mut self, it: Bytes<BufReader<File>>) -> Result<(), Self::Error>
    where Self: Sized;
}