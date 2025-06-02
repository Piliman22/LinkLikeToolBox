use crate::color::Output;
use crate::progress::Progress;
use LinkLike_Core::{AssetBundle, Chart};
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
        // check if the file exists
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
        // check if the file exists
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
            _ => {
                self.banner.print_banner();
                self.banner.print_summary();
            }
        }

        Ok(())
    }
}