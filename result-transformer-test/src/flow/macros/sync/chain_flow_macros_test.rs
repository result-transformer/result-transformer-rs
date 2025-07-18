//! chain_flow_macros_test.rs – Const chain macros for sync flows
//!
//! Located at `result-transformer-test/src/flow/macros/sync/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-sync-macros -- flow::macros::sync::chain_flow_macros_test --show-output
//! ────────────────────────────────────────────────────────────────

#[allow(unused)]
use result_transformer::flow::sync::macros::*;
#[allow(unused)]
use result_transformer::flow::sync::*;

#[test]
fn ok_flow_const_chain_macro() {
    fn inc(x: i32) -> i32 {
        x + 1
    }
    fn dbl(x: i32) -> i32 {
        x * 2
    }

    const FLOW: OkFlowChain<
        i32,
        OkMapStep<i32, i32, fn(i32) -> i32>,
        OkMapStep<i32, i32, fn(i32) -> i32>,
    > = chain_ok_flow!(
        OkMapStep::new(inc as fn(i32) -> i32),
        OkMapStep::new(dbl as fn(i32) -> i32),
    );

    assert_eq!(FLOW.apply_ok(3), 8);
}

#[test]
fn err_flow_const_chain_macro() {
    fn inc(x: i32) -> i32 {
        x + 1
    }
    fn dbl(x: i32) -> i32 {
        x * 2
    }

    const FLOW: ErrFlowChain<
        i32,
        ErrMapStep<i32, i32, fn(i32) -> i32>,
        ErrMapStep<i32, i32, fn(i32) -> i32>,
    > = chain_err_flow!(
        ErrMapStep::new(inc as fn(i32) -> i32),
        ErrMapStep::new(dbl as fn(i32) -> i32),
    );

    assert_eq!(FLOW.apply_err(3), 8);
}

#[test]
fn result_flow_const_chain_macro() {
    fn inc_map(r: Result<i32, &str>) -> Result<i32, &str> {
        r.map(|x| x + 1)
    }
    fn dbl_map(r: Result<i32, &str>) -> Result<i32, String> {
        r.map(|x| x * 2).map_err(|e| format!("E:{e}"))
    }

    const FLOW: ResultFlowChain<
        i32,
        &str,
        ResultMapStep<i32, &str, i32, &str, fn(Result<i32, &str>) -> Result<i32, &str>>,
        ResultMapStep<i32, &str, i32, String, fn(Result<i32, &str>) -> Result<i32, String>>,
    > = chain_result_flow!(
        ResultMapStep::new(inc_map as fn(Result<i32, &str>) -> Result<i32, &str>),
        ResultMapStep::new(dbl_map as fn(Result<i32, &str>) -> Result<i32, String>),
    );

    assert_eq!(FLOW.apply_result(Ok(4)), Ok(10));
    assert_eq!(FLOW.apply_result(Err("oops")), Err("E:oops".to_string()));
}
