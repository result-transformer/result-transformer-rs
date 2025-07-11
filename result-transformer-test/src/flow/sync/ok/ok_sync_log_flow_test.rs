//!
//! Located under `result-transformer-test/src/flow/sync/ok/`.
//! These tests validate logging behavior of log-capable OkStep implementations using the public API of **result-transformer**.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-sync-log-step -- flow::sync::ok::ok_sync_log_step_test --show-output
//! ────────────────────────────────────────────────────────────────

#[allow(unused_imports)]
use logtest::*;
#[allow(unused_imports)]
use result_transformer::__internal::log;
#[allow(unused_imports)]
use result_transformer::flow::sync::*;

/// `TapLogOkStep` should emit exactly one log record containing the
/// user-formatted message at the requested log level.
#[test]
#[serial_test::serial]
fn tap_log_ok_step_emits_log() {
    let mut logger = crate::helper::logtest_helper::get_logger();
    let fmt: fn(&i32) -> String = |v| format!("val={v}");

    let step = OkLogTapStep::new(log::Level::Info, fmt);
    let out = step.apply_ok(5);

    // value is unchanged
    assert_eq!(out, 5);

    // exactly one log entry with the expected contents
    assert_eq!(logger.len(), 1);
    let record = logger.pop().unwrap();
    assert_eq!(record.level(), log::Level::Info);
    assert_eq!(record.args(), "val=5");
}
