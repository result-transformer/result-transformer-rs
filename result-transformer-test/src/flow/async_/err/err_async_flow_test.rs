//! err_async_flow_test.rs – Async unit tests for ErrFlow steps

use result_transformer::__internal::*;
#[allow(unused_imports)]
use result_transformer::flow::async_::*;
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::test]
async fn map_err_step_transforms_async() {
    let step = ErrMapStep::new(|x: i32| x * 2);
    assert_eq!(step.apply_err(21).await, 42);
}

#[tokio::test]
async fn tap_err_step_works_async() {
    let step = ErrTapStep::new(|x: &str| format!("Hello, {x}!"));
    assert_eq!(step.apply_err("Rust").await, "Hello, Rust!");
}

#[tokio::test]
async fn inspect_err_step_side_effect_only_async() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    let step = ErrInspectStep::new(|_: &i32| {
        CALLS.fetch_add(1, Ordering::SeqCst);
    });
    let out = step.apply_err(7).await;
    assert_eq!(out, 7);
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn noop_err_step_identity_async() {
    let step = ErrNoopStep::<u8>::new();
    assert_eq!(step.apply_err(255).await, 255);
}

#[tokio::test]
async fn if_err_step_branches_async() {
    let even_flow = ErrMapStep::new(|x: i32| x / 2);
    let odd_flow = ErrMapStep::new(|x: i32| x * 3 + 1);
    let branch = ErrIfStep::new(|x: &i32| x % 2 == 0, even_flow, odd_flow);
    assert_eq!(branch.apply_err(10).await, 5);
    assert_eq!(branch.apply_err(11).await, 34);
}

#[tokio::test]
async fn err_flow_then_chaining_async() {
    let inc = ErrMapStep::new(|x: i32| x + 1);
    let dbl = ErrMapStep::new(|x: i32| x * 2);
    let chain = inc.then_err(dbl);
    assert_eq!(chain.apply_err(3).await, 8);
}
