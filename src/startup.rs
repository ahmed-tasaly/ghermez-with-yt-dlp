use std::{path::PathBuf, fs};

#[cfg(target_os = "linux")]
use std::os::unix::prelude::PermissionsExt;

#[cfg(target_os = "macos")]
use std::env;
#[cfg(target_os = "macos")]
use std::path::Path;
#[cfg(target_os = "macos")]
use std::process::Command;

use home::home_dir;
use once_cell::sync::Lazy;
use pyo3::prelude::*;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

static HOME_ADDRESS: Lazy<PathBuf> = Lazy::new(|| home_dir().unwrap());

// check startup
#[pyfunction]
pub fn checkstartup() -> bool {
  // check if the startup exists
  #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
  if HOME_ADDRESS.join("/.config/autostart/persepolis.desktop").is_file() {
    return true;
  } else {
    return false;
  }

  #[cfg(target_os = "macos")]
  if HOME_ADDRESS.join("/Library/LaunchAgents/com.persepolisdm.plist").is_file() {
    return true;
  } else {
    return false;
  }

  #[cfg(target_os = "windows")]
  {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    match hklm.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run") {
      Ok(_) => return true,
      Err(_) => return false,
    }
  }
}

#[pyfunction]
pub fn addstartup() {
  #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
  {
    let entry = "[Desktop Entry]
    Name=Persepolis Download Manager
    Name[fa]=پرسپولیس
    Comment=Download Manager
    GenericName=Download Manager
    GenericName[fa]=نرم افزار مدیریت بارگیری
    Keywords=Internet;WWW;Web;
    Terminal=false
    Type=Application
    Categories=Qt;Network;
    StartupNotify=true
    Exec=persepolis --tray
    Icon=persepolis
    StartupWMClass=persepolis-download-Manager
    ";
    if !HOME_ADDRESS.join("/.config/autostart").exists() {
      fs::create_dir_all(HOME_ADDRESS.join("/.config/autostart")).unwrap();
    }
    fs::write(HOME_ADDRESS.join("/.config/autostart/persepolis.desktop"), entry).unwrap();
    fs::set_permissions(HOME_ADDRESS.join("/.config/autostart/persepolis.desktop"), fs::Permissions::from_mode(0o644)).unwrap();
  }

  #[cfg(target_os = "macos")]
  {
    let args: Vec<String> = env::args().collect();
    let current_directory = &args[0];
    let cwd = Path::new(current_directory).parent().unwrap();
    let entry = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>
    <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
    <plist version=\"1.0\">
    <dict>
      <key>Label</key>
      <string>com.persepolisdm.persepolis</string>
      <key>Program</key>
      <string>''' {} '''/Persepolis Download Manager</string>
      <key>ProgramArguments</key>
      <array>
        <string>--tray</string>
      </array>
      <key>RunAtLoad</key>
      <true/>
    </dict>
    </plist>\n", cwd.display());
    fs::write(HOME_ADDRESS.join("/Library/LaunchAgents/com.persepolisdm.plist"), entry).unwrap();
    Command::new("launchctl")
      .args(&["load", HOME_ADDRESS.join("/Library/LaunchAgents/com.persepolisdm.plist").to_str().unwrap()])
      .spawn()
      .unwrap();
  }

  #[cfg(target_os = "windows")]
  {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let cur_ver = hklm.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run").unwrap();
    // TODO
  }
}

#[pyfunction]
pub fn removestartup() {
  #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
  fs::remove_file(HOME_ADDRESS.join("/.config/autostart/persepolis.desktop")).unwrap();

  #[cfg(target_os = "macos")]
  {
    if checkstartup() {
      Command::new("launchctl")
        .args(&["unload", HOME_ADDRESS.join("/Library/LaunchAgents/com.persepolisdm.plist").to_str().unwrap()])
        .status()
        .unwrap();
      fs::remove_file(HOME_ADDRESS.join("/Library/LaunchAgents/com.persepolisdm.plist")).unwrap();
    }
  }
}
