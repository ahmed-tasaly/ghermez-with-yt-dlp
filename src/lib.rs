use pyo3::prelude::*;

mod aria2c;
mod database;
mod initialization;
mod logger;
mod os_command;
mod startup;
mod useful_tools;

use aria2c::{
    activeDownloads, aria2Version, downloadPause, downloadUnpause, findDownloadPath, limitSpeed,
    nowDate, shutDown, startAria, tellActive,
};
use database::{DataBase, PluginsDB, TempDB};
use initialization::{init_create_folders, init_log_file};
use logger::{initLogger, sendToLog};
use os_command::{makeDirs, moveFile, remove, removeDir, touch, xdgOpen};
use startup::{addstartup, checkstartup, removestartup};
use useful_tools::{
    convertToByte, determineConfigFolder, humanReadableSize, osAndDesktopEnvironment,
    returnDefaultSettings,
};

#[cfg(not(target_os = "windows"))]
use useful_tools::freeSpace;

#[pymodule]
fn ghermez(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(startAria, m)?)?;
    m.add_function(wrap_pyfunction!(aria2Version, m)?)?;
    m.add_function(wrap_pyfunction!(tellActive, m)?)?;
    m.add_function(wrap_pyfunction!(findDownloadPath, m)?)?;
    m.add_function(wrap_pyfunction!(shutDown, m)?)?;
    m.add_function(wrap_pyfunction!(downloadPause, m)?)?;
    m.add_function(wrap_pyfunction!(downloadUnpause, m)?)?;
    m.add_function(wrap_pyfunction!(limitSpeed, m)?)?;
    m.add_function(wrap_pyfunction!(activeDownloads, m)?)?;
    m.add_function(wrap_pyfunction!(nowDate, m)?)?;

    m.add_class::<DataBase>()?;
    m.add_class::<TempDB>()?;
    m.add_class::<PluginsDB>()?;

    m.add_function(wrap_pyfunction!(determineConfigFolder, m)?)?;
    m.add_function(wrap_pyfunction!(humanReadableSize, m)?)?;
    m.add_function(wrap_pyfunction!(convertToByte, m)?)?;

    #[cfg(not(target_os = "windows"))]
    m.add_function(wrap_pyfunction!(freeSpace, m)?)?;

    m.add_function(wrap_pyfunction!(osAndDesktopEnvironment, m)?)?;
    m.add_function(wrap_pyfunction!(returnDefaultSettings, m)?)?;

    m.add_function(wrap_pyfunction!(checkstartup, m)?)?;
    m.add_function(wrap_pyfunction!(addstartup, m)?)?;
    m.add_function(wrap_pyfunction!(removestartup, m)?)?;

    m.add_function(wrap_pyfunction!(init_create_folders, m)?)?;
    m.add_function(wrap_pyfunction!(init_log_file, m)?)?;

    m.add_function(wrap_pyfunction!(initLogger, m)?)?;
    m.add_function(wrap_pyfunction!(sendToLog, m)?)?;

    m.add_function(wrap_pyfunction!(touch, m)?)?;
    m.add_function(wrap_pyfunction!(xdgOpen, m)?)?;
    m.add_function(wrap_pyfunction!(remove, m)?)?;
    m.add_function(wrap_pyfunction!(removeDir, m)?)?;
    m.add_function(wrap_pyfunction!(makeDirs, m)?)?;
    m.add_function(wrap_pyfunction!(moveFile, m)?)?;

    Ok(())
}
