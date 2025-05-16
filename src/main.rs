// use winreg::enums::*;
// use winreg::RegKey;
// use std::io;
//
// fn set_theme_light() -> io::Result<()> {
//     let hkcu = RegKey::predef(HKEY_CURRENT_USER);
//     let personalize = hkcu.open_subkey_with_flags(
//         "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize",
//         KEY_SET_VALUE,
//     )?;
//
//     personalize.set_value("AppsUseLightTheme", &1u32)?;
//     personalize.set_value("SystemUsesLightTheme", &1u32)?;
//     Ok(())
// }
//
// fn main() {
//     match set_theme_light() {
//         Ok(_) => println!("Theme set to light mode!"),
//         Err(e) => eprintln!("Failed to set light theme: {}", e),
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}",
        gemini_rs::chat("gemini-2.0-flash")
            .send_message("Explain how AI works")
            .await?
    );
    Ok(())
}
