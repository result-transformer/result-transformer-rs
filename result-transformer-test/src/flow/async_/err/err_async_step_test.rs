//! err_async_step_test.rs – Async tests for ErrStep implementations
//!
//! Located at `result-transformer-test/src/flow/async_/err/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-async --features flow-sync -- flow::async_::err::err_async_step_test --show-output
//! ────────────────────────────────────────────────────────────────

#[allow(unused_imports)]
use result_transformer::flow::async_::*;
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::test]
async fn map_err_step_transforms_async() {
    let step = ErrMapStep::new(|x: i32| x * 2);
    assert_eq!(step.apply_err_async(21).await, 42);
}

#[tokio::test]
async fn tap_err_step_works_async() {
    let step = ErrTapStep::new(|x: &str| format!("Hello, {x}!"));
    assert_eq!(step.apply_err_async("Rust").await, "Hello, Rust!");
}

#[tokio::test]
async fn inspect_err_step_side_effect_only_async() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    let step = ErrInspectStep::new(|_: &i32| {
        CALLS.fetch_add(1, Ordering::SeqCst);
    });
    let out = step.apply_err_async(7).await;
    assert_eq!(out, 7);
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn noop_err_step_identity_async() {
    let step = ErrNoopStep::<u8>::new();
    assert_eq!(step.apply_err_async(255).await, 255);
}

#[cfg(feature = "flow-sync")]
#[allow(unused_imports)]
use result_transformer::flow::sync::step::ErrIfStep;

#[cfg(feature = "flow-sync")]
#[tokio::test]
async fn if_err_step_branches_async() {
    let even_flow = ErrMapStep::new(|x: i32| x / 2);
    let odd_flow = ErrMapStep::new(|x: i32| x * 3 + 1);
    let branch = ErrIfStep::new(|x: &i32| x % 2 == 0, even_flow, odd_flow);
    assert_eq!(branch.apply_err_async(10).await, 5);
    assert_eq!(branch.apply_err_async(11).await, 34);
}
