//! result_async_step_async_test.rs – Tests for async-specific Result steps
//!
//! Located at `result-transformer-test/src/flow/async_/result/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-async -- flow::async_::result::result_async_step_async_test --show-output
//! ────────────────────────────────────────────────────────────────

use result_transformer::__internal::*;
#[allow(unused_imports)]
use result_transformer::flow::async_::*;
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::test]
async fn map_result_step_async_transforms_ok_and_err() {
    let step = ResultMapBothStepAsync::new(
        |x: i32| Box::pin(async move { format!("ok={x}") }),
        |e: i32| Box::pin(async move { format!("err={e}") }),
    );
    assert_eq!(step.apply_result_async(Ok(3)).await, Ok("ok=3".to_string()));
    assert_eq!(step.apply_result_async(Err(9)).await, Err("err=9".to_string()));
}

#[tokio::test]
async fn tap_result_step_async_transforms_and_logs() {
    let step = ResultTapBothStepAsync::new(
        |x: i32| Box::pin(async move { format!("O{x}") }),
        |e: i32| Box::pin(async move { format!("E{e}") }),
    );
    assert_eq!(step.apply_result_async(Ok(1)).await, Ok("O1".to_string()));
    assert_eq!(step.apply_result_async(Err(2)).await, Err("E2".to_string()));
}

#[tokio::test]
async fn inspect_result_step_async_side_effect_only() {
    static OK_COUNT: AtomicUsize = AtomicUsize::new(0);
    static ERR_COUNT: AtomicUsize = AtomicUsize::new(0);
    let step = ResultInspectBothStepAsync::new(
        |_| {
            Box::pin(async {
                OK_COUNT.fetch_add(1, Ordering::SeqCst);
            })
        },
        |_| {
            Box::pin(async {
                ERR_COUNT.fetch_add(1, Ordering::SeqCst);
            })
        },
    );
    assert_eq!(step.apply_result_async(Ok(123)).await, Ok(123));
    assert_eq!(step.apply_result_async(Err("fail")).await, Err("fail"));
    assert_eq!(OK_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(ERR_COUNT.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn if_result_step_async_conditional_branching() {
    let even_flow = ResultMapBothStepAsync::new(
        |x| Box::pin(async move { x / 2 }),
        |e| Box::pin(async move { e }),
    );
    let odd_flow = ResultMapBothStepAsync::new(
        |x| Box::pin(async move { x * 3 + 1 }),
        |e| Box::pin(async move { e }),
    );
    let step = ResultIfStepAsync::new(
        |res: &Result<i32, &str>| {
            let r = *res;
            Box::pin(async move { matches!(r, Ok(v) if v % 2 == 0) })
        },
        even_flow,
        odd_flow,
    );
    assert_eq!(step.apply_result_async(Ok(6)).await, Ok(3));
    assert_eq!(step.apply_result_async(Ok(5)).await, Ok(16));
    assert_eq!(step.apply_result_async(Err("e")).await, Err("e"));
}

#[tokio::test]
async fn result_map_step_async_maps_both() {
    let step = ResultMapStepAsync::new(|r: Result<i32, i32>| {
        Box::pin(async move { r.map(|v| v + 1).map_err(|e| e - 1) })
    });
    assert_eq!(step.apply_result_async(Ok(1)).await, Ok(2));
    assert_eq!(step.apply_result_async(Err(3)).await, Err(2));
}

#[tokio::test]
async fn result_map_both_bind_step_async_maps() {
    let step = ResultMapBothBindStepAsync::new(
        |v: i32| Box::pin(async move { Ok(v * 2) }),
        |e: i32| Box::pin(async move { Err(e * 3) }),
    );
    assert_eq!(step.apply_result_async(Ok(2)).await, Ok(4));
    assert_eq!(step.apply_result_async(Err(3)).await, Err(9));
}

#[tokio::test]
async fn result_tap_step_async_transforms() {
    let step = ResultTapStepAsync::new(|r: Result<i32, i32>| {
        Box::pin(async move { r.map(|v| v + 1).map_err(|e| e + 1) })
    });
    assert_eq!(step.apply_result_async(Ok(1)).await, Ok(2));
    assert_eq!(step.apply_result_async(Err(2)).await, Err(3));
}

#[tokio::test]
async fn result_tap_both_step_async_transforms() {
    let step = ResultTapBothStepAsync::new(
        |x: i32| Box::pin(async move { x * 2 }),
        |e: i32| Box::pin(async move { e * 2 }),
    );
    assert_eq!(step.apply_result_async(Ok(4)).await, Ok(8));
    assert_eq!(step.apply_result_async(Err(5)).await, Err(10));
}

#[tokio::test]
async fn result_tap_both_bind_step_async_transforms() {
    let step = ResultTapBothBindStepAsync::new(
        |x: i32| Box::pin(async move { Ok(x + 1) }),
        |e: i32| Box::pin(async move { Err(e + 1) }),
    );
    assert_eq!(step.apply_result_async(Ok(1)).await, Ok(2));
    assert_eq!(step.apply_result_async(Err(2)).await, Err(3));
}
