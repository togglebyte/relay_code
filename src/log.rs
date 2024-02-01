use std::env;

pub static mut LOG_LEVEL: bool = false;
pub fn set_log() {
    let logging_enabled = matches!(
        env::var("RUST_LOG").unwrap_or_default().as_str(),
        "TRACE" | "DEBUG" | "INFO" | "WARN"
    );
    // SAFETY: because I said so!
    unsafe { LOG_LEVEL = logging_enabled }
}

macro_rules! log {
    ($($tt:tt)*) => {
        {if unsafe{$crate::log::LOG_LEVEL} {
            eprintln!($($tt)*);
        }}
    };
}
