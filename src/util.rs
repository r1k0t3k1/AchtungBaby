use std::fs::OpenOptions;
use std::io::Write;

pub struct Logger;

const LOG_FILE: &str = r"C:\Temp\AchthungBaby.log";

impl Logger {
    pub fn log(message: &str) {
        if let Ok(mut f) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(LOG_FILE) {
                let _ = writeln!(f, "{}", message);
            }
    }
}