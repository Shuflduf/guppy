use std::io;
use std::process::Command;
use winreg::RegKey;
use winreg::enums::*;

pub enum Theme {
    Light,
    Dark,
}

pub fn set_theme(theme: Theme) -> Result<(), io::Error> {
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

// this function works for urls as well idk
pub fn open_app(app_id: &str) -> Result<(), io::Error> {
    Command::new("cmd")
        .args(["/C", "start", "", app_id])
        .spawn()?;
    Ok(())
}

pub fn get_volume() -> u32 {
    let ps_script = r#"
Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class Audio {
    [ComImport]
    [Guid("BCDE0395-E52F-467C-8E3D-C4579291692E")]
    public class MMDeviceEnumerator {}
    [ComImport]
    [Guid("A95664D2-9614-4F35-A746-DE8DB63617E6")]
    [InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
    public interface IMMDeviceEnumerator {
        int NotImpl1();
        int GetDefaultAudioEndpoint(int dataFlow, int role, out IMMDevice ppDevice);
    }
    [ComImport]
    [Guid("D666063F-1587-4E43-81F1-B948E807363F")]
    [InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
    public interface IMMDevice {
        int Activate(ref Guid iid, int dwClsCtx, IntPtr pActivationParams, [MarshalAs(UnmanagedType.IUnknown)] out object ppInterface);
    }
    [ComImport]
    [Guid("5CDF2C82-841E-4546-9722-0CF74078229A")]
    [InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
    public interface IAudioEndpointVolume {
        int NotImpl1();
        int NotImpl2();
        int GetChannelCount(out uint channelCount);
        int SetMasterVolumeLevel(float level, Guid eventContext);
        int SetMasterVolumeLevelScalar(float level, Guid eventContext);
        int GetMasterVolumeLevel(out float level);
        int GetMasterVolumeLevelScalar(out float level);
    }
    public static float GetVolume() {
        var enumerator = new MMDeviceEnumerator() as IMMDeviceEnumerator;
        IMMDevice dev = null;
        enumerator.GetDefaultAudioEndpoint(0, 1, out dev);
        Guid IID_IAudioEndpointVolume = typeof(IAudioEndpointVolume).GUID;
        object epv;
        dev.Activate(ref IID_IAudioEndpointVolume, 23, IntPtr.Zero, out epv);
        float vol = -1;
        ((IAudioEndpointVolume)epv).GetMasterVolumeLevelScalar(out vol);
        return vol;
    }
}
"@
"{0:N0}" -f ([Audio]::GetVolume() * 100) + "%"
"#;

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", ps_script])
        .output()
        .expect("Failed to execute PowerShell script");

    String::from_utf8_lossy(&output.stdout)
        .trim()
        .trim_end_matches('%')
        .parse()
        .expect("Failed to parse percent value as u8")
}

pub fn set_volume(volume_percent: u8) -> Result<(), io::Error> {
    let percent = volume_percent.min(100);
    let scalar = percent as f32 / 100.0;

    let ps_script = format!(
        r#"
Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class Audio {{
    [ComImport]
    [Guid("BCDE0395-E52F-467C-8E3D-C4579291692E")]
    public class MMDeviceEnumerator {{}}
    [ComImport]
    [Guid("A95664D2-9614-4F35-A746-DE8DB63617E6")]
    [InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
    public interface IMMDeviceEnumerator {{
        int NotImpl1();
        int GetDefaultAudioEndpoint(int dataFlow, int role, out IMMDevice ppDevice);
    }}
    [ComImport]
    [Guid("D666063F-1587-4E43-81F1-B948E807363F")]
    [InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
    public interface IMMDevice {{
        int Activate(ref Guid iid, int dwClsCtx, IntPtr pActivationParams, [MarshalAs(UnmanagedType.IUnknown)] out object ppInterface);
    }}
    [ComImport]
    [Guid("5CDF2C82-841E-4546-9722-0CF74078229A")]
    [InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
    public interface IAudioEndpointVolume {{
        int NotImpl1();
        int NotImpl2();
        int GetChannelCount(out uint channelCount);
        int SetMasterVolumeLevel(float level, Guid eventContext);
        int SetMasterVolumeLevelScalar(float level, Guid eventContext);
        int GetMasterVolumeLevel(out float level);
        int GetMasterVolumeLevelScalar(out float level);
    }}
    public static void SetVolume(float scalar) {{
        var enumerator = new MMDeviceEnumerator() as IMMDeviceEnumerator;
        IMMDevice dev = null;
        enumerator.GetDefaultAudioEndpoint(0, 1, out dev);
        Guid IID_IAudioEndpointVolume = typeof(IAudioEndpointVolume).GUID;
        object epv;
        dev.Activate(ref IID_IAudioEndpointVolume, 23, IntPtr.Zero, out epv);
        ((IAudioEndpointVolume)epv).SetMasterVolumeLevelScalar(scalar, Guid.Empty);
    }}
}}
"@
[Audio]::SetVolume({})
"#,
        scalar
    );

    Command::new("powershell")
        .args(["-NoProfile", "-Command", &ps_script])
        .output()?;

    Ok(())
}
