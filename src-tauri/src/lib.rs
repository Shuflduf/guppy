use tauri::Manager;
use tauri_plugin_positioner::{WindowExt, Position};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn prompt_gemini(prompt: &str) -> Result<String, String> {
    println!("Prompting Gemini with: {}", prompt);
    Ok(
    gemini_rs::chat("gemini-2.0-flash")
        .send_message(prompt)
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
