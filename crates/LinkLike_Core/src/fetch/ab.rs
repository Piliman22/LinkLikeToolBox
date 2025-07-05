// download asset bundle

use crate::fetch::downloader::{download_assets_async, download_manifest_sync};
use crate::manifest::Catalog;

/// Download manifest file
pub async fn download_manifest(real_name: &str, save_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    download_manifest_sync(real_name, save_dir).await
}

/// Download all assets from catalog
pub async fn download_assets(catalog: &Catalog, download_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    download_assets_async(catalog, download_dir).await
}