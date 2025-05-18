use gemini_rs::types::{FunctionDeclaration, Tools};
use serde_json::json;

use crate::windows::{self, Theme};

pub async fn prompt(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Prompting: {}", input);
    let function_decs = vec![
        FunctionDeclaration {
            name: "set_theme".to_string(),
            description: "Set the theme of Windows".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "theme": {
                        "type": "string",
                        "description": "The theme to set",
                        "enum": ["light", "dark"]
                    }
                },
                "required": ["theme"]
            }),
        },
        FunctionDeclaration {
            name: "open_app".to_string(),
            description: "Open an app using the name of the exe on windows.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "app_id": { "type": "string", "description": "The name of the app to open" }
                },
                "required": ["app_id"]
            }),
        },
        FunctionDeclaration {
            name: "open_website".to_string(),
            description: "Open a website in the default browser. Always include https:// in the URL. It can also understand paths, and GitHub repositories".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string", "description": "The URL to open" }
                },
                "required": ["url"]
            }),
        }
    ];
    let tools = vec![Tools {
        function_declarations: function_decs,
    }];

    let client = gemini_rs::Client::instance();
    let mut req = client.generate_content("gemini-2.0-flash");

    req.tools(tools);
    req.message(input);

    let response = req.await;

    // let mut req = GenerateContent::new("gemini-2.0-pro".into());
    // req.body.tools = tools;
    // req.message("Set the theme to dark.");
    //
    // let mut chat = gemini_rs::chat("gemini-2.0-flash")
    //     .to_json()
    //     .response_schema(response_schema.clone());
    // let response = chat.send_message(input).await?;
    match response {
        Ok(resp) => match &resp.candidates[0].content.parts[0].function_call {
            Some(func) => match func.name.as_str() {
                "set_theme" => {
                    let theme = func.args["theme"].as_str().unwrap();
                    println!("Setting theme to {}", theme);
                    windows::set_theme(if theme == "light" {
                        Theme::Light
                    } else {
                        Theme::Dark
                    })?;
                }
                "open_app" => {
                    let app_id = func.args["app_id"].as_str().unwrap();
                    println!("Opening app {}", app_id);
                    windows::open_app(app_id)?;
                }
                "open_website" => {
                    let url = func.args["url"].as_str().unwrap();
                    println!("Opening website {}", url);
                    windows::open_app(url)?;
                }
                _ => eprintln!("Unknown function: {}", func.name),
            },
            None => eprint!("Function not called"),
        },
        Err(e) => eprintln!("Error: {e}"),
    }
    // if let Some(result) = &response.candidates[0].content.parts[0].text {
    //     println!("Response: {}", result);
    //     if result.contains("setThemeLight") {
    //         println!("Setting theme to light mode...");
    //         set_theme(Theme::Light)?;
    //     } else if result.contains("setThemeDark") {
    //         println!("Setting theme to dark mode...");
    //         set_theme(Theme::Dark)?;
    //     } else {
    //         println!("Unknown theme response.");
    //     }
    // } else {
    //     println!("No response found.");
    // }

    Ok(())
}
