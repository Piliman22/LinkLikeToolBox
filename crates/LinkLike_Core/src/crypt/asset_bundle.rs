// LinkLike Asset Bundle

use std::io::{self, Read, Write};

pub struct AssetBundle {
    data: Vec<u8>,
}

impl AssetBundle {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    // decrypts the asset bundle LinkLike format
    pub fn decrypt(&mut self) -> io::Result<()> {
        if self.data.len() >= 2 && self.data[0] == 0xAB && self.data[1] == 0x00 {
            self.data = self.data[2..].to_vec();
            Ok(())
        } else {
            Ok(())
        }
    }

    // encrypts the asset bundle LinkLike format
    pub fn crypt(&mut self) -> io::Result<()> {
        let mut head = vec![0xAB, 0x00];
        head.extend(&self.data);
        self.data = head;
        Ok(())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> io::Result<Self> {
        let mut file = std::fs::File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Ok(Self::new(data))
    }

    pub fn from_folder<P: AsRef<std::path::Path>>(path: P) -> io::Result<Self> {
        let mut data = Vec::new();
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let mut file = std::fs::File::open(entry.path())?;
                let mut file_data = Vec::new();
                file.read_to_end(&mut file_data)?;
                data.extend(file_data);
            }
        }
        Ok(Self::new(data))
    }

    pub fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        file.write_all(&self.data)?;
        Ok(())
    }
}

// test the AssetBundle
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt() {
        let data = vec![0xAB, 0x00, 0x01, 0x02, 0x03];
        let mut bundle = AssetBundle::new(data);
        bundle.decrypt().unwrap();
        assert_eq!(bundle.get_data(), &[0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_crypt() {
        let data = vec![0x01, 0x02, 0x03];
        let mut bundle = AssetBundle::new(data);
        bundle.crypt().unwrap();
        assert_eq!(bundle.get_data(), &[0xAB, 0x00, 0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_decrypt_without_header() {
        let data = vec![0x01, 0x02, 0x03];
        let mut bundle = AssetBundle::new(data.clone());
        bundle.decrypt().unwrap();
        assert_eq!(bundle.get_data(), &data);
    }
}