//! ok_async_flow_test.rs – Async OkFlow tests
//!
//! Located at `result-transformer-test/src/flow/async_/ok/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-async -- flow::async_::ok::ok_async_flow_test --show-output
//! ────────────────────────────────────────────────────────────────

use result_transformer::__internal::*;
#[allow(unused_imports)]
use result_transformer::flow::async_::*;
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::test]
async fn ok_flow_then_chaining_async() {
    let inc = OkMapStep::new(|x: i32| x + 1);
    let dbl = OkMapStep::new(|x: i32| x * 2);
    let chain = inc.then_async_ok(dbl);
    assert_eq!(chain.apply_ok_async(3).await, 8);
}
