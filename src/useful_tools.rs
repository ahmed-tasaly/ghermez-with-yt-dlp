#![allow(non_snake_case)]
#![allow(unused_assignments)]

use std::{collections::HashMap, path::PathBuf};

#[cfg(not(target_os = "windows"))]
use std::{env, fs, path::Path};

use home::home_dir;
use once_cell::sync::Lazy;
use pyo3::prelude::*;

static HOME_ADDRESS: Lazy<PathBuf> = Lazy::new(|| home_dir().unwrap());

#[cfg(target_os = "linux")]
const OS_TYPE: &str = "Linux";

#[cfg(target_os = "windows")]
const OS_TYPE: &str = "Windows";

#[cfg(target_os = "macos")]
const OS_TYPE: &str = "Darwin";

#[cfg(target_os = "openbsd")]
const OS_TYPE: &str = "OpenBSD";

#[cfg(target_os = "freebsd")]
const OS_TYPE: &str = "FreeBSD";

// determine the config folder path based on the operating system
#[pyfunction]
pub fn determineConfigFolder() -> PathBuf {
    #[cfg(target_os = "linux")]
    let config_folder = HOME_ADDRESS.join(".config/ghermez_download_manager");

    #[cfg(target_os = "macos")]
    let config_folder = HOME_ADDRESS.join("Library/Application Support/ghermez_download_manager");

    #[cfg(target_os = "windows")]
    let config_folder = HOME_ADDRESS
        .join("AppData")
        .join("Local")
        .join("ghermez_download_manager");

    config_folder
}

// this function returns operating system and desktop environment(for linux and bsd).
#[pyfunction]
pub fn osAndDesktopEnvironment() -> (&'static str, Option<String>) {
    let mut desktop_env: Option<String> = None;

    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
    {
        desktop_env = match env::var("XDG_CURRENT_DESKTOP") {
            Ok(val) => Some(val),
            Err(_) => None,
        };
    }
    (OS_TYPE, desktop_env)
}

// this function converts file_size to KiB or MiB or GiB
#[pyfunction]
#[pyo3(signature = (size, input_type="file_size"))]
pub fn humanReadableSize(size: f32, input_type: &str) -> String {
    let labels = ["KiB", "MiB", "GiB", "TiB"];

    if size < 1024.0 {
        return size.to_string() + " B";
    }

    let mut size = size;
    let mut i: isize = -1;
    while size >= 1024.0 {
        i += 1;
        size /= 1024.0;
    }

    let j = if input_type == "speed" { 0 } else { 1 };

    if i > j {
        round(size, 2).to_string() + " " + labels[i as usize]
    } else {
        size.to_string() + " " + labels[i as usize]
    }
}

// this function converts human readable size to byte
#[pyfunction]
pub fn convertToByte(file_size: &str) -> f32 {
    let unit;
    let size_value: f32;
    let len = file_size.len();
    if &file_size[len - 2..] != " B" {
        unit = Some(file_size[len - 3..].to_string());
        if unit.as_ref().is_some_and(|x| x == "GiB" || x == "TiB") {
            size_value = file_size[..len - 4].parse().unwrap();
        } else {
            size_value = round(file_size[..len - 4].parse().unwrap(), 0);
        }
    } else {
        unit = None;
        size_value = round(file_size[..len - 3].parse().unwrap(), 0);
    }

    let in_byte_value;
    if unit.is_none() {
        in_byte_value = size_value;
    } else if unit.as_ref().is_some_and(|x| x == "KiB") {
        in_byte_value = size_value * 1024.0;
    } else if unit.as_ref().is_some_and(|x| x == "MiB") {
        in_byte_value = size_value * 1024.0 * 1024.0;
    } else if unit.as_ref().is_some_and(|x| x == "GiB") {
        in_byte_value = size_value * 1024.0 * 1024.0 * 1024.0;
    } else if unit.as_ref().is_some_and(|x| x == "TiB") {
        in_byte_value = size_value * 1024.0 * 1024.0 * 1024.0 * 1024.0;
    } else {
        in_byte_value = 0.0;
    }
    round(in_byte_value, 0)
}

fn round(x: f32, decimals: u32) -> f32 {
    let y = 10i32.pow(decimals) as f32;
    (x * y).round() / y
}

