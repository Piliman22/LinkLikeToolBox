use std::collections::HashMap;
use serde_json::{Value, Map};
use crate::master::field_mappings::{get_master_map, get_field_mapping};


pub trait MasterDataType {
    fn type_name() -> &'static str;
    fn from_records(records: Vec<Map<String, Value>>) -> Result<Vec<Self>, Box<dyn std::error::Error>>
    where
        Self: Sized;
}


pub fn get_type_name_from_tsv(tsv_label: &str) -> String {
    let master_map = get_master_map();
    
    if let Some(type_name) = master_map.get(tsv_label) {
        type_name.to_string()
    } else {
        
        crate::master::parse::get_json_type_name(tsv_label)
    }
}


pub fn map_field_names(records: Vec<Map<String, Value>>, tsv_label: &str) -> Vec<Map<String, Value>> {
    if let Some(field_mapping) = get_field_mapping(tsv_label) {
        records.into_iter().map(|record| {
            let mut new_record = Map::new();
            
            for (i, field_name) in field_mapping.iter().enumerate() {
                let field_key = format!("field_{}", i);
                if let Some(value) = record.get(&field_key) {
                    new_record.insert(field_name.clone(), value.clone());
                }
            }
            
            new_record
        }).collect()
    } else {
        println!("Warning: No field mapping found for {}", tsv_label);
        records
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_type_name_from_tsv() {
        assert_eq!(get_type_name_from_tsv("gpprizeexchanges.tsv"), "GpPrizeExchanges");
        assert_eq!(get_type_name_from_tsv("carddatas.tsv"), "CardDatas");
        assert_eq!(get_type_name_from_tsv("characters.tsv"), "Characters");
    }
    
    #[test]
    fn test_field_mapping() {
        let mapping = get_field_mapping("gpprizeexchanges.tsv");
        assert!(mapping.is_some());
        let fields = mapping.unwrap();
        assert_eq!(fields[0], "Id");
        assert_eq!(fields[1], "ProductItemType");
    }
}