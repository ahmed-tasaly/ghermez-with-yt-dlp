#![allow(non_snake_case)]

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread, time,
};

#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::env;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use log::{error, info};
use once_cell::sync::Lazy;
use pyo3::prelude::*;
use serde_json::Map;
use tokio::{runtime::Runtime, sync::RwLock};

use aria2_ws::{Client, TaskOptions};

static SERVER_URL: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::new()));

#[pyfunction]
#[pyo3(signature = (port, _aria2_path=None))]
pub fn startAria(port: u16, _aria2_path: Option<String>) -> Option<String> {
    Runtime::new().unwrap().handle().block_on(async {
        let mut tmp = SERVER_URL.write().await;
        *tmp = format!("ws://127.0.0.1:{port}/jsonrpc");
    });

    #[cfg(target_os = "linux")]
    let _child = match Command::new("aria2c")
        .arg("--no-conf")
        .arg("--enable-rpc")
        .arg(format!("--rpc-listen-port={}", port))
        .arg("--rpc-allow-origin-all")
        .arg("--quiet=true")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn aria2c: {:?}", why),
        Ok(child) => child,
    };

    #[cfg(target_os = "macos")]
    {
        let aria2d;
        if _aria2_path.is_none()
            || _aria2_path
                .as_ref()
                .is_some_and(|x| x == "" || !Path::new(&x).is_file())
        {
            let args: Vec<String> = env::args().collect();
            let current_directory = &args[0];
            let aria2 = Path::new(current_directory)
                .parent()
                .unwrap()
                .join("aria2c");
            aria2d = aria2.to_str().unwrap().to_string();
        } else {
            aria2d = _aria2_path.as_ref().unwrap().to_string();
        }
        if !Path::new(&aria2d).exists() {
            error!("Aria2 does not exist in the current path!");
            return None;
        }

        let _child = match Command::new(aria2d)
            .arg("--no-conf")
            .arg("--enable-rpc")
            .arg(format!("--rpc-listen-port={}", port))
            .arg("--rpc-allow-origin-all")
            .arg("--quiet=true")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
        {
            Err(why) => panic!("couldn't spawn aria2c: {:?}", why),
            Ok(child) => child,
        };
    }

    #[cfg(target_os = "windows")]
    {
        let aria2d;
        if _aria2_path.is_none()
            || _aria2_path
                .as_ref()
                .is_some_and(|x| x == "" || !Path::new(&x).is_file())
        {
            let args: Vec<String> = env::args().collect();
            let current_directory = &args[0];
            let aria2 = Path::new(current_directory)
                .parent()
                .unwrap()
                .join("aria2c.exe");
            aria2d = aria2.to_str().unwrap().to_string();
        } else {
            aria2d = _aria2_path.as_ref().unwrap().to_string();
        }
        if !Path::new(&aria2d).exists() {
            error!("Aria2 does not exist in the current path!");
            return None;
        }

        // NO_WINDOW option avoids opening additional CMD window in MS Windows.
        const NO_WINDOW: u32 = 0x08000000;

        let _child = match Command::new(aria2d)
            .arg("--no-conf")
            .arg("--enable-rpc")
            .arg(format!("--rpc-listen-port={}", port))
            .arg("--rpc-allow-origin-all")
            .arg("--quiet=true")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .creation_flags(NO_WINDOW)
            .spawn()
        {
            Err(why) => panic!("couldn't spawn aria2c: {:?}", why),
            Ok(child) => child,
        };
    }

    thread::sleep(time::Duration::from_secs(2));

    // check that starting is successful or not!
    let answer = aria2Version();
    Some(answer)
}

#[pyfunction]
pub fn aria2Version() -> String {
    let version = Runtime::new().unwrap().handle().block_on(async {
        let server_url = SERVER_URL.read().await;
        match Client::connect(&server_url, None).await {
            Ok(client) => client.get_version().await,
            Err(e) => Err(e),
        }
    });

    match version {
        Ok(v) => v.version,
        Err(_) => {
            // write ERROR messages in terminal and log
            error!("Aria2 didn't respond!");
            "did not respond".to_string()
        }
    }
}

fn _download_aria(url: &str) -> String {
    let gid = Runtime::new().unwrap().handle().block_on(async {
        let server_url = SERVER_URL.read().await;
        let client = Client::connect(&server_url, None).await.unwrap();
        let options = TaskOptions::default();

        client
            .add_uri(vec![url.to_string()], Some(options.clone()), None, None)
            .await
            .unwrap()
    });
    gid
}

fn _tell_active() -> Vec<Map<String, serde_json::Value>> {
    let args = vec![
        "gid".to_string(),
        "status".to_string(),
        "connections".to_string(),
        "errorCode".to_string(),
        "errorMessage".to_string(),
        "downloadSpeed".to_string(),
        "connections".to_string(),
        "dir".to_string(),
        "totalLength".to_string(),
        "completedLength".to_string(),
        "files".to_string(),
    ];
    let status = Runtime::new().unwrap().handle().block_on(async {
        let server_url = SERVER_URL.read().await;
        let client = Client::connect(&server_url, None).await.unwrap();
        client.custom_tell_active(Some(args)).await.unwrap()
    });

    status
}

