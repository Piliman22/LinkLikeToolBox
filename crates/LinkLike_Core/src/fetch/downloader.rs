use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use futures::stream::{self, StreamExt};
use reqwest::Client;
use tokio::sync::Semaphore;

use crate::fetch::header::asset_headers;
use crate::manifest::{Catalog, Entry};
use crate::url::ASSET_URL;

const MAX_CONCURRENCY: usize = 10;
const MAX_RETRIES: usize = 3;
const TIMEOUT_SECONDS: u64 = 1200;


#[derive(Debug)]
pub struct SafeCounter {
    value: AtomicUsize,
}

impl SafeCounter {
    pub fn new() -> Self {
        Self {
            value: AtomicUsize::new(0),
        }
    }

    pub fn increase(&self) {
        self.value.fetch_add(1, Ordering::SeqCst);
    }

    pub fn value(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }

    pub fn clear(&self) {
        self.value.store(0, Ordering::SeqCst);
    }
}


pub async fn download_manifest_sync(real_name: &str, save_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let counter = Arc::new(SafeCounter::new());
    let entry = Entry::new_manifest(real_name.to_string());

    fs::create_dir_all(save_dir)?;
    
    let client = create_client();
    let headers = asset_headers();
    
    download_one(&client, &entry, save_dir, &headers, &counter, 1).await?;
    println!("Manifest is successfully downloaded.");
    
    Ok(())
}


pub async fn download_assets_async(catalog: &Catalog, download_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENCY));
    let dl_amount = catalog.entries.len();
    let counter = Arc::new(SafeCounter::new());
    
    fs::create_dir_all(download_dir)?;
    
    let client = create_client();
    let headers = asset_headers();
    
    
    let download_tasks = stream::iter(&catalog.entries)
        .map(|entry| {
            let semaphore = Arc::clone(&semaphore);
            let counter = Arc::clone(&counter);
            let client = client.clone();
            let headers = headers.clone();
            let download_dir = download_dir.to_string();
            let entry = entry.clone();
            
            async move {
                let _permit = semaphore.acquire().await.unwrap();
                download_one(&client, &entry, &download_dir, &headers, &counter, dl_amount).await
            }
        })
        .buffer_unordered(MAX_CONCURRENCY);
    
    
    let results: Vec<Result<(), Box<dyn std::error::Error>>> = download_tasks.collect().await;
    
    
    for result in results {
        result?;
    }
    
    println!("Successfully downloaded all assets.");
    Ok(())
}


async fn download_one(
    client: &Client,
    entry: &Entry,
    save_dir: &str,
    headers: &HashMap<&'static str, &'static str>,
    counter: &SafeCounter,
    amount: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = prepare_url(entry);
    
    for attempt in 0..MAX_RETRIES {
        match execute_download(client, &url, headers, save_dir, entry).await {
            Ok(_) => {
                counter.increase();
                println!(
                    "({}/{}) Download completed: {:?}({})",
                    counter.value(),
                    amount,
                    entry.str_label_crc,
                    entry.real_name
                );
                return Ok(());
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                eprintln!(
                    "An error occurred when downloading {}, retrying...({}/{})",
                    url,
                    attempt + 1,
                    MAX_RETRIES
                );
                
                if attempt == MAX_RETRIES - 1 {
                    return Err(format!("Max retries exhausted when downloading {}", url).into());
                }
            }
        }
    }
    
    unreachable!()
}


async fn execute_download(
    client: &Client,
    url: &str,
    headers: &HashMap<&'static str, &'static str>,
    save_dir: &str,
    entry: &Entry,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut request_builder = client.get(url);
    
    for (key, value) in headers {
        request_builder = request_builder.header(*key, *value);
    }
    
    let response = request_builder.send().await?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP error: {} {}", response.status(), response.status().canonical_reason().unwrap_or("")).into());
    }
    
    
    let file_name = &entry.real_name;
    
    let file_path = Path::new(save_dir).join(file_name);
    let bytes = response.bytes().await?;
    
    let mut file = fs::File::create(file_path)?;
    file.write_all(&bytes)?;
    file.flush()?;
    
    Ok(())
}


fn prepare_url(entry: &Entry) -> String {
    let res_type = if entry.resource_type <= 1 {
        "android"
    } else {
        "raw"
    };
    
    
    let prefix = if entry.real_name.len() >= 2 {
        &entry.real_name[..2]
    } else {
        &entry.real_name
    };
    
    format!(
        "{}/{}/{}/{}",
        ASSET_URL,
        res_type,
        prefix,
        entry.real_name
    )
}


fn create_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(TIMEOUT_SECONDS))
        .build()
        .expect("Failed to create HTTP client")
}