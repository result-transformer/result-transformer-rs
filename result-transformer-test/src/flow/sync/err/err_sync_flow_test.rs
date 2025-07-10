//!
//! Located at `result-transformer-test/src/flow/sync/err/` and focuses on flow composition using the public API of **result-transformer**.
//!
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-sync -- flow::sync::err::err_sync_flow_test --show-output
//! ────────────────────────────────────────────────────────────────

#[allow(unused_imports)]
use result_transformer::flow::sync::*;
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};

/// `then_err` must chain two `ErrFlow`s in sequence.
#[test]
fn err_flow_then_chaining() {
    let inc = ErrMapStep::new(|x: i32| x + 1);
    let dbl = ErrMapStep::new(|x: i32| x * 2);

    let chain = inc.then_err(dbl);
    assert_eq!(chain.apply_err(3), 8); // (3 + 1) * 2
}
