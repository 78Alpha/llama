extern crate lgl;
extern crate log;

use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};

struct UILogger;

static TRACE_ENABLED: AtomicBool = ATOMIC_BOOL_INIT;

impl log::Log for UILogger {
    fn enabled(&self, _: &log::LogMetadata) -> bool {
        true
    }

    fn log(&self, record: &log::LogRecord) {
        if !self.enabled(record.metadata()) { return }

        if record.level() > log::LogLevel::Debug && !TRACE_ENABLED.load(Ordering::SeqCst) { return }

        let string = format!("{}: {}\n", record.level(), record.args());
        lgl::log(&string);
    }
}

pub fn init() -> Result<(), log::SetLoggerError> {
    log::set_logger(|max| {
        max.set(log::LogLevelFilter::Trace);
        Box::new(UILogger)
    })
}

pub fn allow_trace(yes: bool) {
    TRACE_ENABLED.store(yes, Ordering::SeqCst);
}
