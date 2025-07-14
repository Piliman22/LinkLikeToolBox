use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Cursor};
use std::collections::HashMap;
use byteorder::{BigEndian, ReadBytesExt};

use crate::master::{
    crypto::{decode_asset_with_data, update_crc64, update_crc32, ManifestCryptoData},
    encoding::{read_uvarint, read_until_null_byte},
    manifest_utils::get_real_name,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub priority: u32,
    pub resource_type: u32,
    pub num_deps: u32,
    pub num_contents: u32,
    pub num_categories: u32,
    pub size: u64,
    pub content_type_crcs: Vec<u8>,
    pub type_crc: u32,
    pub category_crcs: Vec<u8>,
    pub label_crc: u64,
    pub content_name_crcs: Vec<u8>,
    pub dep_crcs: Vec<u8>,
    pub rec_dep_crcs: Vec<u8>,
    pub num_rec_dep_crcs: u32,
    pub checksum: u64,
    pub seed: u64,
    pub str_type_crc: String,
    pub str_content_type_crcs: Vec<String>,
    pub str_category_crcs: Vec<String>,
    pub str_label_crc: String,
    pub str_content_name_crcs: Vec<String>,
    pub str_dep_crcs: Vec<String>,
    pub real_name: String,
}

impl Entry {
    
