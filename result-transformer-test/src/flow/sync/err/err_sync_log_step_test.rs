//! err_sync_log_step_test.rs – Logging tests for ErrStep
//!
//! Located under `result-transformer-test/src/flow/sync/err/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-sync-log-step -- flow::sync::err::err_sync_log_step_test --show-output
//! ────────────────────────────────────────────────────────────────

#[allow(unused_imports)]
use logtest::*;
#[allow(unused_imports)]
use result_transformer::__internal::log;
#[allow(unused_imports)]
use result_transformer::flow::sync::*;

#[test]
#[serial_test::serial]
fn tap_log_err_step_emits_log() {
    let mut logger = crate::helper::logtest_helper::get_logger();
    let fmt: fn(&i32) -> String = |v| format!("err={v}");

    let step = ErrLogTapStep::new(log::Level::Warn, fmt);
    let out = step.apply_err(7);

    assert_eq!(out, 7);
    assert_eq!(logger.len(), 1);
    let record = logger.pop().unwrap();
    assert_eq!(record.level(), log::Level::Warn);
    assert_eq!(record.args(), "err=7");
}
