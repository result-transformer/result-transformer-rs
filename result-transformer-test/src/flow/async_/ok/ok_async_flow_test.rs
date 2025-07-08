//! ok_async_flow_test.rs – Async unit tests for OkFlow steps

use result_transformer::__internal::*;
#[allow(unused_imports)]
use result_transformer::flow::async_::*;
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::test]
async fn map_ok_step_transforms_async() {
    let step = OkMapStep::new(|x: i32| x * 2);
    assert_eq!(step.apply_ok(21).await, 42);
}

#[tokio::test]
async fn tap_ok_step_works_async() {
    let step = OkTapStep::new(|x: &str| format!("Hello, {x}!"));
    assert_eq!(step.apply_ok("Rust").await, "Hello, Rust!");
}

#[tokio::test]
async fn inspect_ok_step_side_effect_only_async() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    let step = OkInspectStep::new(|_: &i32| {
        CALLS.fetch_add(1, Ordering::SeqCst);
    });
    let out = step.apply_ok(7).await;
    assert_eq!(out, 7);
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn noop_ok_step_identity_async() {
    let step = OkNoopStep::<u8>::new();
    assert_eq!(step.apply_ok(255).await, 255);
}

#[tokio::test]
async fn if_ok_step_branches_async() {
    let even_flow = OkMapStep::new(|x: i32| x / 2);
    let odd_flow = OkMapStep::new(|x: i32| x * 3 + 1);
    let branch = OkIfStep::new(|x: &i32| x % 2 == 0, even_flow, odd_flow);
    assert_eq!(branch.apply_ok(10).await, 5);
    assert_eq!(branch.apply_ok(11).await, 34);
}

#[tokio::test]
async fn ok_flow_then_chaining_async() {
    let inc = OkMapStep::new(|x: i32| x + 1);
    let dbl = OkMapStep::new(|x: i32| x * 2);
    let chain = inc.then_ok(dbl);
    assert_eq!(chain.apply_ok(3).await, 8);
}
