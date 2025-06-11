use std::fs;

use gemini_rs::types::{Content, FileData, InlineData, Part};
use serde_json::Value;
use tauri::Manager;
use tauri_plugin_positioner::{WindowExt, Position};

mod gemini_files;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn prompt_gemini(chat_history: &str) -> Result<String, String> {
    let file_uri = gemini_files::upload_from_path("").await;

    let chat_history_parsed: Vec<serde_json::Value> = serde_json::from_str(chat_history).unwrap();

    let mut history: Vec<Content> = chat_history_parsed
        .iter()
        .map(|v| {
            Content {
                role: if v.get("role").and_then(Value::as_str) == Some("user") {
                    gemini_rs::types::Role::User
                } else {
                    gemini_rs::types::Role::Model
                },
                parts: vec![Part {
                    text: Some(v.get("content").and_then(Value::as_str).unwrap_or("").to_string()),
                    ..Default::default()
                }]
            }
        })
        .collect();
    let prompt = history.pop().unwrap().parts[0].text.clone().unwrap();

    history.insert(0,
        Content {
            role: gemini_rs::types::Role::User,
            parts: vec![Part {
                file_data: Some(FileData {
                    file_uri,
                    mime_type: "application/xml".to_string()
                }),
                ..Default::default()
            }],
        }
    );
    let mut chat = gemini_rs::chat("gemini-2.0-flash");
    // chat.system_instruction("You are Guppy, the Gup digital assistant.")
    *chat.history_mut() = history;
    Ok(
        chat
            .system_instruction("You are Guppy, the Gup digital assistant.")
            .send_message(&prompt)
            .await
            .unwrap()
            .to_string()
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(desktop)]
            {
                let win = app.get_webview_window("main").unwrap();
                let _ = win.as_ref().window().move_window(Position::BottomRight);
            }
            // notifs dont work
            use tauri_plugin_notification::NotificationExt;
            app.notification()
                .builder()
                .title("Tauri")
                .body("Tauri is awesome")
                .show()
                .unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, prompt_gemini])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
