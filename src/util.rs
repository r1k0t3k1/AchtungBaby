use capstone::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;

use windows::Win32::System::Diagnostics::Debug::OutputDebugStringW;
use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;
use windows_core::{w, HSTRING};

pub struct Logger;

const LOG_FILE: &str = r"C:\Temp\AchthungBaby.log";

impl Logger {
    pub fn log_to_file(message: &str) {
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(LOG_FILE) {
            let _ = writeln!(f, "{}", message);
        }
    }

    pub fn log_to_debugger(message: &str) {
        let wide_msg = HSTRING::from(message);
        unsafe {
            OutputDebugStringW(&wide_msg);
        }
    }

    pub fn log_to_message_window(message: &str) {
        let title = w!("AchtungBaby Debug message");
        let wide_msg = HSTRING::from(message);
        unsafe {
            MessageBoxW(None, &wide_msg, title, Default::default());
        }
    }

    pub fn show_disasm(binary: &[u8]) {
        let cs = Capstone::new()
            .x86()
            .mode(arch::x86::ArchMode::Mode64)
            .syntax(arch::x86::ArchSyntax::Intel)
            .detail(true)
            .build()
            .unwrap();

        cs.disasm_all(binary, 0)
            .unwrap()
            .as_ref()
            .iter()
            .for_each(|i| {
                println!("{}", i);
            });
    }
}
