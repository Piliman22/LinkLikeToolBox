use std::io::{self, Read, Write,};
use std::fs::File;
use sha2::{Sha256, Digest};
use aes::Aes128;
use aes::cipher::{BlockDecryptMut, KeyIvInit};
use cbc::Decryptor;
use lz4_flex::frame::FrameDecoder;
use crc::{Crc, CRC_64_ECMA_182};

type Aes128CbcDec = Decryptor<Aes128>;

const PREFIX: &str = "c34ea77df4976cd8907096fa47bb97e61852305892494e3692ba0c7eb434f022c549c96cf7ca0ee1b6ba7f203b6c76e8679699ce9c44af7b1cb000173a515938";
const RAW_STR: &str = "raw";
const CATALOG_STR: &str = "android";

const CRC64: Crc<u64> = Crc::<u64>::new(&CRC_64_ECMA_182);

#[derive(Debug, Clone, PartialEq)]
pub enum AssetType {
    Raw,
    Catalog,
}

pub struct AssetDecoder {
    pub asset_type: AssetType,
    pub seed: u64,
    pub calc_crc64_name: String,
    pub size: u64,
}

impl AssetDecoder {
    pub fn new(asset_type: AssetType, seed: u64, calc_crc64_name: String, size: u64) -> Self {
        Self {
            asset_type,
            seed,
            calc_crc64_name,
            size,
        }
    }

    pub fn decode_asset<R: Read, W: Write>(&self, mut src: R, mut dst: W) -> Result<(), Box<dyn std::error::Error>> {
        
        let hex_prefix = hex::decode(PREFIX)?;
        let mut buf = Vec::new();
        buf.extend_from_slice(&hex_prefix);

        
        let seed = match self.asset_type {
            AssetType::Raw => self.seed & 0x7FFFFFFFFFFFFFFF,
            AssetType::Catalog => self.seed,
        };
        buf.extend_from_slice(&seed.to_be_bytes());

        
        let crc64 = self.update_crc64(0, self.calc_crc64_name.as_bytes());
        buf.extend_from_slice(&crc64.to_be_bytes());

        
        let crc32_str = match self.asset_type {
            AssetType::Raw => RAW_STR,
            AssetType::Catalog => CATALOG_STR, 
        };
        let crc32 = self.update_crc32(0, crc32_str.as_bytes());
        buf.extend_from_slice(&crc32.to_be_bytes());

        
        let mut vlq_buf = Vec::new();
        self.write_uvarint(&mut vlq_buf, self.size)?;
        buf.extend_from_slice(&vlq_buf);

        
        let keyiv = Sha256::digest(&buf);
        let key = &keyiv[..16];
        let iv = &keyiv[16..];

        
        let mut encrypted_data = Vec::new();
        src.read_to_end(&mut encrypted_data)?;

        
        let decrypted_data = self.decrypt(key, iv, &encrypted_data)?;

        
        let mut decoder = FrameDecoder::new(&decrypted_data[..]);
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data)?;

        dst.write_all(&decompressed_data)?;
        Ok(())
    }

    fn decrypt(&self, key: &[u8], iv: &[u8], data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        use aes::cipher::block_padding::Pkcs7;
        
        if data.len() % 16 != 0 {
            return Err(format!("Encrypted data size {} is not aligned to block size 16", data.len()).into());
        }

        let mut buffer = data.to_vec();
        
        let cipher = Aes128CbcDec::new(key.into(), iv.into());
        match cipher.decrypt_padded_mut::<Pkcs7>(&mut buffer) {
            Ok(decrypted) => Ok(decrypted.to_vec()),
            Err(_) => {
                println!("Warning: Padding error for asset, trying alternative decryption");
                
                
                let mut cipher = Aes128CbcDec::new(key.into(), iv.into());
                let mut buffer = data.to_vec();
                
                
                let mut blocks: Vec<_> = buffer
                    .chunks_exact_mut(16)
                    .map(|chunk| {
                        let mut block = [0u8; 16];
                        block.copy_from_slice(chunk);
                        block.into()
                    })
                    .collect();
                
                cipher.decrypt_blocks_mut(&mut blocks);
                
                
                for (i, block) in blocks.iter().enumerate() {
                    let start = i * 16;
                    buffer[start..start + 16].copy_from_slice(block);
                }
                
                
                if let Some(&padding_len) = buffer.last() {
                    let padding_len = padding_len as usize;
                    if padding_len > 0 && padding_len <= 16 && padding_len <= buffer.len() {
                        let start_pos = buffer.len() - padding_len;
                        
                        if buffer[start_pos..].iter().all(|&b| b == padding_len as u8) {
                            buffer.truncate(start_pos);
                        }
                    }
                }
                
                Ok(buffer)
            }
        }
    }

    fn write_uvarint(&self, buf: &mut Vec<u8>, value: u64) -> Result<(), Box<dyn std::error::Error>> {
        let mut x = value;
        while x >= 0x80 {
            buf.push((x as u8) | 0x80);
            x >>= 7;
        }
        buf.push(x as u8);
        Ok(())
    }

    fn update_crc64(&self, _crc: u64, data: &[u8]) -> u64 {
        CRC64.checksum(data)
    }

    fn update_crc32(&self, _crc: u32, data: &[u8]) -> u32 {
        crc32fast::hash(data)
    }
}

