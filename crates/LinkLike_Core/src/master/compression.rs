use std::io::{Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};

/// 詳細なLZ4解凍デバッグ
pub fn try_lz4_decompress_detailed(data: &[u8], expected_size: u64) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if data.len() < 8 {
        return Err("Data too short for LZ4".into());
    }
    
    println!("LZ4 decompression attempts:");
    println!("Data length: {} bytes", data.len());
    println!("Expected uncompressed size: {} bytes", expected_size);
    
    // hexクレートが必要な場合のみ使用
    #[cfg(feature = "hex")]
    println!("First 16 bytes: {}", hex::encode(&data[..std::cmp::min(16, data.len())]));
    
    #[cfg(not(feature = "hex"))]
    println!("First 16 bytes: {:?}", &data[..std::cmp::min(16, data.len())]);
    
    // LZ4のマジックナンバーをチェック
    let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    println!("LZ4 magic number: 0x{:08x}", magic);
    
    // 方法1: LZ4フレーム形式での解凍
    println!("Trying LZ4 frame decompression...");
    if let Ok(result) = try_lz4_frame_decompress(data) {
        println!("LZ4 frame decompression successful: {} bytes", result.len());
        return Ok(result);
    }
    
    // 方法2: カスタムLZ4ヘッダー解析
    println!("Trying custom LZ4 header analysis...");
    if let Ok(result) = try_custom_lz4_decompress(data, expected_size) {
        println!("Custom LZ4 decompression successful: {} bytes", result.len());
        return Ok(result);
    }
    
    // 方法3: サイズプレフィックス付き（標準）
    println!("Trying size-prepended decompression...");
    if let Ok(result) = lz4_flex::decompress_size_prepended(data) {
        println!("Size-prepended decompression successful: {} bytes", result.len());
        return Ok(result);
    }
    
    // 方法4: 8バイトスキップしてからLZ4解凍
    if data.len() > 8 {
        println!("Trying LZ4 decompression with 8-byte skip...");
        if let Ok(result) = lz4_flex::decompress(&data[8..], expected_size as usize) {
            println!("8-byte skip decompression successful: {} bytes", result.len());
            return Ok(result);
        }
    }
    
    // 方法5: 4バイトスキップしてからLZ4解凍
    if data.len() > 4 {
        println!("Trying LZ4 decompression with 4-byte skip...");
        if let Ok(result) = lz4_flex::decompress(&data[4..], expected_size as usize) {
            println!("4-byte skip decompression successful: {} bytes", result.len());
            return Ok(result);
        }
    }
    
    // 方法6: 最初の4バイトをサイズとして解釈
    if data.len() >= 4 {
        let mut cursor = Cursor::new(&data[..4]);
        if let Ok(size) = cursor.read_u32::<LittleEndian>() {
            println!("Trying with LittleEndian size header: {} bytes", size);
            if size > 0 && size < 100_000_000 { // 合理的なサイズチェック
                if let Ok(result) = lz4_flex::decompress(&data[4..], size as usize) {
                    println!("LittleEndian decompression successful: {} bytes", result.len());
                    return Ok(result);
                }
            }
        }
    }
    
    // 方法7: 圧縮されていない可能性をチェック
    println!("Checking if data is uncompressed...");
    if data.len() >= 2 {
        let magic = u16::from_be_bytes([data[0], data[1]]);
        if magic == 0xCA01 {
            println!("Found magic number 0xCA01 - data appears to be uncompressed");
            return Ok(data.to_vec());
        }
    }
    
    Err("All LZ4 decompression methods failed".into())
}

/// LZ4フレーム形式での解凍を試行
pub fn try_lz4_frame_decompress(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // LZ4フレームのマジックナンバー: 0x184D2204
    const LZ4_MAGIC: u32 = 0x184D2204;
    
    if data.len() < 4 {
        return Err("Data too short for LZ4 frame".into());
    }
    
    let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    
    if magic == LZ4_MAGIC {
        println!("Found LZ4 frame magic number");
        
        let mut decoder = lz4_flex::frame::FrameDecoder::new(data);
        let mut result = Vec::new();
        match decoder.read_to_end(&mut result) {
            Ok(_) => return Ok(result),
            Err(e) => {
                println!("LZ4 frame read failed: {}", e);
            }
        }
    }
    
    Err("Not a valid LZ4 frame".into())
}

/// カスタムLZ4ヘッダー解析
pub fn try_custom_lz4_decompress(data: &[u8], expected_size: u64) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if data.len() < 16 {
        return Err("Data too short for custom LZ4 header".into());
    }
    
    let mut cursor = Cursor::new(data);
    
    // 最初の4バイト: LZ4マジック（可能性）
    let magic = cursor.read_u32::<LittleEndian>()?;
    println!("Potential LZ4 magic: 0x{:08x}", magic);
    
    // 次の4バイト: 非圧縮サイズ（可能性）
    let uncompressed_size = cursor.read_u32::<LittleEndian>()?;
    println!("Potential uncompressed size: {} bytes", uncompressed_size);
    
    // 次の4バイト: 圧縮サイズ（可能性）
    let compressed_size = cursor.read_u32::<LittleEndian>()?;
    println!("Potential compressed size: {} bytes", compressed_size);
    
    // 次の4バイト: その他のヘッダー情報
    let header4 = cursor.read_u32::<LittleEndian>()?;
    println!("Header field 4: 0x{:08x}", header4);
    
    // 16バイト後からLZ4データが始まると仮定
    let lz4_data = &data[16..];
    
    // 解凍を試行（非圧縮サイズを使用）
    if uncompressed_size > 0 && uncompressed_size < 100_000_000 {
        if let Ok(result) = lz4_flex::decompress(lz4_data, uncompressed_size as usize) {
            println!("Custom LZ4 decompression with header size successful");
            return Ok(result);
        }
    }
    
    // 期待されるサイズで解凍を試行
    if let Ok(result) = lz4_flex::decompress(lz4_data, expected_size as usize) {
        println!("Custom LZ4 decompression with expected size successful");
        return Ok(result);
    }
    
    // 12バイト後からLZ4データが始まると仮定
    let lz4_data_12 = &data[12..];
    if let Ok(result) = lz4_flex::decompress(lz4_data_12, expected_size as usize) {
        println!("Custom LZ4 decompression with 12-byte skip successful");
        return Ok(result);
    }
    
    Err("Custom LZ4 decompression failed".into())
}