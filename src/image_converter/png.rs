use crate::image_converter::{FileConverter, RawImage};
use crate::vec_extension::VecExt;

use std::io::{self, BufReader, Bytes};
use std::fs::File;
use anyhow::{Result};
use thiserror::Error;



#[derive(Error, Debug)]
enum PngError {
    #[error("invalid png header (expected {expected:?}, found {found:?}")]
    InvalidPngHeader {
        expected: String,
        found: String
    },
    #[error("end of file reached")]
    EndOfFileReached,
    #[error("failed to read file")]
    FileReadError,
}


pub struct Png {
    image_header: Option<IHDR>
}

struct IHDR {
    width: u32,
    height: u32,
    bit_depth: u8,
    colour_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8
}


impl Png {
    fn new() -> Png {
        Png {
            image_header: None
        }
    }

    fn verify_header(it: &mut Bytes<BufReader<File>>) -> Result<(), PngError> {
        let correct_header: Vec<u8> = vec![137, 80, 78, 71, 13, 10, 26, 10];

        let png_header: Vec<Result<u8, io::Error>> = it.take(8).collect();
        let png_header: Result<Vec<u8>, io::Error>  = png_header.into_iter().collect();
        match png_header {
            Ok(header) => {
                if header.verify_elements(&correct_header) {
                    Ok(())
                }
                else {
                    Err(PngError::InvalidPngHeader { expected: (correct_header.to_string()), found: (header.to_string()) })
                }
            }
            Err(_) => {
                Err(PngError::FileReadError)
            }
        }
    }
}

impl FileConverter for Png {
    fn decode(mut it: Bytes<BufReader<File>>) -> Result<Png> {
        Png::verify_header(&mut it)?;
        
        for (index, byte) in it.enumerate() {

        }
        Ok(Png::new())
    }
}