#[derive(Debug, Clone)]
pub enum ResourceType {
    Raw = 0,
    RawAlt = 128,
    AssetBundle = 1,
    Encrypted = 192,
}

impl From<u32> for ResourceType {
    fn from(value: u32) -> Self {
        match value {
            0 => ResourceType::Raw,
            128 => ResourceType::RawAlt,
            1 => ResourceType::AssetBundle,
            192 => ResourceType::Encrypted,
            _ => ResourceType::Raw, 
        }
    }
}

pub struct AssetProcessor;

impl AssetProcessor {
    
    pub fn decrypt_all_assets(
        catalog: &crate::manifest::Catalog,
        dst_dir: &str,
        src_dir: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let amount = catalog.entries.len();
        
        
        std::fs::create_dir_all(dst_dir)?;
        
        let mut error_count = 0;
        let mut success_count = 0;
        
        println!("Starting decryption of {} assets...", amount);
        
        for (counter, entry) in catalog.entries.iter().enumerate() {
            let counter = counter + 1;
            
            
            let suffix = if entry.resource_type == 1 { ".assetbundle" } else { "" };
            
            
            let src_path = format!("{}/{}", src_dir, entry.real_name);
            let dst_path = format!("{}/{}{}", dst_dir, entry.str_label_crc, suffix);
            
            
            if !std::path::Path::new(&src_path).exists() {
                error_count += 1;
                continue;
            }
            
            
            let resource_type = ResourceType::from(entry.resource_type);
            
            let result: Result<(), Box<dyn std::error::Error>> = match resource_type {
                ResourceType::Raw | ResourceType::RawAlt => {
                    
                    std::fs::copy(&src_path, &dst_path)
                        .map_err(|e| -> Box<dyn std::error::Error> { 
                            format!("Error copying file {}: {}", src_path, e).into() 
                        })?;
                    Ok(())
                }
                ResourceType::AssetBundle => {
                    
                    let mut raw_file = File::open(&src_path)?;
                    let mut plain_file = File::create(&dst_path)?;
                    
                    
                    let mut skip_bytes = [0u8; 2];
                    raw_file.read_exact(&mut skip_bytes)?;
                    
                    
                    io::copy(&mut raw_file, &mut plain_file)?;
                    Ok(())
                }
                ResourceType::Encrypted => {
                    
                    if counter <= 3 {  
                        println!("Decrypting encrypted asset: {} (seed: {}, size: {})", 
                                 entry.str_label_crc, entry.seed, entry.size);
                    }
                    
                    let decoder = AssetDecoder::new(
                        AssetType::Raw,
                        entry.seed,
                        entry.str_label_crc.clone(),
                        entry.size,
                    );
                    
                    let raw_file = File::open(&src_path)?;
                    let plain_file = File::create(&dst_path)?;
                    
                    decoder.decode_asset(raw_file, plain_file)
                        .map_err(|e| -> Box<dyn std::error::Error> { 
                            format!("Error decrypting {}: {}", src_path, e).into() 
                        })?;
                    Ok(())
                }
            };
            
            match result {
                Ok(_) => {
                    success_count += 1;
                    if counter % 50 == 0 || entry.str_label_crc.starts_with("rhythmgame_chart") {
                        println!("({}/{}) Asset file {:?} -> {} processed", 
                                 counter, amount, entry.real_name, entry.str_label_crc);
                    }
                }
                Err(e) => {
                    eprintln!("Error processing {}: {}", src_path, e);
                    error_count += 1;
                    
                    
                    if error_count > 20 {
                        return Err("Too many decryption errors, aborting".into());
                    }
                }
            }
        }
        
        println!("Decryption completed: {} successful, {} errors", success_count, error_count);
        Ok(())
    }
}