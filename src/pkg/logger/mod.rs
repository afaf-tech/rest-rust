use crate::config::Config;

use fern::Dispatch;
use log::LevelFilter;
use serde_json::json;
use std::fs::OpenOptions;
use std::path::PathBuf;

pub fn setup_logger(conf: &Config) {
    // Build file paths for logs dynamically
    let info_log_path = PathBuf::from(&conf.log_dir).join(format!("{}.info.log", conf.log_name));
    let error_log_path = PathBuf::from(&conf.log_dir).join(format!("{}.error.log", conf.log_name));

    // Open files for logging
    let info_log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&info_log_path)
        .expect(&format!(
            "Failed to open info log file: {:?}",
            info_log_path
        ));

    let error_log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&error_log_path)
        .expect(&format!(
            "Failed to open error log file: {:?}",
            error_log_path
        ));

    // Define the formatter once
    let formatter =
        |out: fern::FormatCallback, message: &std::fmt::Arguments, record: &log::Record| {
            let log_entry = json!({
                "timestamp": chrono::Local::now().to_rfc3339(),
                "level": record.level().to_string(),
                "target": record.target(),
                "message": message.to_string()
            });
            out.finish(format_args!("{}", log_entry));
        };

    // Configure the logger for INFO level
    let info_config = Dispatch::new()
        .format(formatter)
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(info_log_file);

    // the logger for ERROR level
    let error_config = Dispatch::new()
        .format(formatter)
        .level(LevelFilter::Error)
        .chain(error_log_file);

    Dispatch::new()
        .level(LevelFilter::Debug)
        .chain(info_config)
        .chain(error_config)
        .apply()
        .unwrap();
}
