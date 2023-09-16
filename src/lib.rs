use pyo3::prelude::*;

mod aria2c;
mod database;
mod initialization;
mod startup;
mod useful_tools;

use aria2c::{aria2Version, new_date, startAria};
use database::{DataBase, PluginsDB, TempDB};
use initialization::{init_create_folders, init_log_file};
use startup::{addstartup, checkstartup, removestartup};
use useful_tools::{
    convertToByte, determineConfigFolder, humanReadableSize, osAndDesktopEnvironment,
    returnDefaultSettings,
};

#[pymodule]
fn ghermez(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_function(wrap_pyfunction!(startAria, m)?)?;
    m.add_function(wrap_pyfunction!(aria2Version, m)?)?;
    m.add_function(wrap_pyfunction!(new_date, m)?)?;

    m.add_class::<DataBase>()?;
    m.add_class::<TempDB>()?;
    m.add_class::<PluginsDB>()?;

    m.add_function(wrap_pyfunction!(determineConfigFolder, m)?)?;
    m.add_function(wrap_pyfunction!(humanReadableSize, m)?)?;
    m.add_function(wrap_pyfunction!(convertToByte, m)?)?;
    m.add_function(wrap_pyfunction!(osAndDesktopEnvironment, m)?)?;
    m.add_function(wrap_pyfunction!(returnDefaultSettings, m)?)?;

    m.add_function(wrap_pyfunction!(checkstartup, m)?)?;
    m.add_function(wrap_pyfunction!(addstartup, m)?)?;
    m.add_function(wrap_pyfunction!(removestartup, m)?)?;

    m.add_function(wrap_pyfunction!(init_create_folders, m)?)?;
    m.add_function(wrap_pyfunction!(init_log_file, m)?)?;

    Ok(())
}
