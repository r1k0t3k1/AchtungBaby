use std::fs::OpenOptions;
use std::io::Write;

use windows::Win32::System::Diagnostics::Debug::OutputDebugStringW;
use windows_core::HSTRING;

pub struct Logger;

const LOG_FILE: &str = r"C:\Temp\AchthungBaby.log";

impl Logger {
    pub fn log_to_file(message: &str) {
        if let Ok(mut f) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(LOG_FILE) {
                let _ = writeln!(f, "{}", message);
            }
    }

    pub fn debug_log(message: &str) {
        let wide_msg = HSTRING::from(message);
        unsafe {
            OutputDebugStringW(&wide_msg);
        }
    }
}