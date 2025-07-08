use result_transformer_dependencies::*;

/// Configuration used by log tap steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LogConfig<T> {
    log_level: log::Level,
    log_format: fn(&T) -> String,
}

impl<T> LogConfig<T> {
    /// Creates a new configuration.
    ///
    /// * `log_level` - log level to emit
    /// * `log_format` - function that formats the value into a log string
    pub fn new(log_level: log::Level, log_format: fn(&T) -> String) -> Self {
        Self {
            log_level,
            log_format,
        }
    }

    /// Returns the configured log level.
    pub fn log_level(&self) -> &log::Level {
        &self.log_level
    }

    /// Returns a reference to the log formatting function.
    pub fn log_format(&self) -> &fn(&T) -> String {
        &self.log_format
    }
}
