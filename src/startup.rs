#[cfg(not(target_os = "windows"))]
use std::{fs, path::PathBuf};

#[cfg(not(target_os = "windows"))]
use home::home_dir;

#[cfg(not(target_os = "windows"))]
use once_cell::sync::Lazy;

#[cfg(target_os = "linux")]
use std::os::unix::prelude::PermissionsExt;

#[cfg(any(target_os = "macos", target_os = "windows"))]
use std::env;
#[cfg(target_os = "macos")]
use std::process::Command;

use pyo3::prelude::*;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

#[cfg(not(target_os = "windows"))]
static HOME_ADDRESS: Lazy<PathBuf> = Lazy::new(|| home_dir().unwrap());

// check startup
#[pyfunction]
pub fn checkstartup() -> bool {
    // check if the startup exists
    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
    return HOME_ADDRESS
        .join("/.config/autostart/ghermez.desktop")
        .is_file();

    #[cfg(target_os = "macos")]
    return HOME_ADDRESS
        .join("/Library/LaunchAgents/com.ghermez.plist")
        .is_file();

    #[cfg(target_os = "windows")]
    {
        let key = RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
            KEY_ALL_ACCESS,
        );
        match key {
            Ok(key) => key.get_value::<String, &str>("ghermez").is_ok(),
            Err(_) => false,
        }
    }
}

#[pyfunction]
pub fn addstartup() {
    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
    {
        let entry = "[Desktop Entry]
    Name=Ghermez Download Manager
    Name[fa]=قرمز
    Comment=Download Manager
    GenericName=Download Manager
    GenericName[fa]=نرم افزار مدیریت بارگیری
    Keywords=Internet;WWW;Web;
    Terminal=false
    Type=Application
    Categories=Qt;Network;
    StartupNotify=true
    Exec=ghermez --tray
    Icon=ghermez
    StartupWMClass=ghermez-download-Manager
    ";
        let autostart_dir = HOME_ADDRESS.join(".config").join("autostart");
        if !autostart_dir.exists() {
            fs::create_dir_all(&autostart_dir).unwrap();
            let _ = fs::set_permissions(&autostart_dir, fs::Permissions::from_mode(0o755));
        }
        let desktop_file_path = autostart_dir.join("ghermez.desktop");
        let _ = fs::write(&desktop_file_path, entry);
        let _ = fs::set_permissions(&desktop_file_path, fs::Permissions::from_mode(0o644));
    }

    #[cfg(target_os = "macos")]
    {
        let cwd = env::current_dir().unwrap();
        let entry = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>
    <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
    <plist version=\"1.0\">
    <dict>
      <key>Label</key>
      <string>ir.iamrezamousavi.ghermez</string>
      <key>Program</key>
      <string>''' {} '''/Ghermez Download Manager</string>
      <key>ProgramArguments</key>
      <array>
        <string>--tray</string>
      </array>
      <key>RunAtLoad</key>
      <true/>
    </dict>
    </plist>\n", cwd.display());
        let startup_file_path = HOME_ADDRESS.join("/Library/LaunchAgents/com.ghermez.plist");
        fs::write(&startup_file_path, entry).unwrap();
        Command::new("launchctl")
            .args(["load", startup_file_path.to_str().unwrap()])
            .spawn()
            .unwrap();
    }

    #[cfg(target_os = "windows")]
    {
        let key = RegKey::predef(HKEY_CURRENT_USER)
            .open_subkey_with_flags(
                "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                KEY_ALL_ACCESS,
            )
            .unwrap();

        let cwd = env::current_dir().unwrap();
        let ghermez_exe_tray =
            format!("\"{}\\Ghermez Download Manager.exe\" --tray", cwd.display());

        let _ = key.set_value("ghermez", &ghermez_exe_tray);
    }
}

#[pyfunction]
pub fn removestartup() {
    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
    fs::remove_file(HOME_ADDRESS.join("/.config/autostart/ghermez.desktop")).unwrap();

    #[cfg(target_os = "macos")]
    {
        if checkstartup() {
            let startup_file_path = HOME_ADDRESS.join("/Library/LaunchAgents/com.ghermez.plist");
            Command::new("launchctl")
                .args(["unload", startup_file_path.to_str().unwrap()])
                .status()
                .unwrap();
            let _ = fs::remove_file(startup_file_path);
        }
    }

    #[cfg(target_os = "windows")]
    {
        if checkstartup() {
            let key = RegKey::predef(HKEY_CURRENT_USER)
                .open_subkey_with_flags(
                    "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                    KEY_ALL_ACCESS,
                )
                .unwrap();
            let _ = key.delete_value("ghermez");
        }
    }
}
