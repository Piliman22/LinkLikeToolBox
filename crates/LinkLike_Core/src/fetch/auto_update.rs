use crate::fetch::{downloader, login, playversion};
use crate::manifest::{Catalog, Manifest};
use std::fs::{self, File};

use crate::fetch::catalog_processor::CatalogProcessor;
use crate::fetch::asset_processor::AssetProcessor as AutoAssetProcessor;
use crate::fetch::file_converter::FileConverter;

pub use crate::fetch::types::{UpdateOptions, UpdateResult};

pub struct AutoUpdater {
    pub manifest_save_dir: String,
    pub assets_save_dir: String,
    pub decrypted_assets_save_dir: String,
    pub db_save_dir: String,
    pub catalog_version_file: String,
    pub catalog_json_file: String,
    pub catalog_json_file_prev: String,
    pub catalog_json_diff_file: String,
    pub updated_flag_file: String,
}

impl Default for AutoUpdater {
    fn default() -> Self {
        Self {
            manifest_save_dir: "cache".to_string(),
            assets_save_dir: "cache/assets".to_string(),
            decrypted_assets_save_dir: "cache/plain".to_string(),
            db_save_dir: "masterdata".to_string(),
            catalog_version_file: "cache/currentVersion.txt".to_string(),
            catalog_json_file: "cache/catalog.json".to_string(),
            catalog_json_file_prev: "cache/catalog_prev.json".to_string(),
            catalog_json_diff_file: "cache/catalog_diff.json".to_string(),
            updated_flag_file: "cache/updated".to_string(),
        }
    }
}

impl AutoUpdater {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_custom_paths(
        assets_save_dir: String,
        db_save_dir: String,
        _keep_raw: bool,
    ) -> Self {
        Self {
            manifest_save_dir: "cache".to_string(),
            assets_save_dir,
            decrypted_assets_save_dir: "cache/plain".to_string(),
            db_save_dir,
            catalog_version_file: "cache/currentVersion.txt".to_string(),
            catalog_json_file: "cache/catalog.json".to_string(),
            catalog_json_file_prev: "cache/catalog_prev.json".to_string(),
            catalog_json_diff_file: "cache/catalog_diff.json".to_string(),
            updated_flag_file: "cache/updated".to_string(),
        }
    }

    
    pub async fn auto_update(&self, options: UpdateOptions) -> Result<UpdateResult, Box<dyn std::error::Error>> {
        let _ = fs::remove_file(&self.updated_flag_file);

        
        let client_version = playversion::get_play_version("com.oddno.lovelive").await?;
        println!("Client version: {}", client_version);

        
        let res_info = login::login(&client_version).await?;
        println!("Resource version: {}", res_info);

        
        let current_version = self.read_current_version()?;
        
        if !options.force && res_info == current_version {
            println!("Nothing updated, stopping process.");
            return Ok(UpdateResult::NoUpdate);
        }

        println!("New resource version: {}", res_info);

        
        let mut manifest = Manifest::new();
        manifest.init(res_info.clone(), client_version)?; 

        
        self.download_catalog(&manifest).await?;

        
        let catalog = self.init_catalog(&manifest).await?;

        
        let catalog_processor = CatalogProcessor::new(self);
        let mut filtered_catalog = if options.force {
            catalog
        } else {
            catalog_processor.process_catalog_diff(catalog).await?
        };

        
        if options.db_only {
            filtered_catalog.filter_db();
        } else if options.chart_only {
            filtered_catalog.filter_chart();
        }

        if filtered_catalog.entries.is_empty() {
            println!("Nothing is updated, stopping process.");
            return Ok(UpdateResult::NoUpdate);
        }

        
        downloader::download_assets_async(&filtered_catalog, &self.assets_save_dir).await?;
        println!("Assets downloaded to: {}", self.assets_save_dir);

        
        let asset_processor = AutoAssetProcessor::new(self);
        asset_processor.decrypt_all_assets(&filtered_catalog).await?;

        
        let file_converter = FileConverter::new(self);
        if options.chart_only {
            file_converter.convert_chart_files(&filtered_catalog).await?;
            println!("Chart files converted to: {}", self.db_save_dir);
        }

        if options.db_only {
            file_converter.convert_db_files(&filtered_catalog).await?;
            self.convert_tsv_to_json(&filtered_catalog).await?;
            println!("TSV files converted to: {}", self.db_save_dir);
        }

        
        fs::write(&self.catalog_version_file, &res_info)?;

        
        fs::write(&self.updated_flag_file, "")?;

        
        if !options.keep_raw {
            println!("Cleaning up raw files from: {}", self.assets_save_dir);
            let _ = fs::remove_dir_all(&self.assets_save_dir);
        } else {
            println!("Raw files preserved in: {}", self.assets_save_dir);
        }

        println!("Update completed successfully!");
        Ok(UpdateResult::Updated)
    }

    
    async fn download_catalog(&self, manifest: &Manifest) -> Result<(), Box<dyn std::error::Error>> {
        println!("Downloading catalog...");
        downloader::download_manifest_sync(&manifest.real_name, &self.manifest_save_dir).await?;
        Ok(())
    }

    
    async fn init_catalog(&self, manifest: &Manifest) -> Result<Catalog, Box<dyn std::error::Error>> {
        let catalog_file_path = format!("{}/{}", self.manifest_save_dir, manifest.real_name);
        let catalog_file = File::open(&catalog_file_path)?;
        
        let mut catalog = Catalog::new();
        catalog.init(manifest, catalog_file)?;
        
        
        fs::remove_file(catalog_file_path)?;
        
        Ok(catalog)
    }

    
    fn read_current_version(&self) -> Result<String, Box<dyn std::error::Error>> {
        match fs::read_to_string(&self.catalog_version_file) {
            Ok(version) => Ok(version),
            Err(_) => Ok(String::new()),
        }
    }

