use crate::color::Output;
use crate::progress::Progress;
use LinkLike_Core::{AssetBundle, Chart};
use LinkLike_Core::fetch::auto_update::{AutoUpdater, UpdateOptions, UpdateResult};
use std::path::Path;
use std::fs;
use crate::Banner;

pub struct Commands {
    output: Output,
    banner: Banner,
}

impl Commands {
    pub fn new() -> Self {
        Self {
            output: Output::new(),
            banner: Banner::new(),
        }
    }

    pub fn decrypt_ab(&self, file_path: &str) -> std::io::Result<()> {
        
        if !Path::new(file_path).exists() {
            self.output.print_error("file is not found");
            return Ok(());
        }

        let spinner = Progress::new_spinner("Decrypting AssetBundle...");
        
        let mut bundle = AssetBundle::from_file(file_path)?;
        bundle.decrypt()?;
        
        let output_path = format!("{}.decrypted", file_path);
        bundle.save_to_file(&output_path)?;

        spinner.finish_with_message("Completed decryption");
        self.output.print_success(&format!("Files: {}", output_path));
        
        Ok(())
    }

    pub fn decrypt_ab_folder(&self, folder_path: &str) -> std::io::Result<()> {
        if !Path::new(folder_path).is_dir() {
            self.output.print_error("this is not a folder");
            return Ok(());
        }

        let spinner = Progress::new_spinner("Decrypting AssetBundle folder...");
        
        let bundle = AssetBundle::from_folder(folder_path)?;
        let mut decrypted_bundle = bundle;
        decrypted_bundle.decrypt()?;
        
        let output_path = format!("{}_decrypted", folder_path);
        std::fs::create_dir_all(&output_path)?;
        decrypted_bundle.save_to_file(&output_path)?;

        spinner.finish_with_message("completed decryption");
        self.output.print_success(&format!("output: {}", output_path));
        
        Ok(())
    }

    pub fn crypt_ab(&self, file_path: &str) -> std::io::Result<()> {
        
        if !Path::new(file_path).exists() {
            self.output.print_error("File is not found");
            return Ok(());
        }

        let spinner = Progress::new_spinner("Encrypting AssetBundle...");
        
        let mut bundle = AssetBundle::from_file(file_path)?;
        bundle.crypt()?;
        
        let output_path = format!("{}.encrypted", file_path);
        bundle.save_to_file(&output_path)?;

        spinner.finish_with_message("completed encryption");
        self.output.print_success(&format!("files: {}", output_path));
        
        Ok(())
    }

    pub fn crypt_ab_folder(&self, folder_path: &str) -> std::io::Result<()> {
        if !Path::new(folder_path).is_dir() {
            self.output.print_error("This is not a folder");
            return Ok(());
        }

        let spinner = Progress::new_spinner("Encrypting AssetBundle folder...");
        
        let bundle = AssetBundle::from_folder(folder_path)?;
        let mut encrypted_bundle = bundle;
        encrypted_bundle.crypt()?;
        
        let output_path = format!("{}_encrypted", folder_path);
        std::fs::create_dir_all(&output_path)?;
        encrypted_bundle.save_to_file(&output_path)?;

        spinner.finish_with_message("completed encryption");
        self.output.print_success(&format!("output: {}", output_path));
        
        Ok(())
    }    

    pub fn decompress_chart(&self, file_path: &str) -> std::io::Result<()> {
        if !Path::new(file_path).exists() {
            self.output.print_error("file is not found");
            return Ok(());
        }

        let spinner = Progress::new_spinner("unzip chart file...");
        
        let chart = Chart::from_file(file_path)?;
        
        let decompressed_data = chart.get_decompressed_data()?;
        
        let output_path = format!("{}.json", file_path);
        
        fs::write(&output_path, &decompressed_data)?;

        spinner.finish_with_message("complete");
        self.output.print_success(&format!("file: {}", output_path));
        
        Ok(())
    }

    pub fn compress_chart(&self, file_path: &str, level: u32) -> std::io::Result<()> {
        if !Path::new(file_path).exists() {
            self.output.print_error("file is not found");
            return Ok(());
        }

        let spinner = Progress::new_spinner("compless chart file...");
        
        let data = fs::read(file_path)?;
        
        let chart = Chart::from_uncompressed(&data, level)?;
        
        let output_path = format!("{}.bytes", file_path);
        
        chart.save_to_file(&output_path)?;

        spinner.finish_with_message("complete");
        self.output.print_success(&format!("file: {}", output_path));
        
        Ok(())
    }

    pub async fn download_manifest(&self, real_name: &str, save_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        
        if !Path::new(save_dir).exists() {
            fs::create_dir_all(save_dir)?;
        }

        let spinner = Progress::new_spinner("Downloading manifest...");
        
        LinkLike_Core::fetch::ab::download_manifest(real_name, save_dir).await?;
        
        spinner.finish_with_message("Manifest download completed");
        self.output.print_success(&format!("Manifest saved to: {}", save_dir));
        
        Ok(())
    }

