//! result_sync_log_step_test.rs – Logging tests for ResultStep
//!
//! Located under `result-transformer-test/src/step/sync/result/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-sync-log-step -- step::sync::result::result_sync_log_step_test --show-output
//! ────────────────────────────────────────────────────────────────

#[allow(unused_imports)]
use logtest::*;
#[allow(unused_imports)]
use result_transformer::__internal::log;
#[allow(unused_imports)]
use result_transformer::flow::sync::*;

/// ⚠ WARNING ⚠
/// This test module uses `logtest::Logger`, which internally calls `log::set_logger()`.
/// The `log` crate only allows setting the logger once globally; any subsequent call will panic.
///
/// Therefore, if multiple test functions call `Logger::start()`, tests may fail
/// regardless of whether they are run sequentially or in parallel.
#[test]
fn log_tap_result_step_emits_log() {
    let mut logger = logtest::Logger::start();
    let fmt: fn(&Result<i32, i32>) -> String = |r| format!("r={r:?}");

    let step = ResultLogTapStep::new(log::Level::Info, fmt);
    let out = step.apply_result(Ok(5));

    assert_eq!(out, Ok(5));
    assert_eq!(logger.len(), 1);
    let record = logger.pop().unwrap();
    assert_eq!(record.level(), log::Level::Info);
    assert_eq!(record.args(), "r=Ok(5)");
}

/// ⚠ WARNING ⚠
/// This test module uses `logtest::Logger`, which internally calls `log::set_logger()`.
/// The `log` crate only allows setting the logger once globally; any subsequent call will panic.
///
/// Therefore, if multiple test functions call `Logger::start()`, tests may fail
/// regardless of whether they are run sequentially or in parallel.
#[test]
fn log_both_result_step_emits_log() {
    let mut logger = logtest::Logger::start();
    let ok_fmt: fn(&i32) -> String = |r| format!("ok={r:?}");
    let err_fmt: fn(&i32) -> String = |r| format!("err={r:?}");

    let step = ResultLogTapBothStep::new(log::Level::Debug, ok_fmt, log::Level::Debug, err_fmt);
    let out = step.apply_result(Err(8));

    assert_eq!(out, Err(8));
    assert_eq!(logger.len(), 1);
    let record = logger.pop().unwrap();
    assert_eq!(record.level(), log::Level::Debug);
    assert_eq!(record.args(), "err=8");
}