    async fn convert_tsv_to_json(&self, catalog: &Catalog) -> Result<(), Box<dyn std::error::Error>> {
        use crate::master::parse::{parse_tsv_from_bytes, convert_tsv_to_yaml_file};
        use std::path::Path;

        println!("Converting TSV files to YAML...");
        
        let mut converted_count = 0;
        let mut error_count = 0;
        
        for entry in &catalog.entries {
            if entry.str_type_crc != "tsv" {
                continue;
            }

            let tsv_path = format!("{}/{}", self.db_save_dir, entry.str_label_crc);
            let yaml_path = format!("{}/{}.yaml", self.db_save_dir, entry.str_label_crc);
            
            if !Path::new(&tsv_path).exists() {
                eprintln!("TSV file not found: {}", tsv_path);
                error_count += 1;
                continue;
            }

            match convert_tsv_to_yaml_file(&tsv_path, &yaml_path, &entry.str_label_crc) {
                Ok(_) => {
                    converted_count += 1;
                    println!("Converted: {} -> {}", tsv_path, yaml_path);
                }
                Err(e) => {
                    eprintln!("Failed to convert {}: {}", tsv_path, e);
                    error_count += 1;
                }
            }
        }

        println!("TSV to YAML conversion completed: {} converted, {} errors", converted_count, error_count);
        Ok(())
    }

    async fn convert_single_tsv_to_json(&self, tsv_path: &str, json_path: &str, label: &str) -> Result<(), Box<dyn std::error::Error>> {
        use crate::master::parse::{parse_tsv_from_bytes, get_json_type_name};
        
        let tsv_data = fs::read(tsv_path)?;
        let records = parse_tsv_from_bytes(&tsv_data)?;
        
        let type_name = get_json_type_name(label);
        let json_output = serde_json::json!({
            "type": type_name,
            "data": records
        });
        
        let json_string = serde_json::to_string_pretty(&json_output)?;
        fs::write(json_path, json_string)?;
        
        Ok(())
    }
}