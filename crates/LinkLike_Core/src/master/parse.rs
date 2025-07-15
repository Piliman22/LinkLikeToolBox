use std::fs::File;
use std::io::{BufRead, BufReader, Read, Cursor};
use std::path::Path;
use serde_json::{Value, Map};
use byteorder::{BigEndian, ReadBytesExt};

/// バイナリTSVファイルをパースしてJSONレコードに変換
pub fn parse_tsv_from_bytes(data: &[u8]) -> Result<Vec<Map<String, Value>>, Box<dyn std::error::Error>> {
    let mut cursor = std::io::Cursor::new(data);
    
    // マジックナンバーをチェック (0xDA00)
    let magic = cursor.read_u16::<BigEndian>()?;
    if magic != 0xDA00 {
        return Err(format!("Invalid magic number: expected 0xDA00, got 0x{:04X}", magic).into());
    }
    
    // 不明な2バイトをスキップ
    cursor.read_u16::<BigEndian>()?;
    
    // VLQ形式で行数を読み取り
    let row_num = read_uvarint(&mut cursor)?;
    if row_num == 0 {
        println!("  Database file has 0 rows.");
        return Ok(Vec::new());
    }
    
    // VLQ形式でフィールド数を読み取り
    let field_num = read_uvarint(&mut cursor)?;
    if field_num == 0 {
        return Err("No fields found".into());
    }
    
    println!("  Found {} rows, {} fields", row_num, field_num);
    
    // フィールド情報を読み取り（Goコードと同じ順序）
    let mut _field_names = Vec::new();
    let mut field_types = Vec::new();
    
    for _ in 0..field_num {
        let field_name_crc = cursor.read_u32::<BigEndian>()?;
        let field_type = cursor.read_u32::<BigEndian>()?;
        _field_names.push(field_name_crc);
        field_types.push(field_type);
    }
    
    let mut records = Vec::new();
    
    // Goコードの実装に従い、フィールド毎に全行を読み取る
    let mut all_field_data: Vec<Vec<Value>> = vec![Vec::new(); field_num as usize];
    
    for field_idx in 0..field_num {
        let field_type = field_types[field_idx as usize];
        
        for _row_idx in 0..row_num {
            match read_field_value(&mut cursor, field_type) {
                Ok(value) => {
                    all_field_data[field_idx as usize].push(value);
                }
                Err(e) => {
                    return Err(format!("Error reading field {} at row {}: {}", field_idx, _row_idx, e).into());
                }
            }
        }
    }
    
    // データを行単位で再構成
    for row_idx in 0..row_num {
        let mut record = Map::new();
        
        for field_idx in 0..field_num {
            let field_name = format!("field_{}", field_idx);
            if let Some(value) = all_field_data[field_idx as usize].get(row_idx as usize) {
                record.insert(field_name, value.clone());
            }
        }
        
        records.push(record);
        
        // 進捗表示
        if (row_idx + 1) % 1000 == 0 {
            println!("    Parsed {} rows", row_idx + 1);
        }
    }
    
    println!("  Parsed {} total records", records.len());
    Ok(records)
}

/// Goコードに準拠したVLQ読み取り（binary.ReadUvarint相当）
fn read_uvarint(reader: &mut Cursor<&[u8]>) -> Result<u64, Box<dyn std::error::Error>> {
    let mut result = 0u64;
    let mut shift = 0;
    
    loop {
        let byte = reader.read_u8().map_err(|e| format!("Failed to read byte for uvarint: {}", e))?;
        
        result |= ((byte & 0x7F) as u64) << shift;
        
        if (byte & 0x80) == 0 {
            break;
        }
        
        shift += 7;
        if shift >= 64 {
            return Err("VLQ integer too large".into());
        }
    }
    
    Ok(result)
}

