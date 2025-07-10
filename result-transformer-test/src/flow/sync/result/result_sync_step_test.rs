//! result_sync_step_test.rs – Unit tests for ResultStep implementations
//!
//! Located at `result-transformer-test/src/flow/sync/result/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-sync -- flow::sync::result::result_sync_step_test --show-output
//! ────────────────────────────────────────────────────────────────

#[allow(unused_imports)]
use result_transformer::flow::sync::*;
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};

/// `MapResultStep` transforms both Ok and Err variants.
#[test]
fn map_result_step_transforms_ok_and_err() {
    let step = ResultMapBothStep::new(|x: i32| format!("ok={x}"), |e: i32| format!("err={e}"));
    assert_eq!(step.apply_result(Ok(3)), Ok("ok=3".to_string()));
    assert_eq!(step.apply_result(Err(9)), Err("err=9".to_string()));
}

/// `TapResultStep` allows mutating both Ok and Err values.
#[test]
fn tap_result_step_transforms_and_logs() {
    let step = ResultTapStep::new(|res: Result<i32, i32>| res.map(|x| x + 1).map_err(|e| e + 1));
    assert_eq!(step.apply_result(Ok(1)), Ok(2));
    assert_eq!(step.apply_result(Err(2)), Err(3));
}

/// `InspectResultStep` performs side effects without modifying values.
#[test]
fn inspect_result_step_side_effect_only() {
    static OK_COUNT: AtomicUsize = AtomicUsize::new(0);
    static ERR_COUNT: AtomicUsize = AtomicUsize::new(0);
    let step = ResultInspectStep::new(|r: &Result<i32, &str>| match r {
        Ok(_) => {
            OK_COUNT.fetch_add(1, Ordering::SeqCst);
        }
        Err(_) => {
            ERR_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    });
    assert_eq!(step.apply_result(Ok(123)), Ok(123));
    assert_eq!(step.apply_result(Err("fail")), Err("fail"));
    assert_eq!(OK_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(ERR_COUNT.load(Ordering::SeqCst), 1);
}

/// `NoopResultStep` passes values without transformation.
#[test]
fn noop_result_step_identity() {
    let step = ResultNoopStep::<u8, &str>::new();
    assert_eq!(step.apply_result(Ok(100)), Ok(100));
    assert_eq!(step.apply_result(Err("err")), Err("err"));
}

/// `IfResultStep` chooses between two flows based on a predicate on Ok value.
#[test]
fn if_result_step_conditional_branching() {
    let even_flow = ResultMapStep::new(|r: Result<i32, &str>| r.map(|x| x / 2));
    let odd_flow = ResultMapStep::new(|r: Result<i32, &str>| r.map(|x| x * 3 + 1));
    let step = ResultIfStep::new(
        |res: &Result<i32, &str>| matches!(res, Ok(v) if v % 2 == 0),
        even_flow,
        odd_flow,
    );
    assert_eq!(step.apply_result(Ok(6)), Ok(3));
    assert_eq!(step.apply_result(Ok(5)), Ok(16));
    assert_eq!(step.apply_result(Err("e")), Err("e"));
}

/// `ResultMapStep` maps the entire Result at once.
#[test]
fn result_map_step_maps_both() {
    let step = ResultMapStep::new(|r: Result<i32, i32>| r.map(|v| v + 1).map_err(|e| e - 1));
    assert_eq!(step.apply_result(Ok(1)), Ok(2));
    assert_eq!(step.apply_result(Err(3)), Err(2));
}

/// `ResultMapBothBindStep` returns new Results from closures.
#[test]
fn result_map_both_bind_step_maps() {
    let step = ResultMapBothBindStep::new(|v: i32| Ok(v * 2), |e: i32| Err(e * 3));
    assert_eq!(step.apply_result(Ok(2)), Ok(4));
    assert_eq!(step.apply_result(Err(3)), Err(9));
}

/// `ResultTapBothStep` applies separate closures without changing Ok/Err wrappers.
#[test]
fn result_tap_both_step_transforms() {
    let step = ResultTapBothStep::new(|x: i32| x * 2, |e: i32| e * 2);
    assert_eq!(step.apply_result(Ok(4)), Ok(8));
    assert_eq!(step.apply_result(Err(5)), Err(10));
}

/// `ResultTapBothBindStep` applies closures returning Results.
#[test]
fn result_tap_both_bind_step_transforms() {
    let step = ResultTapBothBindStep::new(|x: i32| Ok(x + 1), |e: i32| Err(e + 1));
    assert_eq!(step.apply_result(Ok(1)), Ok(2));
    assert_eq!(step.apply_result(Err(2)), Err(3));
}

/// `ResultInspectBothStep` runs side effects on Ok and Err values.
#[test]
fn result_inspect_both_step_side_effect_only() {
    static OKS: AtomicUsize = AtomicUsize::new(0);
    static ERRS: AtomicUsize = AtomicUsize::new(0);
    let step = ResultInspectBothStep::new(
        |_| {
            OKS.fetch_add(1, Ordering::SeqCst);
        },
        |_| {
            ERRS.fetch_add(1, Ordering::SeqCst);
        },
    );
    assert_eq!(step.apply_result(Ok(0)), Ok(0));
    assert_eq!(step.apply_result(Err(0)), Err(0));
    assert_eq!(OKS.load(Ordering::SeqCst), 1);
    assert_eq!(ERRS.load(Ordering::SeqCst), 1);
}
