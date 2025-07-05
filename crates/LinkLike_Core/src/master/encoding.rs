use std::io::Read;

/// Goのcrypto.Base32Encoderと同じ実装
pub fn base32_encode(data: &[u8]) -> String {
    // Goのカスタムアルファベット: "abcdefghijklmnopqrstuvwxyz234567"
    const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz234567";
    
    let mut result = String::new();
    let mut buffer = 0u64;
    let mut bits_in_buffer = 0;
    
    for &byte in data {
        buffer = (buffer << 8) | (byte as u64);
        bits_in_buffer += 8;
        
        while bits_in_buffer >= 5 {
            bits_in_buffer -= 5;
            let index = ((buffer >> bits_in_buffer) & 0x1F) as usize;
            result.push(ALPHABET[index] as char);
        }
    }
    
    // 残りのビットを処理（パディングなし）
    if bits_in_buffer > 0 {
        let index = ((buffer << (5 - bits_in_buffer)) & 0x1F) as usize;
        result.push(ALPHABET[index] as char);
    }
    
    result
}

/// VLQ読み取りの実装
pub fn read_uvarint<R: Read>(reader: &mut R) -> Result<u64, std::io::Error> {
    let mut result = 0u64;
    let mut shift = 0;
    
    loop {
        let mut byte = [0u8; 1];
        reader.read_exact(&mut byte)?;
        let b = byte[0];
        
        result |= ((b & 0x7F) as u64) << shift;
        
        if (b & 0x80) == 0 {
            break;
        }
        
        shift += 7;
        if shift >= 64 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "VLQ too long"
            ));
        }
    }
    
    Ok(result)
}

/// null終端バイトまで読み取り
pub fn read_until_null_byte<R: Read>(reader: &mut R) -> Result<Vec<u8>, std::io::Error> {
    let mut result = Vec::new();
    let mut byte = [0u8; 1];
    
    loop {
        reader.read_exact(&mut byte)?;
        if byte[0] == 0 {
            break;
        }
        result.push(byte[0]);
    }
    
    Ok(result)
}