/// NULLターミネートされた文字列を読み取り
fn read_null_terminated_string(reader: &mut Cursor<&[u8]>) -> Result<String, Box<dyn std::error::Error>> {
    let mut bytes = Vec::new();
    
    loop {
        let byte = reader.read_u8().map_err(|e| format!("Failed to read byte for string: {}", e))?;
        
        if byte == 0 {
            break;
        }
        bytes.push(byte);
    }
    
    // UTF-8エラーの場合はlossyな変換を使用
    match String::from_utf8(bytes) {
        Ok(s) => Ok(s),
        Err(e) => {
            let invalid_bytes = e.into_bytes();
            let lossy = String::from_utf8_lossy(&invalid_bytes);
            Ok(lossy.to_string())
        }
    }
}

/// フィールド値を読み取り（Goコードに完全対応）
fn read_field_value(reader: &mut Cursor<&[u8]>, field_type: u32) -> Result<Value, Box<dyn std::error::Error>> {
    match field_type {
        0x10 => {
            // 文字列またはDateTime
            let s = read_null_terminated_string(reader)?;
            
            // DateTimeフォーマットかチェック（Goコードのtime.DateTime = "2006-01-02 15:04:05"）
            if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S") {
                // Goコードと同じISO 8601形式で返す
                Ok(Value::String(dt.format("%Y-%m-%dT%H:%M:%S").to_string()))
            } else {
                Ok(Value::String(s))
            }
        }
        0x20 => {
            // int (VLQ形式) - Goコードでは32bit符号付きintに変換
            let val = read_uvarint(reader)?;
            // Goコードの変換ロジックに従う: *(*int32)(unsafe.Pointer(&uNum))
            let signed_val = val as u32 as i32; // 32bitとして切り詰めてから符号付きに変換
            Ok(Value::Number((signed_val as i64).into()))
        }
        0x33 => {
            // long (8バイト固定)
            let val = reader.read_u64::<BigEndian>()?;
            Ok(Value::Number((val as i64).into()))
        }
        _ => {
            Err(format!("Unknown field type: 0x{:02X}", field_type).into())
        }
    }
}

/// TSVファイル名からJSON型名を生成（Goコードのreflect.TypeOf(ins).Name()に対応）
pub fn get_json_type_name(tsv_label: &str) -> String {
    // Goコードのマスターマップを参考に型名を生成
    let base_name = tsv_label.trim_end_matches(".tsv");
    
    // キャメルケースに変換（最初の文字は大文字）
    let mut result = String::new();
    let mut capitalize_next = true;
    
    for ch in base_name.chars() {
        if ch == '_' || ch == '.' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_uppercase().next().unwrap_or(ch));
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }
    
    result
}

/// ファイルパスからTSVをパースする
pub fn tsv_to_json<P: AsRef<Path>>(file_path: P) -> Result<Vec<Map<String, Value>>, Box<dyn std::error::Error>> {
    let data = std::fs::read(file_path)?;
    parse_tsv_from_bytes(&data)
}

/// TSVファイルをYAMLファイルに変換（適切なフィールド名付きで）
pub fn convert_tsv_to_yaml_file<P1: AsRef<Path>, P2: AsRef<Path>>(
    tsv_path: P1, 
    yaml_path: P2,
    label: &str
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::master::mapping::{get_type_name_from_tsv, map_field_names};
    
    let records = tsv_to_json(tsv_path)?;
    
    // フィールド名を適切にマッピング
    let mapped_records = map_field_names(records, label);
    
    // Goコードと同じ型名を使用
    let type_name = get_type_name_from_tsv(label);
    
    // YAML形式で出力
    let yaml_string = serde_yaml::to_string(&mapped_records)?;
    std::fs::write(yaml_path, yaml_string)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_json_type_name() {
        assert_eq!(get_json_type_name("gpprizeexchanges.tsv"), "Gpprizeexchanges");
        assert_eq!(get_json_type_name("card_datas.tsv"), "CardDatas");
        assert_eq!(get_json_type_name("music_levels.tsv"), "MusicLevels");
    }
    
    #[test]
    fn test_read_uvarint() {
        let data = vec![0x80, 0x01]; // 128 in VLQ
        let mut cursor = std::io::Cursor::new(&data[..]);
        assert_eq!(read_uvarint(&mut cursor).unwrap(), 128);
    }
}