#[pyfunction]
pub fn returnDefaultSettings(_available_styles: Vec<&str>) -> HashMap<&str, String> {
    let (_os_type, _desktop_env) = osAndDesktopEnvironment();

    // persepolis temporary download folder
    #[cfg(not(target_os = "windows"))]
    let download_path_temp = HOME_ADDRESS.join(".ghermez");
    #[cfg(target_os = "windows")]
    let download_path_temp = HOME_ADDRESS.join("AppData").join("Local").join("ghermez");

    // user download folder path
    let download_path = HOME_ADDRESS.join("Downloads").join("Ghermez");

    // find available styles(It's depends on operating system and desktop environments).
    let mut style = "Fusion";
    let mut color_scheme = "Dark Fusion";
    let mut icons = "Breeze-Dark";

    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
    {
        if _desktop_env.is_some_and(|x| x == "KDE") {
            if _available_styles.contains(&"Breeze") {
                style = "Breeze";
                color_scheme = "System";
            } else {
                // finout user prefers dark theme or light theme :)
                // read this links for more information:
                // https://wiki.archlinux.org/index.php/GTK%2B#Basic_theme_configuration
                // https://wiki.archlinux.org/index.php/GTK%2B#Dark_theme_variant

                // find user gtk3 config file path.
                let mut gtk3_confing_file_path = Some(
                    HOME_ADDRESS
                        .join(".config")
                        .join("gtk-3.0")
                        .join("settings.ini"),
                );
                if !gtk3_confing_file_path.as_ref().is_some_and(|x| x.is_file()) {
                    if Path::new("/etc/gtk-3.0/settings.ini").is_file() {
                        gtk3_confing_file_path =
                            Some(Path::new("/etc/gtk-3.0/settings.ini").to_path_buf());
                    } else {
                        gtk3_confing_file_path = None;
                    }
                }

                // read this for more information:
                let mut dark_theme = false;
                if gtk3_confing_file_path.as_ref().is_some() {
                    for line in fs::read_to_string(gtk3_confing_file_path.unwrap())
                        .unwrap()
                        .lines()
                    {
                        if line.contains("gtk-application-prefer-dark-theme") {
                            dark_theme = line.contains("true");
                        }
                    }
                }

                if dark_theme {
                    icons = "Breeze-Dark";
                    if _available_styles.contains(&"Adwaita-Dark") {
                        style = "Adwaita-Dark";
                        color_scheme = "System";
                    }
                } else {
                    icons = "Breeze";
                    if _available_styles.contains(&"Adwaita") {
                        style = "Adwaita";
                        color_scheme = "System";
                    } else {
                        style = "Fusion";
                        color_scheme = "Light Fusion";
                    }
                }
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        if _available_styles.contains(&"macintosh") {
            style = "macintosh";
            color_scheme = "System";
            icons = "Breeze";
        }
    }
    #[cfg(target_os = "windows")]
    {
        style = "Fusion";
        color_scheme = "Dark Fusion";
        icons = "Breeze-Dark";
    }

    // keyboard shortcuts
    let delete_shortcut = "Ctrl+D";
    let remove_shortcut = "Ctrl+R";
    let add_new_download_shortcut = "Ctrl+N";
    let import_text_shortcut = "Ctrl+O";
    let video_finder_shortcut = "Ctrl+V";
    let quit_shortcut = "Ctrl+Q";
    let hide_window_shortcut = "Ctrl+W";
    let move_up_selection_shortcut = "Ctrl+Up";
    let move_down_selection_shortcut = "Ctrl+Down";

    // Persepolis default setting
    let default_setting_dict = HashMap::from([
        ("locale", "en_US".to_string()),
        ("toolbar_icon_size", "32".to_string()),
        ("wait-queue", "[0, 0]".to_string()),
        ("awake", "no".to_string()),
        ("custom-font", "no".to_string()),
        ("column0", "yes".to_string()),
        ("column1", "yes".to_string()),
        ("column2", "yes".to_string()),
        ("column3", "yes".to_string()),
        ("column4", "yes".to_string()),
        ("column5", "yes".to_string()),
        ("column6", "yes".to_string()),
        ("column7", "yes".to_string()),
        ("column10", "yes".to_string()),
        ("column11", "yes".to_string()),
        ("column12", "yes".to_string()),
        ("subfolder", "yes".to_string()),
        ("startup", "no".to_string()),
        ("show-progress", "yes".to_string()),
        ("show-menubar", "no".to_string()),
        ("show-sidepanel", "yes".to_string()),
        ("rpc-port", "6801".to_string()),
        ("notification", "Native notification".to_string()),
        ("after-dialog", "yes".to_string()),
        ("tray-icon", "yes".to_string()),
        ("browser-persepolis", "yes".to_string()),
        ("hide-window", "yes".to_string()),
        ("max-tries", "5".to_string()),
        ("retry-wait", "0".to_string()),
        ("timeout", "60".to_string()),
        ("connections", "16".to_string()),
        (
            "download_path_temp",
            download_path_temp.to_string_lossy().to_string(),
        ),
        ("download_path", download_path.to_string_lossy().to_string()),
        ("sound", "yes".to_string()),
        ("sound-volume", "100".to_string()),
        ("style", style.to_string()),
        ("color-scheme", color_scheme.to_string()),
        ("icons", icons.to_string()),
        ("font", "Ubuntu".to_string()),
        ("font-size", "9".to_string()),
        ("aria2_path", "".to_string()),
        ("video_finder/max_links", "3".to_string()),
        ("shortcuts/delete_shortcut", delete_shortcut.to_string()),
        ("shortcuts/remove_shortcut", remove_shortcut.to_string()),
        (
            "shortcuts/add_new_download_shortcut",
            add_new_download_shortcut.to_string(),
        ),
        (
            "shortcuts/import_text_shortcut",
            import_text_shortcut.to_string(),
        ),
        (
            "shortcuts/video_finder_shortcut",
            video_finder_shortcut.to_string(),
        ),
        ("shortcuts/quit_shortcut", quit_shortcut.to_string()),
        (
            "shortcuts/hide_window_shortcut",
            hide_window_shortcut.to_string(),
        ),
        (
            "shortcuts/move_up_selection_shortcut",
            move_up_selection_shortcut.to_string(),
        ),
        (
            "shortcuts/move_down_selection_shortcut",
            move_down_selection_shortcut.to_string(),
        ),
        ("dont-check-certificate", "no".to_string()),
    ]);
    default_setting_dict
}
