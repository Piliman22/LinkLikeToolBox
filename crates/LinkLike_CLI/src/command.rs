use crate::color::Output;
use crate::progress::Progress;
use LinkLike_Core::AssetBundle;
use std::path::Path;

pub struct Commands {
    output: Output,
}

impl Commands {
    pub fn new() -> Self {
        Self {
            output: Output::new(),
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
}