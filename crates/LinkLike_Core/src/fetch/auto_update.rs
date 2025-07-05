use crate::fetch::{downloader, login, playversion};
use crate::manifest::{Catalog, Entry, Manifest};
use std::fs::{self, File};
use std::path::Path;

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

    // 設定を確認し、必要に応じて調整
    pub fn with_custom_paths(
        assets_save_dir: String,
        db_save_dir: String,
        keep_raw: bool,
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
        // updated flagファイルを削除
        let _ = fs::remove_file(&self.updated_flag_file);

        // クライアントバージョンを取得（非同期に変更）
        let client_version = playversion::get_play_version("com.oddno.lovelive").await?;
        println!("Client version: {}", client_version);

        // ログインしてリソース情報を取得（非同期に変更）
        let res_info = login::login(&client_version).await?;
        println!("Resource version: {}", res_info);

        // 現在のバージョンをチェック
        let current_version = self.read_current_version()?;
        
        if !options.force && res_info == current_version {
            println!("Nothing updated, stopping process.");
            return Ok(UpdateResult::NoUpdate);
        }

        println!("New resource version: {}", res_info);

        // マニフェストを初期化
        let mut manifest = Manifest::new();
        manifest.init(res_info.clone(), client_version);

        // カタログをダウンロード
        self.download_catalog(&manifest).await?;

        // カタログファイルを初期化
        let catalog = self.init_catalog(&manifest).await?;

        // 古いカタログとの差分を取得
        let mut filtered_catalog = if options.force {
            catalog
        } else {
            self.process_catalog_diff(catalog).await?
        };

        // DB専用フィルター
        if options.db_only {
            filtered_catalog.filter_db();
        }

        if filtered_catalog.entries.is_empty() {
            println!("Nothing is updated, stopping process.");
            return Ok(UpdateResult::NoUpdate);
        }

        // アセットダウンロード
        downloader::download_assets_async(&filtered_catalog, &self.assets_save_dir).await?;
        println!("Assets downloaded to: {}", self.assets_save_dir);

        // アセット復号化（必要に応じて実装）
        self.decrypt_all_assets(&filtered_catalog).await?;

        // DBファイルの変換（修正）
        if options.db_only {
            self.convert_db_files(&filtered_catalog).await?;
            println!("TSV files converted to: {}", self.db_save_dir);
        }

        // バージョンファイルを更新
        fs::write(&self.catalog_version_file, &res_info)?;

        // updated flagを作成
        fs::write(&self.updated_flag_file, "")?;

        // 生ファイルの削除（keep_rawがfalseの場合のみ）
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
        
        // ファイルを削除
        fs::remove_file(catalog_file_path)?;
        
        Ok(catalog)
    }

    async fn process_catalog_diff(&self, mut catalog: Catalog) -> Result<Catalog, Box<dyn std::error::Error>> {
        // 古いカタログファイルをリネーム
        if Path::new(&self.catalog_json_file).exists() {
            fs::rename(&self.catalog_json_file, &self.catalog_json_file_prev)?;
            println!("Outdated catalog was renamed to '{}'.", self.catalog_json_file_prev);
        }

        // 新しいカタログをJSONファイルに保存
        self.write_to_json_file(&catalog.entries, &self.catalog_json_file)?;

        // 古いカタログを読み込み
        let old_entries = self.read_from_json_file::<Vec<Entry>>(&self.catalog_json_file_prev)
            .unwrap_or_else(|_| Vec::new());

        let old_catalog = Catalog { entries: old_entries };

        // 差分を取得
        catalog.diff(&old_catalog);

        // 差分をファイルに保存
        self.write_to_json_file(&catalog.entries, &self.catalog_json_diff_file)?;

        Ok(catalog)
    }

    // 復号化処理は不要（manifest.rsで処理）
    async fn decrypt_all_assets(&self, _catalog: &Catalog) -> Result<(), Box<dyn std::error::Error>> {
        println!("Decrypting assets...");
        // 実際の復号化はconvert_to_json内で行う
        Ok(())
    }

    /// DBファイルの変換処理
    async fn convert_db_files(&self, catalog: &Catalog) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.db_save_dir)?;
        
        let mut error_count = 0;
        let mut processed_count = 0;
        
        for entry in &catalog.entries {
            if entry.str_type_crc != "tsv" {
                continue;
            }

            processed_count += 1;
            println!("Processing TSV file: {}", entry.str_label_crc);
            
            // TSVファイルはstr_label_crcがファイル名として保存されている
            let src_path = format!("{}/{}.tsv", self.assets_save_dir, entry.str_label_crc);
            
            // 保存先のパス（.tsvとして保存）
            let dest_path = format!("{}/{}", self.db_save_dir, entry.str_label_crc);
            
            // ファイルが存在するかチェック
            if !Path::new(&src_path).exists() {
                eprintln!("Source file not found: {}", src_path);
                error_count += 1;
                continue;
            }

            // ファイルをコピー（復号化が必要な場合は後で実装）
            match fs::copy(&src_path, &dest_path) {
                Ok(_) => {
                    println!("Successfully copied: {} -> {}", src_path, dest_path);
                }
                Err(e) => {
                    eprintln!("Failed to copy {}: {}", src_path, e);
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

    fn read_current_version(&self) -> Result<String, Box<dyn std::error::Error>> {
        match fs::read_to_string(&self.catalog_version_file) {
            Ok(version) => Ok(version),
            Err(_) => Ok(String::new()),
        }
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

#[derive(Debug)]
pub struct UpdateOptions {
    pub force: bool,
    pub db_only: bool,
    pub keep_raw: bool,
    pub analyze: bool,
}

impl Default for UpdateOptions {
    fn default() -> Self {
        Self {
            force: false,
            db_only: false,
            keep_raw: false,
            analyze: false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UpdateResult {
    Updated,
    NoUpdate,
    AnalysisComplete,
}