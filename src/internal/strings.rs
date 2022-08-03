use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};

use clap::Parser;
use colored::Colorize;

use crate::internal::AppExitCode;

const ERR_SYMBOL: &str = "✘";
const WARN_SYMBOL: &str = "⚠";

#[macro_export]
macro_rules! log {
    ($($arg:tt)+) => {
        $crate::internal::log_fn(&format!("[{}:{}] {}", file!(), line!(), format!($($arg)+)));
    }
}

pub fn log_fn(msg: &str) {
    if crate::args::Args::parse().verbose.unwrap_or(false) {
        eprintln!(
            "{} {}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            msg
        );
    }
}

#[macro_export]
macro_rules! crash {
    ($exit_code:expr, $($arg:tt)+) => {
        $crate::internal::crash_fn($exit_code, &format!("[{}:{}] {}", file!(), line!(), format!($($arg)+)));
    }
}

pub fn crash_fn(exit_code: AppExitCode, msg: &str) {
    println!("{} {}", ERR_SYMBOL.bold().red(), msg.bold().bright_red());
    exit(exit_code as i32);
}

pub fn warn(bin: &str, dist: &str, pman: &str) {
    println!(
        "{} {} is not supported on {}. Please use {} instead!",
        WARN_SYMBOL.bold().yellow(),
        bin.bold().yellow(),
        dist.bold(),
        pman.bold(),
    );
    exit(AppExitCode::Success as i32);
}
