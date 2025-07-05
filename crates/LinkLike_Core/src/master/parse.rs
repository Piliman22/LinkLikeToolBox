use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use serde_json::{Value, Map};


pub fn parse_tsv_from_bytes(data: &[u8]) -> Result<Vec<Map<String, Value>>, Box<dyn std::error::Error>> {
    
    let text = String::from_utf8(data.to_vec())?;
    
    
    let lines: Vec<&str> = text.lines().collect();
    
    if lines.is_empty() {
        return Err("Empty TSV data".into());
    }
    
    
    let headers: Vec<&str> = lines[0].split('\t').collect();
    
    if headers.is_empty() {
        return Err("No headers found in TSV data".into());
    }
    
    println!("  Found {} columns: {:?}", headers.len(), &headers[..std::cmp::min(5, headers.len())]);
    
    let mut records = Vec::new();
    
    
    for (line_num, line) in lines.iter().skip(1).enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        
        let values: Vec<&str> = line.split('\t').collect();
        let mut record = Map::new();
        
        for (i, header) in headers.iter().enumerate() {
            let value_str = values.get(i).unwrap_or(&"").trim();
            
            
            let json_value = infer_json_value(value_str);
            record.insert(header.to_string(), json_value);
        }
        
        records.push(record);
        
        
        if (line_num + 1) % 1000 == 0 {
            println!("    Parsed {} rows", line_num + 1);
        }
    }
    
    println!("  Parsed {} total records", records.len());
    Ok(records)
}


pub fn tsv_to_json<P: AsRef<Path>>(file_path: P) -> Result<Vec<Map<String, Value>>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    
    let header_line = lines.next().ok_or("Empty file")??;
    let headers: Vec<&str> = header_line.split('\t').collect();
    
    let mut records = Vec::new();
    
    
    for line in lines {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        
        let values: Vec<&str> = line.split('\t').collect();
        let mut record = Map::new();
        
        for (i, header) in headers.iter().enumerate() {
            let value = values.get(i).unwrap_or(&"").to_string();
            record.insert(header.to_string(), Value::String(value));
        }
        
        records.push(record);
    }
    
    Ok(records)
}


pub fn get_json_type_name(tsv_label: &str) -> String {
    
    let mut result = String::new();
    let mut capitalize_next = true;
    
    for ch in tsv_label.chars() {
        if ch == '_' || ch == '.' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_uppercase().next().unwrap_or(ch));
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }
    
    
    if result.ends_with(".tsv") || result.ends_with(".Tsv") {
        result.truncate(result.len() - 4);
    }
    
    result
}


pub fn convert_tsv_to_json_file<P1: AsRef<Path>, P2: AsRef<Path>>(
    tsv_path: P1, 
    json_path: P2
) -> Result<(), Box<dyn std::error::Error>> {
    let records = tsv_to_json(tsv_path)?;
    let json_string = serde_json::to_string_pretty(&records)?;
    std::fs::write(json_path, json_string)?;
    Ok(())
}


fn infer_json_value(value_str: &str) -> Value {
    if value_str.is_empty() {
        return Value::Null;
    }
    
    
    if let Ok(int_val) = value_str.parse::<i64>() {
        return Value::Number(int_val.into());
    }
    
    
    if let Ok(float_val) = value_str.parse::<f64>() {
        if let Some(num) = serde_json::Number::from_f64(float_val) {
            return Value::Number(num);
        }
    }
    
    
    match value_str.to_lowercase().as_str() {
        "true" | "1" => return Value::Bool(true),
        "false" | "0" => return Value::Bool(false),
        _ => {}
    }
    
    
    Value::String(value_str.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_tsv_from_bytes() {
        let tsv_data = b"id\tname\tvalue\n1\ttest\t100\n2\thello\t200";
        let records = parse_tsv_from_bytes(tsv_data).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0]["id"], Value::Number(1.into()));
        assert_eq!(records[0]["name"], Value::String("test".to_string()));
    }
    
    #[test]
    fn test_get_json_type_name() {
        assert_eq!(get_json_type_name("card_datas.tsv"), "CardDatas");
        assert_eq!(get_json_type_name("music_levels.tsv"), "MusicLevels");
        assert_eq!(get_json_type_name("characters.tsv"), "Characters");
    }
    
    #[test]
    fn test_infer_json_value() {
        assert_eq!(infer_json_value("123"), Value::Number(123.into()));
        assert_eq!(infer_json_value("12.34"), Value::Number(serde_json::Number::from_f64(12.34).unwrap()));
        assert_eq!(infer_json_value("true"), Value::Bool(true));
        assert_eq!(infer_json_value("false"), Value::Bool(false));
        assert_eq!(infer_json_value("hello"), Value::String("hello".to_string()));
        assert_eq!(infer_json_value(""), Value::Null);
    }
    
    #[test]
    fn test_parse_tsv_to_json() {
        
        use std::io::Write;
        let mut temp_file = std::env::temp_dir();
        temp_file.push("test.tsv");
        
        let mut file = File::create(&temp_file).unwrap();
        file.write_all(b"id\tname\tvalue\n1\ttest\t100\n2\thello\t200").unwrap();
        file.sync_all().unwrap();
        
        let records = parse_tsv_to_json(&temp_file).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0]["id"], Value::Number(1.into()));
        assert_eq!(records[1]["name"], Value::String("hello".to_string()));
        
        
        let _ = std::fs::remove_file(&temp_file);
    }
}