use cstr::cstr;
use log::{Level, Metadata, Record};

#[derive(Debug)]
struct ObsLogger;

static LOGGER: ObsLogger = ObsLogger;

impl log::Log for ObsLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = match record.level() {
                Level::Error => obs_sys::LOG_ERROR,
                Level::Warn => obs_sys::LOG_WARNING,
                Level::Info => obs_sys::LOG_INFO,
                Level::Debug => obs_sys::LOG_DEBUG,
                Level::Trace => obs_sys::LOG_DEBUG,
            };
            let message = format!("[{}] {}\0", record.target(), record.args());
            unsafe {
                obs_sys::blog(
                    level as i32,
                    cstr!("%s").as_ptr(),
                    std::ffi::CStr::from_bytes_with_nul_unchecked(message.as_bytes()).as_ptr(),
                );
            }
        }
    }

    fn flush(&self) {}
}

pub fn init() {
    if let Ok(()) = log::set_logger(&LOGGER) {
        log::set_max_level(log::LevelFilter::Debug);
        log_panics::init();
    };
}
