use eframe::egui;
use gemini_rs::types::Schema;
use std::io;
use winreg::RegKey;
use winreg::enums::*;

enum Theme {
    Light,
    Dark,
}

fn set_theme(theme: Theme) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let personalize = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize",
        KEY_SET_VALUE,
    )?;

    match theme {
        Theme::Light => {
            personalize.set_value("AppsUseLightTheme", &1u32)?;
            personalize.set_value("SystemUsesLightTheme", &1u32)?;
        }
        Theme::Dark => {
            personalize.set_value("AppsUseLightTheme", &0u32)?;
            personalize.set_value("SystemUsesLightTheme", &0u32)?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_decorations(false),
        ..Default::default()
    };

    let mut query_current = String::new();
    let mut query_final = String::new();

    eframe::run_simple_native("Guppy", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Guppy");
            ui.horizontal(|ui| {
                let prompt_label = ui.label("Prompt");
                let query_edit = ui
                    .text_edit_singleline(&mut query_current)
                    .labelled_by(prompt_label.id);

                if query_edit.lost_focus() {
                    query_final = query_current.clone();
                    query_current.clear();
                    println!("Name changed to: {}", query_final);
                    let query = query_final.clone();
                    tokio::spawn(async move {
                        if let Err(e) = prompt(&query).await {
                            println!("Error: {}", e);
                        }
                    });
                }
            });
            ui.label(&query_final)
        });
    })
}

async fn prompt(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response_schema = Schema {
        schema_type: Some(gemini_rs::types::Type::String),
        enum_values: Some(vec!["light".to_string(), "dark".to_string()]),
        ..Default::default()
    };

    let response = gemini_rs::chat("gemini-2.0-flash")
        .to_json()
        .response_schema(response_schema.clone())
        .send_message(input)
        .await?;

    if let Some(result) = &response.candidates[0].content.parts[0].text {
        println!("Response: {}", result);
        if result.contains("light") {
            println!("Setting theme to light mode...");
            set_theme(Theme::Light)?;
        } else if result.contains("dark") {
            println!("Setting theme to dark mode...");
            set_theme(Theme::Dark)?;
        } else {
            println!("Unknown theme response.");
        }
    } else {
        println!("No response found.");
    }

    Ok(())
}
