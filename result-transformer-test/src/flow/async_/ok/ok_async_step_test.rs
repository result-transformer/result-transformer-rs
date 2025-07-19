//! ok_async_step_test.rs – Async tests for OkStep implementations
//!
//! Located at `result-transformer-test/src/flow/async_/ok/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-async --features flow-sync -- flow::async_::ok::ok_async_step_test --show-output
//! ────────────────────────────────────────────────────────────────

#[allow(unused_imports)]
use result_transformer::flow::async_::*;
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::test]
async fn map_ok_step_transforms_async() {
    let step = OkMapStep::new(|x: i32| x * 2);
    assert_eq!(step.apply_ok_async(21).await, 42);
}

#[tokio::test]
async fn tap_ok_step_works_async() {
    let step = OkTapStep::new(|x: &str| format!("Hello, {x}!"));
    assert_eq!(step.apply_ok_async("Rust").await, "Hello, Rust!");
}

#[tokio::test]
async fn inspect_ok_step_side_effect_only_async() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    let step = OkInspectStep::new(|_: &i32| {
        CALLS.fetch_add(1, Ordering::SeqCst);
    });
    let out = step.apply_ok_async(7).await;
    assert_eq!(out, 7);
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn noop_ok_step_identity_async() {
    let step = OkNoopStep::<u8>::new();
    assert_eq!(step.apply_ok_async(255).await, 255);
}

#[cfg(feature = "flow-sync")]
#[allow(unused_imports)]
use result_transformer::flow::sync::step::OkIfStep;

#[cfg(feature = "flow-sync")]
#[tokio::test]
async fn if_ok_step_branches_async() {
    let even_flow = OkMapStep::new(|x: i32| x / 2);
    let odd_flow = OkMapStep::new(|x: i32| x * 3 + 1);
    let branch = OkIfStep::new(|x: &i32| x % 2 == 0, even_flow, odd_flow);
    assert_eq!(branch.apply_ok_async(10).await, 5);
    assert_eq!(branch.apply_ok_async(11).await, 34);
}
