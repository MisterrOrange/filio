use std::fs::File;
use std::io::{Bytes, BufReader, Read};


use crate::image_converter::png::Png;
use crate::image_converter::{FileConverter, png};

mod image_converter;
mod vec_extension;
pub mod algorithms;


pub fn convert_file(file_path: &str) -> Result<(), anyhow::Error>{
    let file: File = File::open(file_path)?;
    let iterator: Bytes<BufReader<File>> = BufReader::new(file).bytes();
    
    // Assume file is png
    let mut png_file: Png = Png::new();
    png_file.decode(iterator)?;
    Ok(())
}