    pub fn new_manifest(real_name: String) -> Self {
        Self {
            priority: 0,
            resource_type: 999,
            num_deps: 0,
            num_contents: 0,
            num_categories: 0,
            size: 0,
            content_type_crcs: Vec::new(),
            type_crc: 0,
            category_crcs: Vec::new(),
            label_crc: 0,
            content_name_crcs: Vec::new(),
            dep_crcs: Vec::new(),
            rec_dep_crcs: Vec::new(),
            num_rec_dep_crcs: 0,
            checksum: 0,
            seed: 0,
            str_type_crc: "Manifest".to_string(),
            str_content_type_crcs: Vec::new(),
            str_category_crcs: Vec::new(),
            str_label_crc: "Manifest".to_string(),
            str_content_name_crcs: Vec::new(),
            str_dep_crcs: Vec::new(),
            real_name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Catalog {
    pub entries: Vec<Entry>,
}

impl Catalog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn init(&mut self, manifest: &Manifest, mut file: File) -> Result<(), Box<dyn std::error::Error>> {
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        
        println!("Manifest file size: {} bytes", contents.len());
        
        let crypto_data = ManifestCryptoData {
            seed: manifest.seed,
            calc_crc64_name: manifest.calc_crc64_name.clone(),
            size: manifest.size,
        };
        
        let mut decrypted_data = Vec::new();
        decode_asset_with_data(&crypto_data, &contents, &mut decrypted_data)?;
        
        let mut buf = Cursor::new(&decrypted_data[..]);
        let mut rev_map = HashMap::new();
        self.parse_transposed_array(&mut buf, &mut rev_map)?;
        self.resolve_all_deps(&rev_map);
        self.resolve_all_real_names();
        
        println!("Catalog initialized with {} entries", self.entries.len());
        
        Ok(())
    }
    
    fn parse_transposed_array(&mut self, buf: &mut Cursor<&[u8]>, rev_map: &mut HashMap<u64, usize>) -> Result<(), Box<dyn std::error::Error>> {
        
        let magic = buf.read_u16::<BigEndian>()?;
        if magic != 0xCA01 {
            return Err(format!("First 2 bytes are [0x{:X}], expect [0xCA01]", magic).into());
        }
        
        
        let _nothing = buf.read_u16::<BigEndian>()?;
        
        
        let size = read_uvarint(buf)? as usize;
        println!("Reading {} entries from manifest", size);
        
        self.entries = vec![Entry::default(); size];
        
        
        let mut read_uvarint_field = |field_name: &str| -> Result<(), Box<dyn std::error::Error>> {
            for i in 0..size {
                let value = read_uvarint(buf)?;
                match field_name {
                    "Priority" => self.entries[i].priority = value as u32,
                    "ResourceType" => self.entries[i].resource_type = value as u32,
                    "NumDeps" => self.entries[i].num_deps = value as u32,
                    "NumContents" => self.entries[i].num_contents = value as u32,
                    "NumCategories" => self.entries[i].num_categories = value as u32,
                    "Size" => self.entries[i].size = value,
                    _ => return Err(format!("Unknown field: {}", field_name).into()),
                }
            }
            Ok(())
        };
        
        read_uvarint_field("Priority")?;
        read_uvarint_field("ResourceType")?;
        read_uvarint_field("NumDeps")?;
        read_uvarint_field("NumContents")?;
        read_uvarint_field("NumCategories")?;
        read_uvarint_field("Size")?;
        
        
        for i in 0..size {
            let raw_bytes = read_until_null_byte(buf)?;
            let crc32 = update_crc32(0, &raw_bytes);
            self.entries[i].type_crc = crc32;
            self.entries[i].str_type_crc = String::from_utf8(raw_bytes)?;
        }
        
        
        for i in 0..size {
            let entry = &mut self.entries[i];
            for _ in 0..entry.num_contents {
                let raw_bytes = read_until_null_byte(buf)?;
                let crc32 = update_crc32(0, &raw_bytes);
                entry.content_type_crcs.extend_from_slice(&crc32.to_be_bytes());
                entry.str_content_type_crcs.push(String::from_utf8(raw_bytes)?);
            }
        }
        
        
        for i in 0..size {
            let entry = &mut self.entries[i];
            for _ in 0..entry.num_categories {
                let raw_bytes = read_until_null_byte(buf)?;
                let crc32 = update_crc32(0, &raw_bytes);
                entry.category_crcs.extend_from_slice(&crc32.to_be_bytes());
                entry.str_category_crcs.push(String::from_utf8(raw_bytes)?);
            }
        }
        
        
        for i in 0..size {
            let raw_bytes = read_until_null_byte(buf)?;
            let crc64 = update_crc64(0, &raw_bytes);
            self.entries[i].label_crc = crc64;
            self.entries[i].str_label_crc = String::from_utf8(raw_bytes)?;
            rev_map.insert(crc64, i);
        }
        
        
        for i in 0..size {
            let entry = &mut self.entries[i];
            for _ in 0..entry.num_contents {
                let raw_bytes = read_until_null_byte(buf)?;
                let crc64 = update_crc64(0, &raw_bytes);
                entry.content_name_crcs.extend_from_slice(&crc64.to_be_bytes());
                entry.str_content_name_crcs.push(String::from_utf8(raw_bytes)?);
            }
        }
        
        
        for i in 0..size {
            let entry = &mut self.entries[i];
            for _ in 0..entry.num_deps {
                let raw_bytes = read_until_null_byte(buf)?;
                let crc64 = update_crc64(0, &raw_bytes);
                entry.dep_crcs.extend_from_slice(&crc64.to_be_bytes());
                entry.str_dep_crcs.push(String::from_utf8(raw_bytes)?);
            }
        }
        
        
        for i in 0..size {
            let checksum = buf.read_u64::<BigEndian>()?;
            self.entries[i].checksum = checksum;
        }
        
        
        for i in 0..size {
            
            let pos = buf.position();
            let peek_byte = buf.read_u8()?;
            buf.set_position(pos);
            
            let seed = if peek_byte != 0 {
                buf.read_u64::<BigEndian>()?
            } else {
                buf.read_u8()? as u64
            };
            self.entries[i].seed = seed;
        }
        
        Ok(())
    }

    fn resolve_all_deps(&mut self, _rev_map: &HashMap<u64, usize>) {
        
        for entry in &mut self.entries {
            entry.rec_dep_crcs = Vec::new();
            entry.num_rec_dep_crcs = 0;
        }
    }

    fn resolve_all_real_names(&mut self) {
        for entry in &mut self.entries {
            entry.real_name = get_real_name(entry.checksum, entry.label_crc, entry.size);
        }
    }

    pub fn filter_db(&mut self) {
        self.entries.retain(|entry| entry.str_type_crc == "tsv");
    }

    /// 譜面ファイルのみにフィルタリング
    pub fn filter_chart(&mut self) {
        self.entries.retain(|entry| {
            entry.str_label_crc.starts_with("rhythmgame_chart")
        });
    }

    pub fn diff(&mut self, old_catalog: &Catalog) {
        let old_map: HashMap<u64, &Entry> = old_catalog.entries
            .iter()
            .map(|entry| (entry.label_crc, entry))
            .collect();
        
        let mut new_entries = Vec::new();
        
        for entry in &self.entries {
            if let Some(old_entry) = old_map.get(&entry.label_crc) {
                if entry.checksum == old_entry.checksum {
                    continue; 
                }
            }
            println!("Found a new or updated entry [{}].", entry.str_label_crc);
            new_entries.push(entry.clone());
        }
        
        self.entries = new_entries;
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            priority: 0,
            resource_type: 0,
            num_deps: 0,
            num_contents: 0,
            num_categories: 0,
            size: 0,
            content_type_crcs: Vec::new(),
            type_crc: 0,
            category_crcs: Vec::new(),
            label_crc: 0,
            content_name_crcs: Vec::new(),
            dep_crcs: Vec::new(),
            rec_dep_crcs: Vec::new(),
            num_rec_dep_crcs: 0,
            checksum: 0,
            seed: 0,
            str_type_crc: String::new(),
            str_content_type_crcs: Vec::new(),
            str_category_crcs: Vec::new(),
            str_label_crc: String::new(),
            str_content_name_crcs: Vec::new(),
            str_dep_crcs: Vec::new(),
            real_name: String::new(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub real_name: String,
    pub client_version: String,
    pub resource_version: String,
    pub simple_resver: String,
    pub checksum: u64,
    pub label_crc: u64,
    pub seed: u64,
    pub size: u64,
    pub calc_crc64_name: String,
}

impl Manifest {
    pub fn new() -> Self {
        Self {
            real_name: String::new(),
            client_version: String::new(),
            resource_version: String::new(),
            simple_resver: String::new(),
            checksum: 0,
            label_crc: 0,
            seed: 0,
            size: 0,
            calc_crc64_name: String::new(),
        }
    }

    
    pub fn init(&mut self, res_header: String, client_version: String) -> Result<(), Box<dyn std::error::Error>> {
        use base64::{Engine as _, engine::general_purpose};
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;
        
        self.resource_version = res_header.clone();
        self.client_version = client_version.clone();
        
        let parts: Vec<&str> = res_header.split('@').collect();
        if parts.len() != 2 {
            return Err(format!("resHeader {:?} cannot be splitted by `@`", res_header).into());
        }
        
        self.simple_resver = parts[0].to_string();
        let decoded = general_purpose::STANDARD.decode(parts[1])?;
        
        let mut cursor = Cursor::new(decoded);
        self.checksum = cursor.read_u64::<BigEndian>()?;
        self.seed = cursor.read_u64::<BigEndian>()?;
        
        
        self.size = read_uvarint(&mut cursor)?;
        
        
        self.label_crc = update_crc64(0, self.simple_resver.as_bytes());
        
        
        self.real_name = get_real_name(self.checksum, self.label_crc, self.size);
        
        self.calc_crc64_name = format!("{}:{}", self.client_version, self.simple_resver);
        
        println!("Initialize manifest completed.");
        println!("Manifest realname: {:?}.", self.real_name);
        
        Ok(())
    }
}