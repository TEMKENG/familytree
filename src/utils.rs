use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::Path;

pub fn concat_id<T: Debug>(condition: bool, id_1: T, id_2: T) -> String {
    if condition {
        format!("{id_1:?}_{id_2:?}")
    } else {
        format!("{id_2:?}_{id_1:?}")
    }
}

pub fn get_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

pub fn set_logger(log_file: Option<&str>) {
    // Set the desired log level
    std::env::set_var("RUST_LOG", "info");
    let log_file: &str = log_file.unwrap_or("application.log");
    let log_file = std::fs::File::create(log_file).expect("Failed to create log file");

    // Initialize the logger
    fern::Dispatch::new()
        .format(move |buf, message, record| {
            buf.finish(format_args!(
                "[{file}::{line}::{date}::{level}::{message}",
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level = record.level(),
                message = message,
                file = record.file().unwrap_or("unknown"),
                line = record.line().unwrap_or(0),
            ));
        })
        .chain(log_file)
        .apply()
        .expect("Failed to initialize logger");
}

pub fn ternary<T: Debug>(condition: bool, a: T, b: T) -> T {
    if condition {
        a
    } else {
        b
    }
}
