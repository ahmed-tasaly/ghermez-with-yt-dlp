use pyo3::prelude::*;

mod aria2c;

use aria2c::{aria2Version, new_date, startAria};

#[pymodule]
fn ghermez(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_function(wrap_pyfunction!(startAria, m)?)?;
    m.add_function(wrap_pyfunction!(aria2Version, m)?)?;
    m.add_function(wrap_pyfunction!(new_date, m)?)?;

    Ok(())
}
