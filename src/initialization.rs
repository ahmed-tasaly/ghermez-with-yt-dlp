use std::fs::{self, OpenOptions};
use std::io::prelude::*;

use chrono::Local;
use pyo3::prelude::*;

use crate::useful_tools::determineConfigFolder;

#[pyfunction]
pub fn init_create_folders() {
    // download manager config folder
    let config_folder = determineConfigFolder();
    // persepolis tmp folder path
    let persepolis_tmp = config_folder.join("persepolis_tmp");
    // create folders
    for folder in [config_folder, persepolis_tmp] {
        fs::create_dir_all(folder).unwrap();
    }
}

#[pyfunction]
pub fn init_log_file() {
    // refresh logs!
    let config_folder = determineConfigFolder();
    let log_file = config_folder.join("persepolisdm.log");

    // get current time
    let current_time = Local::now().format("%Y/%m/%d , %H:%M:%S").to_string();

    // find number of lines in log_file
    let len = fs::read_to_string(log_file.clone())
        .unwrap()
        .lines()
        .count();

    // if number of lines in log_file is more than 300, then keep last 200 lines in log_file.
    if len < 300 {
        let content = format!(
            "
    ===================================================
    Persepolis Download Manager, {}
    ",
            current_time
        );
        fs::write(log_file.clone(), content).unwrap();
    } else {
        // keep last 200 lines
        let line_num = len - 200;
        let f_lines: Vec<String> = fs::read_to_string(log_file.clone())
            .unwrap()
            .lines()
            .map(String::from)
            .collect();
        fs::write(log_file.clone(), f_lines[line_num..].join("\n")).unwrap();
        let mut file = OpenOptions::new()
            .append(true)
            .open(log_file.clone())
            .unwrap();
        let content = format!("Persepolis Download Manager, {}", current_time);
        file.write_all(content.as_bytes()).unwrap();
    }
}
