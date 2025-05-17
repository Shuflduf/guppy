use gemini_rs::types::Schema;

use crate::windows::{set_theme, Theme};

pub async fn prompt(input: &str) -> Result<(), Box<dyn std::error::Error>> {
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
