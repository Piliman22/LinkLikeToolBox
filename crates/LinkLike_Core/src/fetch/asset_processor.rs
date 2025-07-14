use crate::manifest::Catalog;
use crate::crypt::asset_decoder::AssetProcessor as CryptAssetProcessor;
use crate::fetch::auto_update::AutoUpdater;
use std::fs;
use std::path::Path;

pub struct AssetProcessor<'a> {
    updater: &'a AutoUpdater,
}

impl<'a> AssetProcessor<'a> {
    pub fn new(updater: &'a AutoUpdater) -> Self {
        Self { updater }
    }

    pub async fn decrypt_all_assets(&self, catalog: &Catalog) -> Result<(), Box<dyn std::error::Error>> {
        println!("Decrypting assets ...");
        
        
        self.log_assets_directory_contents();
        
        
        if !Path::new(&self.updater.assets_save_dir).exists() {
            return Err(format!("Assets directory does not exist: {}", self.updater.assets_save_dir).into());
        }
        
        
        fs::create_dir_all(&self.updater.decrypted_assets_save_dir)?;
        
        
        self.check_file_existence(catalog);
        
        
        println!("Starting decryption process...");
        CryptAssetProcessor::decrypt_all_assets(
            catalog,
            &self.updater.decrypted_assets_save_dir,
            &self.updater.assets_save_dir,
        )?;
        
        
        self.verify_decryption_results(catalog);
        
        Ok(())
    }

    fn log_assets_directory_contents(&self) {
        println!("Checking assets directory contents: {}", self.updater.assets_save_dir);
        if let Ok(entries) = fs::read_dir(&self.updater.assets_save_dir) {
            let downloaded_files: Vec<String> = entries
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| entry.file_name().into_string().ok())
                .collect();
            
            println!("Found {} downloaded files in assets directory", downloaded_files.len());
            for (i, file) in downloaded_files.iter().take(5).enumerate() {
                println!("  [{}] {}", i + 1, file);
            }
            if downloaded_files.len() > 5 {
                println!("  ... and {} more", downloaded_files.len() - 5);
            }
        }
    }

    fn check_file_existence(&self, catalog: &Catalog) {
        let mut missing_files = Vec::new();
        let mut existing_files = Vec::new();
        
        println!("Checking file existence for {} catalog entries...", catalog.entries.len());
        for (i, entry) in catalog.entries.iter().enumerate() {
            let file_path = format!("{}/{}", self.updater.assets_save_dir, entry.real_name);
            if !Path::new(&file_path).exists() {
                missing_files.push(entry.real_name.clone());
                if i < 5 {
                    println!("  Missing: {} (looking for: {})", entry.real_name, file_path);
                }
            } else {
                existing_files.push(entry.real_name.clone());
                if i < 5 {
                    println!("  Found: {} -> will decrypt to: {}", entry.real_name, entry.str_label_crc);
                }
            }
        }
        
        println!("Found {} existing files, {} missing files", existing_files.len(), missing_files.len());
        
        if !missing_files.is_empty() {
            eprintln!("Warning: {} files are missing from assets directory:", missing_files.len());
            for file in missing_files.iter().take(10) {
                eprintln!("  Missing: {}", file);
            }
            if missing_files.len() > 10 {
                eprintln!("  ... and {} more", missing_files.len() - 10);
            }
        }
    }

    fn verify_decryption_results(&self, catalog: &Catalog) {
        println!("Checking decryption results...");
        let mut decrypted_count = 0;
        for entry in &catalog.entries {
            let decrypted_path = format!("{}/{}", self.updater.decrypted_assets_save_dir, entry.str_label_crc);
            if Path::new(&decrypted_path).exists() {
                decrypted_count += 1;
            }
        }
        println!("Successfully decrypted {} files to {}", decrypted_count, self.updater.decrypted_assets_save_dir);
    }
}