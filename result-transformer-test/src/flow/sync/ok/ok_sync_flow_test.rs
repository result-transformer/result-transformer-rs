//! ok_sync_flow_test.rs – OkFlow specific tests
//!
//! This file lives under `result-transformer-test/src/flow/sync/ok/`.
//! It contains tests focusing on flow composition using the public API of **result-transformer**.
//!
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-sync -- flow::sync::ok::ok_sync_flow_test --show-output
//! ────────────────────────────────────────────────────────────────

#[allow(unused_imports)]
use result_transformer::flow::sync::*;
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};

/// `then_ok` must chain two `OkFlow`s in sequence.
#[test]
fn ok_flow_then_chaining() {
    let inc = OkMapStep::new(|x: i32| x + 1);
    let dbl = OkMapStep::new(|x: i32| x * 2);

    let chain = inc.then_ok(dbl);
    assert_eq!(chain.apply_ok(3), 8); // (3 + 1) * 2
}
