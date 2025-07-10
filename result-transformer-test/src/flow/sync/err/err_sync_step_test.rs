//! err_sync_step_test.rs – Unit tests for ErrStep implementations
//!
//! Located at `result-transformer-test/src/flow/sync/err/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-sync -- flow::sync::err::err_sync_step_test --show-output
//! ────────────────────────────────────────────────────────────────

#[allow(unused_imports)]
use result_transformer::flow::sync::*;
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};

/// `MapErrStep` must perform a pure *value* transformation.
#[test]
fn map_err_step_transforms() {
    let step = ErrMapStep::new(|x: i32| x * 2);
    assert_eq!(step.apply_err(21), 42);
}

/// `TapErrStep` can mutate the value **and** perform side-effects.
#[test]
fn tap_err_step_works() {
    let step = ErrTapStep::new(|x: &str| format!("Hello, {x}!"));
    assert_eq!(step.apply_err("Rust"), "Hello, Rust!");
}

/// `InspectErrStep` should run side-effects only; the value is returned unchanged.
#[test]
fn inspect_err_step_side_effect_only() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);

    let step = ErrInspectStep::new(|_: &i32| {
        CALLS.fetch_add(1, Ordering::SeqCst);
    });

    let out = step.apply_err(7);
    assert_eq!(out, 7);
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}

/// `NoopErrStep` must be the identity function.
#[test]
fn noop_err_step_identity() {
    let step = ErrNoopStep::<u8>::new();
    assert_eq!(step.apply_err(255), 255);
}

/// `IfErrStep` chooses the correct branch based on the user-supplied predicate.
#[test]
fn if_err_step_branches() {
    let even_flow = ErrMapStep::new(|x: i32| x / 2); // even  → half
    let odd_flow = ErrMapStep::new(|x: i32| x * 3 + 1); // odd   → 3n+1

    let branch = ErrIfStep::new(|x: &i32| x % 2 == 0, even_flow, odd_flow);

    assert_eq!(branch.apply_err(10), 5); // even
    assert_eq!(branch.apply_err(11), 34); // odd
}
