use std::io::{Bytes, BufReader};
use std::fs::File;

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
    fn decode(it: Bytes<BufReader<File>>) -> anyhow::Result<Self>
    where Self: Sized;
}