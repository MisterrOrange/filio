use std::fs::File;
use std::io::{Bytes, BufReader, Read};


use crate::image_converter::FileConverter;

mod image_converter;
mod vec_extension;


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn convert_file(file_path: &str) -> Result<(), anyhow::Error>{
    let file: File = File::open(file_path)?;
    let iterator: Bytes<BufReader<File>> = BufReader::new(file).bytes();
    
    // Assume file is png
    image_converter::png::Png::decode(iterator)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}