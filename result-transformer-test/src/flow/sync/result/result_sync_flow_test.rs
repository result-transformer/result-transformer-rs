//! ResultFlow specific tests
//!
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --features flow-sync -- flow::sync::result::result_sync_flow_test --show-output
//! ────────────────────────────────────────────────────────────────

#[allow(unused_imports)]
use result_transformer::flow::sync::*;
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};

/// `then_result` chains multiple ResultFlow transformations.
#[test]
fn result_flow_then_chaining() {
    let f1 = ResultMapStep::new(|r: Result<i32, &str>| r.map(|x| x + 1));
    let f2 =
        ResultMapStep::new(|r: Result<i32, &str>| r.map(|x| x * 2).map_err(|e| format!("E:{}", e)));

    let flow = f1.then_result(f2);

    assert_eq!(flow.apply_result(Ok(4)), Ok(10));
    assert_eq!(flow.apply_result(Err("oops")), Err("E:oops".to_string()));
}
