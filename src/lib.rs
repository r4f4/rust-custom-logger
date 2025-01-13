use chrono::{DateTime, Local};

//#[derive(Default)]
pub struct Logging {
    log_level: log::LevelFilter,
}

// Re-export so they can be used from the same crate
pub use log::{debug, error, info, trace, warn, LevelFilter};

#[macro_export]
macro_rules! hi {
    ($($arg:tt)+) => ({
        let msg = format_args!($($arg)+);
        log::info!("{}", $crate::Logging::hi(msg.as_str().unwrap_or("")));
    })
}

#[macro_export]
macro_rules! mid {
    ($($arg:tt)+) => ({
        let msg = format_args!($($arg)+);
        log::info!("{}", $crate::Logging::mid(msg.as_str().unwrap_or("")));
    })
}

#[macro_export]
macro_rules! lo {
    ($($arg:tt)+) => ({
        let msg = format_args!($($arg)+);
        log::info!("{}", $crate::Logging::lo(msg.as_str().unwrap_or("")));
    })
}

#[macro_export]
macro_rules! ex {
    ($($arg:tt)+) => ({
        let msg = format_args!($($arg)+);
        log::info!("{}", $crate::Logging::ex(msg.as_str().unwrap_or("")));
    })
}

impl log::Log for Logging {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level().to_level_filter() <= self.log_level
    }

    fn log(&self, record: &log::Record) {
        let dt = Local::now();
        let dt_new = DateTime::<Local>::from_naive_utc_and_offset(dt.naive_utc(), *dt.offset());
        let dt_formated = dt_new.format("%Y-%m-%d %H:%M:%S%.3f");

        let level_log = match record.level() {
            log::Level::Info => format!("\x1b[1;94m [ INFO  {} ] \x1b[0m", dt_formated),
            log::Level::Debug => format!("\x1b[1;92m [ DEBUG {} ] \x1b[0m", dt_formated),
            log::Level::Trace => format!("\x1b[1;96m [ TRACE {} ] \x1b[0m", dt_formated),
            log::Level::Warn => format!("\x1b[1;93m [ WARN  {} ] \x1b[0m", dt_formated),
            log::Level::Error => format!("\x1b[1;91m [ ERROR {} ] \x1b[0m", dt_formated),
        };

        println!("{} : {}", level_log, record.args());
    }

    fn flush(&self) {}
}

impl Logging {
    #[allow(clippy::new_without_default)]
    #[must_use = "You must call init() to begin logging"]
    pub fn new() -> Self {
        Self {
            log_level: LevelFilter::Info,
        }
    }

    #[must_use = "You must call init() to begin logging"]
    pub fn with_level(mut self, level: LevelFilter) -> Self {
        self.log_level = level;
        self
    }

    pub fn init(self) -> Result<(), log::SetLoggerError> {
        log::set_max_level(self.log_level);
        log::set_boxed_logger(Box::new(self))
    }

    pub fn hi(s: &str) -> String {
        format!("\x1b[1;93m{} \x1b[0m", s)
    }

    pub fn mid(s: &str) -> String {
        format!("\x1b[1;94m{} \x1b[0m", s)
    }

    pub fn lo(s: &str) -> String {
        format!("\x1b[1;95m{} \x1b[0m", s)
    }

    pub fn ex(s: &str) -> String {
        format!("\x1b[1;98m{} \x1b[0m", s)
    }
}

#[cfg(test)]
mod tests {
    use log::{Level, Log};

    // this brings everything from parent's scope into this scope
    use super::*;

    #[test]
    fn test_logging() {
        let log = Logging::new().with_level(LevelFilter::Trace);
        log.init().expect("should initialize");
        info!("testing info logging");
        debug!("testing debug logging");
        trace!("testing trace logging");
        warn!("testing warn logging");
        error!("testing error logging");
        mid!("testing mid logging");
        hi!("testing hi logging");
        lo!("testing lo logging");
        ex!("testing ex logging");
    }

    #[test]
    fn test_info_pass() {
        let log = Logging::new().with_level(LevelFilter::Info);
        assert!(log.enabled(&log::Metadata::builder().level(Level::Info).build()));
        assert!(log.enabled(&log::Metadata::builder().level(Level::Error).build()));
        assert!(log.enabled(&log::Metadata::builder().level(Level::Warn).build()));
        assert!(!log.enabled(&log::Metadata::builder().level(Level::Debug).build()));
        assert!(!log.enabled(&log::Metadata::builder().level(Level::Trace).build()));
    }

    #[test]
    fn test_debug_pass() {
        let log = Logging::new().with_level(LevelFilter::Debug);
        assert!(log.enabled(&log::Metadata::builder().level(Level::Debug).build()));
        assert!(log.enabled(&log::Metadata::builder().level(Level::Info).build()));
        assert!(log.enabled(&log::Metadata::builder().level(Level::Error).build()));
        assert!(log.enabled(&log::Metadata::builder().level(Level::Warn).build()));
        assert!(!log.enabled(&log::Metadata::builder().level(Level::Trace).build()));
    }

    #[test]
    fn test_trace_pass() {
        let log = Logging::new().with_level(LevelFilter::Trace);
        assert!(log.enabled(&log::Metadata::builder().level(Level::Trace).build()));
        assert!(log.enabled(&log::Metadata::builder().level(Level::Info).build()));
        assert!(log.enabled(&log::Metadata::builder().level(Level::Error).build()));
        assert!(log.enabled(&log::Metadata::builder().level(Level::Warn).build()));
        assert!(log.enabled(&log::Metadata::builder().level(Level::Debug).build()));
    }

    #[test]
    fn test_warn_pass() {
        let log = Logging::new().with_level(LevelFilter::Warn);
        assert!(log.enabled(&log::Metadata::builder().level(Level::Warn).build()));
        assert!(log.enabled(&log::Metadata::builder().level(Level::Error).build()));
        assert!(!log.enabled(&log::Metadata::builder().level(Level::Info).build()));
        assert!(!log.enabled(&log::Metadata::builder().level(Level::Trace).build()));
    }

    #[test]
    fn test_error_pass() {
        let log = Logging::new().with_level(LevelFilter::Error);
        assert!(log.enabled(&log::Metadata::builder().level(Level::Error).build()));
        assert!(!log.enabled(&log::Metadata::builder().level(Level::Warn).build()));
        assert!(!log.enabled(&log::Metadata::builder().level(Level::Info).build()));
        assert!(!log.enabled(&log::Metadata::builder().level(Level::Debug).build()));
        assert!(!log.enabled(&log::Metadata::builder().level(Level::Trace).build()));
    }
}
