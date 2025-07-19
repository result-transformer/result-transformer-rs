//! err_async_flow_test.rs – Async ErrFlow tests
//!
//! Located at `result-transformer-test/src/flow/async_/err/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-async -- flow::async_::err::err_async_flow_test --show-output
//! ────────────────────────────────────────────────────────────────

#[allow(unused_imports)]
use result_transformer::flow::async_::*;
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::test]
async fn err_flow_then_chaining_async() {
    let inc = ErrMapStep::new(|x: i32| x + 1);
    let dbl = ErrMapStep::new(|x: i32| x * 2);
    let chain = inc.then_async_err(dbl);
    assert_eq!(chain.apply_err_async(3).await, 8);
}
