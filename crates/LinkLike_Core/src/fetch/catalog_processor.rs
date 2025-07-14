use crate::manifest::{Catalog, Entry};
use crate::fetch::auto_update::AutoUpdater;
use std::fs;
use std::path::Path;

pub struct CatalogProcessor<'a> {
    updater: &'a AutoUpdater,
}

impl<'a> CatalogProcessor<'a> {
    pub fn new(updater: &'a AutoUpdater) -> Self {
        Self { updater }
    }

    pub async fn process_catalog_diff(&self, mut catalog: Catalog) -> Result<Catalog, Box<dyn std::error::Error>> {
        
        if Path::new(&self.updater.catalog_json_file).exists() {
            fs::rename(&self.updater.catalog_json_file, &self.updater.catalog_json_file_prev)?;
            println!("Outdated catalog was renamed to '{}'.", self.updater.catalog_json_file_prev);
        }

        
        self.write_to_json_file(&catalog.entries, &self.updater.catalog_json_file)?;

        
        let old_entries = self.read_from_json_file::<Vec<Entry>>(&self.updater.catalog_json_file_prev)
            .unwrap_or_else(|_| Vec::new());

        let old_catalog = Catalog { entries: old_entries };

        
        catalog.diff(&old_catalog);

        
        self.write_to_json_file(&catalog.entries, &self.updater.catalog_json_diff_file)?;

        Ok(catalog)
    }

    fn write_to_json_file<T: serde::Serialize>(&self, data: &T, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(data)?;
        fs::write(path, json)?;
        Ok(())
    }

    fn read_from_json_file<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let data = serde_json::from_str(&content)?;
        Ok(data)
    }
}