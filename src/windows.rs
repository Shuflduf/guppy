use std::io;
use std::process::Command;
use winreg::RegKey;
use winreg::enums::*;

pub enum Theme {
    Light,
    Dark,
}

pub fn set_theme(theme: Theme) -> io::Result<()> {
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


pub fn open_app(app_id: &str) -> io::Result<()> {
    Command::new("cmd")
        .args(["/C", "start", "", app_id])
        .spawn()?; // Use spawn to not wait for the app to finish
    Ok(())
}
