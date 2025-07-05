use sha2::{Sha256, Digest};
use aes::Aes128;
use aes::cipher::{BlockDecryptMut, KeyIvInit};
use cbc::Decryptor;
use crate::master::compression::try_lz4_decompress_detailed;


#[derive(Debug)]
pub struct ManifestCryptoData {
    pub seed: u64,
    pub calc_crc64_name: String,
    pub size: u64,
}


pub fn decode_asset_with_data(manifest_data: &ManifestCryptoData, src: &[u8], dst: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    const PREFIX: &str = "c34ea77df4976cd8907096fa47bb97e61852305892494e3692ba0c7eb434f022c549c96cf7ca0ee1b6ba7f203b6c76e8679699ce9c44af7b1cb000173a515938";
    
    let mut buf = Vec::new();
    
    
    let hex_prefix = hex::decode(PREFIX)?;
    buf.extend_from_slice(&hex_prefix);
    
    
    buf.extend_from_slice(&manifest_data.seed.to_be_bytes());
    
    
    let crc64 = update_crc64(0, manifest_data.calc_crc64_name.as_bytes());
    buf.extend_from_slice(&crc64.to_be_bytes());
    
    
    let crc32 = update_crc32(0, b"android");
    buf.extend_from_slice(&crc32.to_be_bytes());
    
    
    let mut varint_buf = [0u8; 10];
    let n = encode_uvarint(manifest_data.size, &mut varint_buf);
    buf.extend_from_slice(&varint_buf[..n]);
    
    println!("Key derivation buffer length: {} bytes", buf.len());
    println!("Manifest details - Seed: {}, CRC64: {}, CRC32: {}, Size: {}", 
             manifest_data.seed, crc64, crc32, manifest_data.size);
    
    
    let mut hasher = Sha256::new();
    hasher.update(&buf);
    let keyiv = hasher.finalize();
    
    let key = &keyiv[..16];
    let iv = &keyiv[16..];
    
    println!("Source data length: {} bytes", src.len());
    println!("Key: {}", hex::encode(key));
    println!("IV: {}", hex::encode(iv));
    
    
    let cipher = Decryptor::<Aes128>::new(key.into(), iv.into());
    let mut decrypted = src.to_vec();
    
    
    if decrypted.len() % 16 != 0 {
        return Err("Invalid AES block size".into());
    }
    
    let decrypted_data = cipher.decrypt_padded_mut::<aes::cipher::block_padding::Pkcs7>(&mut decrypted)
        .map_err(|e| format!("AES decryption failed: {:?}", e))?;
    
    println!("Decrypted data length: {} bytes", decrypted_data.len());
    
    
    if decrypted_data.len() >= 32 {
        println!("Decrypted header (32 bytes): {}", hex::encode(&decrypted_data[..32]));
        
        
        for i in 0..std::cmp::min(16, decrypted_data.len()) {
            println!("Byte {}: 0x{:02x} ({})", i, decrypted_data[i], decrypted_data[i]);
        }
    }
    
    
    match try_lz4_decompress_detailed(decrypted_data, manifest_data.size) {
        Ok(decompressed) => {
            println!("LZ4 decompression successful: {} bytes", decompressed.len());
            
            
            if decompressed.len() >= 16 {
                println!("Decompressed header: {}", hex::encode(&decompressed[..16]));
                let magic = u16::from_be_bytes([decompressed[0], decompressed[1]]);
                println!("Decompressed magic number: 0x{:04x}", magic);
            }
            
            dst.extend_from_slice(&decompressed);
        }
        Err(e) => {
            println!("LZ4 decompression failed: {}", e);
            
            
            println!("Attempting to use decrypted data directly...");
            dst.extend_from_slice(decrypted_data);
        }
    }
    
    Ok(())
}


pub fn update_crc64(_crc: u64, data: &[u8]) -> u64 {
    use crc::{Crc, CRC_64_ECMA_182};
    
    const CRC64: Crc<u64> = Crc::<u64>::new(&CRC_64_ECMA_182);
    CRC64.checksum(data)
}


pub fn update_crc32(_crc: u32, data: &[u8]) -> u32 {
    use crc::{Crc, CRC_32_ISO_HDLC};
    
    const CRC32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    CRC32.checksum(data)
}


pub fn encode_uvarint(mut x: u64, buf: &mut [u8]) -> usize {
    let mut i = 0;
    while x >= 0x80 {
        buf[i] = (x as u8) | 0x80;
        x >>= 7;
        i += 1;
    }
    buf[i] = x as u8;
    i + 1
}