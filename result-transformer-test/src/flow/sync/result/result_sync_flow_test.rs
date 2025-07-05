//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --features flow-sync -- --show-output
//! ────────────────────────────────────────────────────────────────

#[allow(unused_imports)]
use result_transformer::__internal::log;
#[allow(unused_imports)]
use result_transformer::flow::sync::*;
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};

/// `MapResultStep` transforms both Ok and Err variants.
#[test]
fn map_result_step_transforms_ok_and_err() {
    let step = ResultMapBothStep::new(
        |x: i32| format!("ok={x}"),
        |e: i32| format!("err={e}"),
    );

    assert_eq!(step.apply_result(Ok(3)), Ok("ok=3".to_string()));
    assert_eq!(step.apply_result(Err(9)), Err("err=9".to_string()));
}

/// `TapResultStep` allows mutating both Ok and Err values.
#[test]
fn tap_result_step_transforms_and_logs() {
    let step = ResultTapStep::new(
        |x: i32| format!("O{x}"),
        |e: i32| format!("E{e}"),
    );

    assert_eq!(step.apply_result(Ok(1)), Ok("O1".to_string()));
    assert_eq!(step.apply_result(Err(2)), Err("E2".to_string()));
}

/// `InspectResultStep` performs side effects without modifying values.
#[test]
fn inspect_result_step_side_effect_only() {
    static OK_COUNT: AtomicUsize = AtomicUsize::new(0);
    static ERR_COUNT: AtomicUsize = AtomicUsize::new(0);

    let step = ResultInspectStep::new(
        |_| {
            OK_COUNT.fetch_add(1, Ordering::SeqCst);
        },
        |_| {
            ERR_COUNT.fetch_add(1, Ordering::SeqCst);
        },
    );

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
    let even_flow = ResultMapStep::new(|x| x / 2, |e| e);
    let odd_flow = ResultMapStep::new(|x| x * 3 + 1, |e| e);

    let step = ResultIfStep::new(|x: &i32| x % 2 == 0, even_flow, odd_flow);

    assert_eq!(step.apply_result(Ok(6)), Ok(3));
    assert_eq!(step.apply_result(Ok(5)), Ok(16));
    assert_eq!(step.apply_result(Err("e")), Err("e"));
}

/// `then_result` chains multiple ResultFlow transformations.
#[test]
fn result_flow_then_chaining() {
    let f1 = ResultMapStep::new(|x: i32| x + 1, |e| e);
    let f2 = ResultMapStep::new(|x: i32| x * 2, |e| format!("E:{}", e));

    let flow = f1.then_result(f2);

    assert_eq!(flow.apply_result(Ok(4)), Ok(10));
    assert_eq!(flow.apply_result(Err("oops")), Err("E:oops".to_string()));
}
