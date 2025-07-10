//! ok_sync_step_test.rs – Unit tests for OkStep implementations
//!
//! This file lives under `result-transformer-test/src/flow/sync/ok/`.
//! All tests rely only on the public API of **result-transformer**.
//!
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-sync -- flow::sync::ok::ok_sync_step_test --show-output
//! ────────────────────────────────────────────────────────────────

#[allow(unused_imports)]
use result_transformer::flow::sync::*;
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};

/// `MapOkStep` must perform a pure *value* transformation.
#[test]
fn map_ok_step_transforms() {
    let step = OkMapStep::new(|x: i32| x * 2);
    assert_eq!(step.apply_ok(21), 42);
}

/// `TapOkStep` can mutate the value **and** perform side-effects.
#[test]
fn tap_ok_step_works() {
    let step = OkTapStep::new(|x: &str| format!("Hello, {x}!"));
    assert_eq!(step.apply_ok("Rust"), "Hello, Rust!");
}

/// `InspectOkStep` should run side-effects only; the value is
/// returned unchanged.
#[test]
fn inspect_ok_step_side_effect_only() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);

    let step = OkInspectStep::new(|_: &i32| {
        CALLS.fetch_add(1, Ordering::SeqCst);
    });

    let out = step.apply_ok(7);
    assert_eq!(out, 7);
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}

/// `NoopOkStep` must be the identity function.
#[test]
fn noop_ok_step_identity() {
    let step = OkNoopStep::<u8>::new();
    assert_eq!(step.apply_ok(255), 255);
}

/// `IfOkStep` chooses the correct branch (`then_flow` vs `else_flow`)
/// based on the user-supplied predicate.
#[test]
fn if_ok_step_branches() {
    let even_flow = OkMapStep::new(|x: i32| x / 2); // even  → half
    let odd_flow = OkMapStep::new(|x: i32| x * 3 + 1); // odd   → 3n+1

    let branch = OkIfStep::new(|x: &i32| x % 2 == 0, even_flow, odd_flow);

    assert_eq!(branch.apply_ok(10), 5); // even
    assert_eq!(branch.apply_ok(11), 34); // odd
}
