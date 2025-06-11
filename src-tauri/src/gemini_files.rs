use std::{env, fs};

use reqwest::Client;
use serde_json::Value;

pub async fn upload_from_path(path: &str) -> String {
    let file = fs::read("ror2.xml").unwrap();
    let num_bytes = file.len();

    let client = Client::new();
    let res = client.post(
        format!("https://generativelanguage.googleapis.com/upload/v1beta/files?key={}",
            std::env::var("GEMINI_API_KEY").unwrap()
        ))
        .header("X-Goog-Upload-Protocol", "resumable")
        .header("X-Goog-Upload-Command", "start")
        .header("X-Goog-Upload-Header-Content-Length", num_bytes)
        .header("X-Goog-Upload-Header-Content-Type", "application/xml")
        .header("Content-Type", "application/json")
        .body("{'file': {'display_name': 'ror2.xml'}}")
        .send()
        .await
        .unwrap();

    let upload_uri = res
        .headers()
        .get("x-goog-upload-url")
        .unwrap()
        .to_str()
        .unwrap();

    let res = client
        .post(upload_uri)
        .header("Content-Length", num_bytes)
        .header("X-Goog-Upload-Offset", 0)
        .header("X-Goog-Upload-Command", "upload, finalize")
        .body(file)
        .send()
        .await
        .unwrap();

    let file_uri = serde_json::from_str::<Value>(&res.text().await.unwrap())
        .unwrap()
        ["file"]["uri"]
        .take()
        .to_string();

    let res = client
        .get(format!("https://generativelanguage.googleapis.com/v1beta/files?key={}", env::var("GEMINI_API_KEY").unwrap()))
        .send()
        .await
        .unwrap();

    println!("{:?}", res);
    file_uri
}
