// LinkLike chartdata

use std::io::{self, Read, Write};
use flate2::Compression;
use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
use std::fs::File;
use std::path::Path;

pub struct Chart {
    data: Vec<u8>,
}

impl Chart {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data } 
    }

    // load from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Ok(Self::new(data))
    }

    // save to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(&self.data)?;
        Ok(())
    }

    // unzips the chart data use RawDeflate
    pub fn decompress(&self) -> io::Result<Vec<u8>> {
        let mut decoder = DeflateDecoder::new(self.data.as_slice());
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data)?;
        Ok(decompressed_data)
    }

    pub fn compress(data: &[u8], level: u32) -> io::Result<Vec<u8>> {
        let mut compressed_data = Vec::new();
        {
            // Use DeflateEncoder for compression
            let mut encoder = DeflateEncoder::new(&mut compressed_data, Compression::new(level));
            encoder.write_all(data)?;
            encoder.finish()?;
        }
        Ok(compressed_data)
    }

    // get unzipped data
    pub fn get_decompressed_data(&self) -> io::Result<Vec<u8>> {
        self.decompress()
    }
    
    // get compressed data
    pub fn from_uncompressed(data: &[u8], level: u32) -> io::Result<Self> {
        let compressed = Self::compress(data, level)?;
        Ok(Self::new(compressed))
    }
}