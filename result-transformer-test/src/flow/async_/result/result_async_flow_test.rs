//! result_async_flow_test.rs – Async ResultFlow tests
//!
//! Located at `result-transformer-test/src/flow/async_/result/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-async -- flow::async_::result::result_async_flow_test --show-output
//! ────────────────────────────────────────────────────────────────

#[allow(unused_imports)]
use result_transformer::flow::async_::*;
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::test]
async fn result_flow_then_chaining_async() {
    let f1 = ResultMapBothStep::new(|x: i32| x + 1, |e| e);
    let f2 = ResultMapBothStep::new(|x: i32| x * 2, |e| format!("E:{}", e));
    let flow = f1.then_async_result(f2);
    assert_eq!(flow.apply_result_async(Ok(4)).await, Ok(10));
    assert_eq!(
        flow.apply_result_async(Err("oops")).await,
        Err("E:oops".to_string())
    );
}