    pub async fn download_assets(&self, catalog_path: &str, download_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        
        if !Path::new(catalog_path).exists() {
            self.output.print_error("Catalog file not found");
            return Ok(());
        }

        
        if !Path::new(download_dir).exists() {
            fs::create_dir_all(download_dir)?;
        }

        let spinner = Progress::new_spinner("Loading catalog...");
        
        
        let catalog_data = fs::read_to_string(catalog_path)?;
        let catalog: LinkLike_Core::manifest::Catalog = serde_json::from_str(&catalog_data)?;
        
        spinner.finish_with_message("Catalog loaded");
        
        let progress = Progress::new_progress_bar(catalog.entries.len() as u64);
        
        LinkLike_Core::fetch::ab::download_assets(&catalog, download_dir).await?;
        
        progress.finish_with_message("All assets downloaded successfully");
        self.output.print_success(&format!("Assets saved to: {}", download_dir));
        
        Ok(())
    }

    pub async fn execute_download_command(&self, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() < 3 {
            self.output.print_error("Usage: download <manifest|assets|auto> ...");
            return Ok(());
        }

        match args[2].as_str() {
            "manifest" => {
                if args.len() < 5 {
                    self.output.print_error("Usage: download manifest <real_name> <save_dir>");
                    return Ok(());
                }
                self.download_manifest(&args[3], &args[4]).await?;
            }
            "assets" => {
                if args.len() < 5 {
                    self.output.print_error("Usage: download assets <catalog_path> <download_dir>");
                    return Ok(());
                }
                self.download_assets(&args[3], &args[4]).await?;
            }
            "auto" => {
                self.auto_update(args).await?;
            }
            _ => {
                self.output.print_error("Unknown download command. Use 'manifest', 'assets', or 'auto'.");
            }
        }

        Ok(())
    }

    pub async fn auto_update(&self, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let mut options = UpdateOptions::default();
        
        
        for arg in args.iter().skip(3) {
            match arg.as_str() {
                "--force" => options.force = true,
                "--db-only" => options.db_only = true,
                "--chart-only" => options.chart_only = true,
                "--keep-raw" => options.keep_raw = true,
                "--analyze" => options.analyze = true,
                _ => {}
            }
        }

        if options.analyze {
            self.output.print_info("Start analyzing code...");
            
            self.output.print_success("Analysis completed.");
            return Ok(());
        }

        let updater = AutoUpdater::new();
        let spinner = Progress::new_spinner("Starting auto update...");
        
        match updater.auto_update(options).await? {
            UpdateResult::Updated => {
                spinner.finish_with_message("Auto update completed successfully");
                self.output.print_success("All resources have been updated!");
            }
            UpdateResult::NoUpdate => {
                spinner.finish_with_message("No updates available");
                self.output.print_info("Everything is up to date.");
            }
            UpdateResult::AnalysisComplete => {
                spinner.finish_with_message("Analysis completed");
                self.output.print_success("Code analysis finished.");
            }
        }

        Ok(())
    }

    pub fn execute(&self, args: &[String]) -> std::io::Result<()> {
        if args.len() < 2 {
            self.banner.print_banner();
            self.banner.print_summary();
            return Ok(());
        }

        match args[1].as_str() {
            "decrypt" => {
                if args.len() < 3 {
                    self.banner.print_summary();
                    return Ok(());
                }
                
                match args[2].as_str() {
                    "ab" => {
                        if args.len() < 4 {
                            self.banner.print_summary();
                            return Ok(());
                        }
                        self.decrypt_ab(&args[3])?;
                    },
                    "chart" => {
                        if args.len() < 4 {
                            self.banner.print_summary();
                            return Ok(());
                        }
                        self.decompress_chart(&args[3])?;
                    },
                    _ => {
                        self.banner.print_summary();
                    }
                }
            },
            "crypt" => {
                if args.len() < 3 {
                    self.banner.print_summary();
                    return Ok(());
                }
                
                match args[2].as_str() {
                    "ab" => {
                        if args.len() < 4 {
                            self.banner.print_summary();
                            return Ok(());
                        }
                        self.crypt_ab(&args[3])?;
                    },
                    "chart" => {
                        if args.len() < 4 {
                            self.banner.print_summary();
                            return Ok(());
                        }
                        
                        let level = if args.len() >= 5 {
                            args[4].parse::<u32>().unwrap_or(6)
                        } else {
                            6
                        };
                        
                        self.compress_chart(&args[3], level)?;
                    },
                    _ => {
                        self.banner.print_summary();
                    }
                }
            },
            "help" => {
                self.banner.print_banner();
                self.banner.print_summary();
            },
            _ => {
                self.banner.print_banner();
                self.banner.print_summary();
            }
        }

        Ok(())
    }
}