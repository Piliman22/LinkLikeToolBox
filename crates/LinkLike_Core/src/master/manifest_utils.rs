use crate::master::{
    crypto::{encode_uvarint},
    encoding::base32_encode,
};
use md5;
use std::io::Write;

pub fn get_real_name(checksum: u64, label_crc: u64, size: u64) -> String {
    let mut buf = Vec::new();
    
    buf.write_all(&checksum.to_be_bytes()).unwrap();

    buf.write_all(&label_crc.to_be_bytes()).unwrap();
    
    let mut varint_slice = [0u8; 10];
    let n = encode_uvarint(size, &mut varint_slice);
    buf.write_all(&varint_slice[..n]).unwrap();
    
    let digest = md5::compute(&buf);
    
    base32_encode(&digest.0)
}