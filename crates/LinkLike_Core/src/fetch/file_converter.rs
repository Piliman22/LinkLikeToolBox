use crate::manifest::Catalog;
use crate::fetch::auto_update::AutoUpdater;
use std::fs;
use std::path::Path;

pub struct FileConverter<'a> {
    updater: &'a AutoUpdater,
}

impl<'a> FileConverter<'a> {
    pub fn new(updater: &'a AutoUpdater) -> Self {
        Self { updater }
    }

    pub async fn convert_db_files(&self, catalog: &Catalog) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.updater.db_save_dir)?;
        
        let mut error_count = 0;
        let mut processed_count = 0;
        
        for entry in &catalog.entries {
            if entry.str_type_crc != "tsv" {
                continue;
            }

            processed_count += 1;
            println!("Processing TSV file: {}", entry.str_label_crc);
            
            let src_path = format!("{}/{}", self.updater.decrypted_assets_save_dir, entry.str_label_crc);
            let dest_path = format!("{}/{}", self.updater.db_save_dir, entry.str_label_crc);
            
            if !Path::new(&src_path).exists() {
                eprintln!("Decrypted file not found: {}", src_path);
                error_count += 1;
                continue;
            }

            match fs::copy(&src_path, &dest_path) {
                Ok(_) => {
                    println!("Successfully copied TSV: {} -> {}", src_path, dest_path);
                }
                Err(e) => {
                    eprintln!("Failed to copy TSV {}: {}", src_path, e);
                    error_count += 1;
                }
            }
        }

        println!("Processed {} TSV files", processed_count);
        if error_count > 0 {
            eprintln!("{} Error(s) occurred during processing.", error_count);
        } else {
            println!("All databases processed successfully.");
        }

        Ok(())
    }

    pub async fn convert_chart_files(&self, catalog: &Catalog) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.updater.db_save_dir)?;
        
        let mut error_count = 0;
        let mut processed_count = 0;
        
        
        self.log_decrypted_directory_contents();
        
        for entry in &catalog.entries {
            if !entry.str_label_crc.starts_with("rhythmgame_chart") {
                continue;
            }

            processed_count += 1;
            println!("Processing chart file: {}", entry.str_label_crc);
            
            let src_path = format!("{}/{}", self.updater.decrypted_assets_save_dir, entry.str_label_crc);
            let dest_path = format!("{}/{}.json", self.updater.db_save_dir, entry.str_label_crc);
            
            if !Path::new(&src_path).exists() {
                eprintln!("Decrypted file not found: {}", src_path);
                self.find_similar_files(&entry.str_label_crc);
                error_count += 1;
                continue;
            }

            match self.process_chart_file(&src_path, &dest_path).await {
                Ok(_) => {
                    println!("Successfully processed chart: {} -> {}", src_path, dest_path);
                }
                Err(e) => {
                    eprintln!("Failed to process chart {}: {}", src_path, e);
                    error_count += 1;
                }
            }
        }

        println!("Processed {} chart files", processed_count);
        if error_count > 0 {
            eprintln!("{} Error(s) occurred during processing.", error_count);
        } else {
            println!("All chart files processed successfully.");
        }

        Ok(())
    }

    async fn process_chart_file(&self, src_path: &str, dest_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        use crate::crypt::chart::Chart;
        
        let chart = Chart::from_file(src_path)?;
        let decompressed_data = chart.decompress()?;
        fs::write(dest_path, decompressed_data)?;
        
        Ok(())
    }

    fn log_decrypted_directory_contents(&self) {
        println!("Checking contents of decrypted directory: {}", self.updater.decrypted_assets_save_dir);
        if let Ok(entries) = fs::read_dir(&self.updater.decrypted_assets_save_dir) {
            let decrypted_files: Vec<String> = entries
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| entry.file_name().into_string().ok())
                .collect();
            
            println!("Found {} files in decrypted directory", decrypted_files.len());
            
            let chart_files: Vec<&String> = decrypted_files
                .iter()
                .filter(|name| name.starts_with("rhythmgame_chart"))
                .collect();
            
            println!("Found {} chart files in decrypted directory:", chart_files.len());
            for (i, file) in chart_files.iter().take(5).enumerate() {
                println!("  [{}] {}", i + 1, file);
            }
            if chart_files.len() > 5 {
                println!("  ... and {} more", chart_files.len() - 5);
            }
        }
    }

    fn find_similar_files(&self, target: &str) {
        if let Ok(entries) = fs::read_dir(&self.updater.decrypted_assets_save_dir) {
            for dir_entry in entries.filter_map(|e| e.ok()) {
                let file_name = dir_entry.file_name().to_string_lossy().to_string();
                if file_name.contains(&target[..20]) {
                    eprintln!("  Similar file found: {}", file_name);
                }
            }
        }
    }
}