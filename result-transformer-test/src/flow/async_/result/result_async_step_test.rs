//! result_async_step_test.rs – Async tests for ResultStep implementations
//!
//! Located at `result-transformer-test/src/flow/async_/result`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-async --features flow-sync -- flow::async_::result::resultasync_step_test --show-output
//! ────────────────────────────────────────────────────────────────

#[allow(unused_imports)]
use result_transformer::flow::async_::*;
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::test]
async fn map_result_step_transforms_ok_and_err_async() {
    let step = ResultMapBothStep::new(|x: i32| format!("ok={x}"), |e: i32| format!("err={e}"));
    assert_eq!(step.apply_result_async(Ok(3)).await, Ok("ok=3".to_string()));
    assert_eq!(
        step.apply_result_async(Err(9)).await,
        Err("err=9".to_string())
    );
}

#[tokio::test]
async fn tap_result_step_transforms_and_logs_async() {
    let step = ResultTapBothStep::new(|x: i32| format!("O{x}"), |e: i32| format!("E{e}"));
    assert_eq!(step.apply_result_async(Ok(1)).await, Ok("O1".to_string()));
    assert_eq!(step.apply_result_async(Err(2)).await, Err("E2".to_string()));
}

#[tokio::test]
async fn inspect_result_step_side_effect_only_async() {
    static OK_COUNT: AtomicUsize = AtomicUsize::new(0);
    static ERR_COUNT: AtomicUsize = AtomicUsize::new(0);
    let step = ResultInspectBothStep::new(
        |_| {
            OK_COUNT.fetch_add(1, Ordering::SeqCst);
        },
        |_| {
            ERR_COUNT.fetch_add(1, Ordering::SeqCst);
        },
    );
    assert_eq!(step.apply_result_async(Ok(123)).await, Ok(123));
    assert_eq!(step.apply_result_async(Err("fail")).await, Err("fail"));
    assert_eq!(OK_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(ERR_COUNT.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn noop_result_step_identity_async() {
    let step = ResultNoopStep::<u8, &str>::new();
    assert_eq!(step.apply_result_async(Ok(100)).await, Ok(100));
    assert_eq!(step.apply_result_async(Err("err")).await, Err("err"));
}

#[cfg(feature = "flow-sync")]
#[allow(unused_imports)]
use result_transformer::flow::sync::step::ResultIfStep;

#[cfg(feature = "flow-sync")]
#[tokio::test]
async fn if_result_step_conditional_branching_async() {
    let even_flow = ResultMapBothStep::new(|x| x / 2, |e| e);
    let odd_flow = ResultMapBothStep::new(|x| x * 3 + 1, |e| e);
    let step = ResultIfStep::new(
        |res: &Result<i32, &str>| matches!(res, Ok(v) if v % 2 == 0),
        even_flow,
        odd_flow,
    );
    assert_eq!(step.apply_result_async(Ok(6)).await, Ok(3));
    assert_eq!(step.apply_result_async(Ok(5)).await, Ok(16));
    assert_eq!(step.apply_result_async(Err("e")).await, Err("e"));
}
