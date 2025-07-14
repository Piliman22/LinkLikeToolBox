use std::collections::HashMap;

// Login headers for the LinkLike API (Goコードと完全に同じ)
pub fn login_headers() -> HashMap<&'static str, &'static str> {
    let mut headers = HashMap::new();
    headers.insert("X-Idempotency-Key", "eb6afd7c69cd9a87ca1fb167b21ae95c");
    headers.insert("X-Client-Version", "1.10.40");
    headers.insert("User-Agent", "inspix-android/1.10.40");
    headers.insert("x-res-version", "R2402010");
    headers.insert("x-device-type", "android");
    headers.insert("inspix-user-api-version", "1.0.0");
    headers.insert("Accept", "application/json");
    headers.insert("x-api-key", "4e769efa67d8f54be0b67e8f70ccb23d513a3c841191b6b2ba45ffc6fb498068");
    headers.insert("Content-Type", "application/json");
    headers.insert("Accept-Encoding", "gzip, deflate");
    headers
}

// asset headers
pub fn asset_headers() -> HashMap<&'static str, &'static str> {
    let mut headers = HashMap::new();
    headers.insert("User-Agent", "UnityPlayer/2021.3.16f1 (UnityWebRequest/1.0, libcurl/7.84.0-DEV)");
    headers.insert("Accept", "*/*");
    headers.insert("Accept-Encoding", "deflate, gzip");
    headers.insert("X-Unity-Version", "2021.3.16f1");
    headers
}