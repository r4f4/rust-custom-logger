use chrono::{DateTime, Local};

// logging convenience functions
#[derive(Eq, PartialEq)]
pub enum Level {
    INFO,
    DEBUG,
    TRACE,
    WARN,
}

//#[derive(Default)]
pub struct Logging {
    log_level: Level,
}

impl Logging {
    fn timestamp(&self) -> String {
        let dt = Local::now();
        let dt_new = DateTime::<Local>::from_naive_utc_and_offset(dt.naive_utc(), *dt.offset());
        dt_new.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_level(mut self, level: Level) -> Self {
        self.log_level = level;
        self
    }

    // info
    pub fn info<S: std::fmt::Display>(&self, msg: S) {
        if self.log_level == Level::INFO
            || self.log_level == Level::DEBUG
            || self.log_level == Level::TRACE
        {
            println!(
                "\x1b[1;94m [ INFO {} ] \x1b[0m  : {}",
                self.timestamp(),
                msg
            );
        }
    }
    /// debug
    pub fn debug<S: std::fmt::Display>(&self, msg: S) {
        if self.log_level == Level::DEBUG || self.log_level == Level::TRACE {
            println!(
                "\x1b[1;92m [ DEBUG {} ] \x1b[0m  : {}",
                self.timestamp(),
                msg
            );
        }
    }
    /// info with highlight
    pub fn hi<S: std::fmt::Display>(&self, msg: S) {
        if self.log_level == Level::INFO
            || self.log_level == Level::DEBUG
            || self.log_level == Level::TRACE
        {
            println!(
                "\x1b[1;94m [ INFO  {} ] \x1b[0m  : \x1b[1;93m{} \x1b[0m",
                self.timestamp(),
                msg
            );
        }
    }
    /// info with mid level highlight
    pub fn mid<S: std::fmt::Display>(&self, msg: S) {
        if self.log_level == Level::INFO
            || self.log_level == Level::DEBUG
            || self.log_level == Level::TRACE
        {
            println!(
                "\x1b[1;94m [ INFO  {} ]  \x1b[0m : \x1b[1;94m{} \x1b[0m",
                self.timestamp(),
                msg
            );
        }
    }
    // info with low level highlight
    pub fn lo<S: std::fmt::Display>(&self, msg: S) {
        if self.log_level == Level::INFO
            || self.log_level == Level::DEBUG
            || self.log_level == Level::TRACE
        {
            println!(
                "\x1b[1;94m [ INFO  {} ]  \x1b[0m : \x1b[1;95m{} \x1b[0m",
                self.timestamp(),
                msg
            );
        }
    }
    // info with extra level highlight
    pub fn ex<S: std::fmt::Display>(&self, msg: S) {
        if self.log_level == Level::INFO
            || self.log_level == Level::DEBUG
            || self.log_level == Level::TRACE
        {
            println!(
                "\x1b[1;94m [ INFO  {} ]  \x1b[0m : \x1b[1;98m{} \x1b[0m",
                self.timestamp(),
                msg
            );
        }
    }
    /// trace
    pub fn trace<S: std::fmt::Display>(&self, msg: S) {
        if self.log_level == Level::TRACE {
            println!(
                "\x1b[1;96m [ TRACE {} ] \x1b[0m  : {}",
                self.timestamp(),
                msg
            );
        }
    }
    /// warning
    pub fn warn<S: std::fmt::Display>(&self, msg: S) {
        println!(
            "\x1b[1;93m [ WARN  {} ] \x1b[0m  : {}",
            self.timestamp(),
            msg
        );
    }
    /// error
    pub fn error<S: std::fmt::Display>(&self, msg: S) {
        println!(
            "\x1b[1;91m [ ERROR {} ] \x1b[0m  : {}",
            self.timestamp(),
            msg
        );
    }
}

impl Default for Logging {
    fn default() -> Self {
        Self {
            log_level: Level::INFO,
        }
    }
}

#[cfg(test)]
mod tests {
    // this brings everything from parent's scope into this scope
    use super::*;

    #[test]
    fn test_info_pass() {
        let log = Logging::new();
        log.info("testing info logging");
    }

    #[test]
    fn test_debug_pass() {
        let log = Logging::new().with_level(Level::DEBUG);
        log.debug("testing debug logging");
    }

    #[test]
    fn test_trace_pass() {
        let log = Logging::new().with_level(Level::TRACE);
        log.trace("testing trace logging");
    }

    #[test]
    fn test_warn_pass() {
        let log = Logging::new().with_level(Level::WARN);
        log.warn("testing warn logging");
    }

    #[test]
    fn test_mid_pass() {
        let log = Logging::new().with_level(Level::INFO);
        log.mid("testing mid logging");
    }

    #[test]
    fn test_hi_pass() {
        let log = Logging::new();
        log.hi("testing hi logging");
    }

    #[test]
    fn test_lo_pass() {
        let log = Logging::new();
        log.lo("testing lo logging");
    }

    #[test]
    fn test_ex_pass() {
        let log = Logging::new();
        log.ex("testing ex logging");
    }

    #[test]
    fn test_error_pass() {
        let log = Logging::new();
        log.error("testing error logging");
    }

    // set level to TRACE
    #[test]
    fn test_mid_with_trace_pass() {
        let log = Logging::new().with_level(Level::TRACE);
        log.mid("testing mid logging with TRACE");
    }

    #[test]
    fn test_hi_with_trace_pass() {
        let log = Logging::new().with_level(Level::TRACE);
        log.hi("testing hi logging with TRACE");
    }

    #[test]
    fn test_display_types() {
        let log = Logging::new();
        log.info(format!("testing logging with format: {}", true));
        log.info(String::from("testing logging with String"));
        log.info(1);
        log.info('a');

        struct Custom {
            value1: u8,
            value2: f32,
        }

        impl std::fmt::Display for Custom {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "[{} {}]", self.value1, self.value2)
            }
        }
        log.info(Custom {
            value1: 42,
            value2: 0.07,
        });
    }
}
