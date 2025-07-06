use result_transformer_dependencies::*;

/// Configuration used by logging steps.
pub struct LogConfig<T> {
    log_level: log::Level,
    log_format: fn(&T) -> String,
}

impl<T> LogConfig<T> {
    pub fn new(log_level: log::Level, log_format: fn(&T) -> String) -> Self {
        Self {
            log_level,
            log_format,
        }
    }

    pub fn log_level(&self) -> &log::Level {
        &self.log_level
    }

    pub fn log_format(&self) -> &fn(&T) -> String {
        &self.log_format
    }
}
