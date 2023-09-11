use pyo3::prelude::*;

mod aria2c;
mod database;

use aria2c::{aria2Version, new_date, startAria};
use database::{DataBase, PluginsDB, TempDB};

#[pymodule]
fn ghermez(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_function(wrap_pyfunction!(startAria, m)?)?;
    m.add_function(wrap_pyfunction!(aria2Version, m)?)?;
    m.add_function(wrap_pyfunction!(new_date, m)?)?;

    m.add_class::<DataBase>()?;
    m.add_class::<TempDB>()?;
    m.add_class::<PluginsDB>()?;

    Ok(())
}
