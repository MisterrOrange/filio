use crate::image_converter::{FileConverter};
use crate::vec_extension::{VecExt, VecExtU8};

use core::error;
use std::io::{self, BufReader, Bytes};
use std::fs::File;
use thiserror::Error;



#[derive(Error, Debug)]
pub enum PngError {
    #[error("invalid png header (expected {expected:?}, found {found:?}")]
    InvalidPngHeader {
        expected: String,
        found: String
    },
    #[error("end of file reached")]
    EndOfFileReached,
    #[error("failed to read file")]
    FileReadError,
    #[error("invalid chunk length (expected {expected:?}, found {found:?}")]
    InvalidChunkLength {
        expected: u32,
        found: u32
    },
    #[error("end of png reached (this error is for library maintainers and shouldn't be propagated out of the lib)")]
    PngEndReached,
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
    pub fn new() -> Png {
        Png {
            image_header: None
        }
    }

    fn verify_header(&self, it: &mut Bytes<BufReader<File>>) -> Result<(), PngError> {
        let correct_header: Vec<u8> = vec![137, 80, 78, 71, 13, 10, 26, 10];

        let png_header: Vec<u8> = self.read_bytes(it, 8)?;
        if png_header.verify_elements(&correct_header) {
            Ok(())
        }
        else {
            Err(PngError::InvalidPngHeader { expected: (correct_header.to_string()), found: (png_header.to_string()) })
        }
    }

    fn read_bytes(&self, it: &mut Bytes<BufReader<File>>, amount: usize) -> Result<Vec<u8>, PngError> {
        let bytes: Vec<Result<u8, io::Error>> = it.take(amount).collect();
        let bytes: Result<Vec<u8>, io::Error> = bytes.into_iter().collect();
        match bytes {
            Ok(value) => Ok(value),
            Err(_) => Err(PngError::FileReadError)
        }
    }

    fn decode_chunk(&mut self, it: &mut Bytes<BufReader<File>>, chunk_length: u32) -> Result<(), PngError>{
        let chunk_name_vec = self.read_bytes(it, 4)?;
        let chunk_name: &[u8] = chunk_name_vec.as_slice();
        let chunk_data = self.read_bytes(it, chunk_length as usize)?;
        // = Cyclic redundancy check = checksum to verify integrity of data 
        let crc = self.read_bytes(it, 4);



        match chunk_name {
            // IHDR chunk
            [73, 72, 68, 82] => {self.process_IHDR(it, chunk_length)?;}


            _ => {/*TODO: ADD CODE TO SKIP UNKNOWN CHUNK */}
        }
        Ok(())
    }



    fn process_IHDR(&mut self, it: &mut Bytes<BufReader<File>>, chunk_length: u32) -> Result<(), PngError> {
        if chunk_length != 13 {
            return Err(PngError::InvalidChunkLength { expected: (13), found: (chunk_length) })
        }

        let width: u32 = self.read_bytes(it, 4)?.to_u32();
        let height: u32 = self.read_bytes(it, 4)?.to_u32();
        let bit_depth: u8 = self.read_bytes(it, 1)?.to_u8();
        let colour_type: u8 = self.read_bytes(it, 1)?.to_u8();
        let compression_method: u8 = self.read_bytes(it, 1)?.to_u8();
        let filter_method: u8 = self.read_bytes(it, 1)?.to_u8();
        let interlace_method: u8 = self.read_bytes(it, 1)?.to_u8();
        
        self.image_header = Some(IHDR {
            width: width,
            height: height,
            bit_depth: bit_depth,
            colour_type: colour_type,
            compression_method: compression_method,
            filter_method: filter_method,
            interlace_method: interlace_method
        });
        Ok(())
    }
}

impl FileConverter for Png {
    type Error = PngError;
    fn decode(&mut self, mut it: Bytes<BufReader<File>>) -> Result<(), PngError> {
        self.verify_header(&mut it)?;
        
        loop {
            let data_length = self.read_bytes(&mut it, 4)?.to_u32();
            match self.decode_chunk(&mut it, data_length) {
                Ok(_) => {}
                Err(error) => {
                    match error {
                        PngError::PngEndReached => {break;}
                        _ => {return Err(error)}
                    }
                }
            }

        }

        Ok(())
    }
}

