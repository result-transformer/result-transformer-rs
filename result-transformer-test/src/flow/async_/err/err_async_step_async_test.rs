//! err_async_step_async_test.rs – Tests for async-specific Err steps
//!
//! Located at `result-transformer-test/src/flow/async_/err/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-async -- flow::async_::err::err_async_step_async_test --show-output
//! ────────────────────────────────────────────────────────────────

use result_transformer::__internal::*;
#[allow(unused_imports)]
use result_transformer::flow::async_::*;
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::test]
async fn map_err_step_async_transforms() {
    let step = ErrMapStepAsync::new(|x: i32| Box::pin(async move { x * 2 }));
    assert_eq!(step.apply_err_async(21).await, 42);
}

#[tokio::test]
async fn tap_err_step_async_works() {
    let step = ErrTapStepAsync::new(|x: &str| Box::pin(async move { format!("Hello, {x}!") }));
    assert_eq!(step.apply_err_async("Rust").await, "Hello, Rust!");
}

#[tokio::test]
async fn inspect_err_step_async_side_effect_only() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    let step = ErrInspectStepAsync::new(|_: &i32| {
        Box::pin(async move {
            CALLS.fetch_add(1, Ordering::SeqCst);
        })
    });
    let out = step.apply_err_async(7).await;
    assert_eq!(out, 7);
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn if_err_step_async_branches() {
    let even_flow = ErrMapStepAsync::new(|x: i32| Box::pin(async move { x / 2 }));
    let odd_flow = ErrMapStepAsync::new(|x: i32| Box::pin(async move { x * 3 + 1 }));
    let branch = ErrIfStepAsync::new(
        |x: &i32| {
            let val = *x;
            Box::pin(async move { val % 2 == 0 })
        },
        even_flow,
        odd_flow,
    );
    assert_eq!(branch.apply_err_async(10).await, 5);
    assert_eq!(branch.apply_err_async(11).await, 34);
}
