#![allow(non_snake_case)]

use lazy_static::lazy_static;
use log::{error, info, warn, Level, LevelFilter, Log, Metadata, Record};
use pyo3::prelude::*;

use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::Path,
    sync::Mutex,
};

use crate::os_command;
use crate::useful_tools;

lazy_static! {
    static ref LOGGER: SimpleLogger = SimpleLogger {
        inner: Mutex::new(None),
    };
}

struct SimpleLogger {
    inner: Mutex<Option<SimpleLoggerInner>>,
}

impl SimpleLogger {
    // Set this `SimpleLogger`'s sink and reset the start time.
    fn renew<T: Write + Send + 'static>(&self, sink: T) {
        *self.inner.lock().unwrap() = Some(SimpleLoggerInner {
            sink: Box::new(sink),
        });
    }
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        if let Some(ref mut inner) = *self.inner.lock().unwrap() {
            inner.log(record);
        }
    }

    fn flush(&self) {}
}

struct SimpleLoggerInner {
    sink: Box<dyn Write + Send>,
}

impl SimpleLoggerInner {
    fn log(&mut self, record: &Record) {
        let now = chrono::Local::now();

        let _ = writeln!(
            self.sink,
            "[{}] {} - {} - {}",
            now.format("%Y-%m-%d %H:%M:%S,%.3f"),
            record.module_path().unwrap_or("<unknown>"),
            record.level(),
            record.args()
        );
    }
}

pub fn _log_to_file<T: AsRef<Path>>(path: T, max_log_level: LevelFilter) -> io::Result<()> {
    let file = File::create(path)?;
    _log_to(file, max_log_level);

    Ok(())
}

pub fn _log_to_stderr(max_log_level: LevelFilter) {
    _log_to(io::stderr(), max_log_level);
}

pub fn _log_to<T: Write + Send + 'static>(sink: T, max_log_level: LevelFilter) {
    LOGGER.renew(sink);
    log::set_max_level(max_log_level);
    // The only possible error is if this has been called before
    let _ = log::set_logger(&*LOGGER);
    // TODO: too much?
    // assert_eq!(log::logger() as *const dyn Log, &*LOGGER as *const dyn Log);
}

#[pyfunction]
pub fn initLogger() {
    let config_folder = useful_tools::determineConfigFolder();
    if !config_folder.exists() {
        fs::create_dir_all(config_folder.clone()).unwrap();
    }

    let log_file = config_folder.join("ghermezdm.log");
    if !log_file.is_file() {
        os_command::touch(log_file.to_str().unwrap());
    }

    let file = OpenOptions::new().append(true).open(log_file).unwrap();
    LOGGER.renew(file);
    let _ = log::set_logger(&*LOGGER).map(|()| log::set_max_level(LevelFilter::Info));
}

#[pyfunction]
#[pyo3(signature = (text="", level=""))]
pub fn sendToLog(text: &str, level: &str) {
    match level {
        "INFO" => info!("{}", text),
        "ERROR" => error!("{}", text),
        _ => warn!("{}", text),
    }
}
