use logtest::Logger;
use once_cell::sync::OnceCell;
use std::sync::{Mutex, MutexGuard};

/// Global logger instance that is initialized only once.
/// Wrapped in a `Mutex` to ensure safe concurrent access during tests.
#[allow(unused)]
static LOGGER: OnceCell<Mutex<Logger>> = OnceCell::new();

/// Retrieves a locked reference to the global logger.
///
/// On the first call, it initializes the logger using `Logger::start()`.
/// Subsequent calls reuse the same logger instance.
///
/// The returned `MutexGuard` ensures exclusive access until it is dropped,
/// preventing log messages from different tests from interfering with each other.
///
/// # Panics
/// Panics if the mutex is poisoned or cannot be locked.
#[allow(unused)]
pub fn get_logger() -> MutexGuard<'static, Logger> {
    LOGGER
        .get_or_init(|| {
            let logger = Logger::start();
            Mutex::new(logger)
        })
        .lock()
        .expect("Failed to acquire logger lock")
}