fn _tell_status(gid: &str) -> Map<String, serde_json::Value> {
    let args = vec![
        "status".to_string(),
        "connections".to_string(),
        "errorCode".to_string(),
        "errorMessage".to_string(),
        "downloadSpeed".to_string(),
        "connections".to_string(),
        "dir".to_string(),
        "totalLength".to_string(),
        "completedLength".to_string(),
        "files".to_string(),
    ];
    let status = Runtime::new().unwrap().handle().block_on(async {
        let server_url = SERVER_URL.read().await;
        let client = Client::connect(&server_url, None).await.unwrap();
        client.custom_tell_status(gid, Some(args)).await.unwrap()
    });

    status
}

fn _find_download_path(file_name: &str, download_path: PathBuf, subfolder: bool) -> PathBuf {
    let file_extension = Path::new(file_name)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap()
        .to_lowercase();

    // audio formats
    let audio = [
        "act", "aiff", "aac", "amr", "ape", "au", "awb", "dct", "dss", "dvf", "flac", "gsm",
        "iklax", "ivs", "m4a", "m4p", "mmf", "mp3", "mpc", "msv", "ogg", "oga", "opus", "ra",
        "raw", "sln", "tta", "vox", "wav", "wma", "wv",
    ];

    // video formats
    let video = [
        "3g2", "3gp", "asf", "avi", "drc", "flv", "m4v", "mkv", "mng", "mov", "qt", "mp4", "m4p",
        "mpg", "mp2", "mpeg", "mpe", "mpv", "m2v", "mxf", "nsv", "ogv", "rmvb", "roq", "svi",
        "vob", "webm", "wmv", "yuv", "rm",
    ];

    // document formats
    let document = [
        "doc", "docx", "html", "htm", "fb2", "odt", "sxw", "pdf", "ps", "rtf", "tex", "txt",
        "epub", "pub", "mobi", "azw", "azw3", "azw4", "kf8", "chm", "cbt", "cbr", "cbz", "cb7",
        "cba", "ibooks", "djvu", "md",
    ];

    // compressed formats
    let compressed = [
        "a", "ar", "cpio", "shar", "LBR", "iso", "lbr", "mar", "tar", "bz2", "F", "gz", "lz",
        "lzma", "lzo", "rz", "sfark", "sz", "xz", "Z", "z", "infl", "7z", "s7z", "ace", "afa",
        "alz", "apk", "arc", "arj", "b1", "ba", "bh", "cab", "cfs", "cpt", "dar", "dd", "dgc",
        "dmg", "ear", "gca", "ha", "hki", "ice", "jar", "kgb", "lzh", "lha", "lzx", "pac",
        "partimg", "paq6", "paq7", "paq8", "pea", "pim", "pit", "qda", "rar", "rk", "sda", "sea",
        "sen", "sfx", "sit", "sitx", "sqx", "tar.gz", "tgz", "tar.Z", "tar.bz2", "tbz2",
        "tar.lzma", "tlz", "uc", "uc0", "uc2", "ucn", "ur2", "ue2", "uca", "uha", "war", "wim",
        "xar", "xp3", "yz1", "zip", "zipx", "zoo", "zpaq", "zz", "ecc", "par", "par2",
    ];

    if subfolder {
        if audio.contains(&file_extension.as_str()) {
            download_path.join("Audios")
        } else if video.contains(&file_extension.as_str()) {
            download_path.join("Videos")
        } else if document.contains(&file_extension.as_str()) {
            download_path.join("Documents")
        } else if compressed.contains(&file_extension.as_str()) {
            download_path.join("Compressed")
        } else {
            download_path.join("Other")
        }
    } else {
        download_path
    }
}

fn _download_pause(gid: &str) -> Result<(), aria2_ws::Error> {
    let answer = Runtime::new().unwrap().handle().block_on(async {
        let server_url = SERVER_URL.read().await;
        let client = Client::connect(&server_url, None).await.unwrap();
        client.pause(gid).await
    });
    info!("{:?} paused", answer);
    answer
}

fn _download_unpause(gid: &str) -> Result<(), aria2_ws::Error> {
    let answer = Runtime::new().unwrap().handle().block_on(async {
        let server_url = SERVER_URL.read().await;
        let client = Client::connect(&server_url, None).await.unwrap();
        client.unpause(gid).await
    });
    info!("{:?} paused", answer);
    answer
}

fn _limit_speed(gid: &str, limit: &str) {
    let options = TaskOptions {
        max_download_limit: Some(limit.to_string()),
        ..Default::default()
    };

    let answer = Runtime::new().unwrap().handle().block_on(async {
        let server_url = SERVER_URL.read().await;
        let client = Client::connect(&server_url, None).await.unwrap();
        client.change_option(gid, options).await
    });

    match answer {
        Ok(_) => info!("Download speed limit  value is changed"),
        Err(_) => error!("Speed limitation was unsuccessful"),
    }
}

#[pyfunction]
pub fn new_date() -> String {
    let now = chrono::Local::now();
    now.format("%Y/%m/%d , %H:%M:%S").to_string()
}
