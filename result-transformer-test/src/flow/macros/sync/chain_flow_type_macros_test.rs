//! chain_flow_type_macros_test.rs – Macros generating chain types for sync flows
//!
//! Located at `result-transformer-test/src/flow/macros/sync/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-sync-macros -- flow::macros::sync::chain_flow_type_macros_test --show-output
//! ────────────────────────────────────────────────────────────────

#[allow(unused)]
use result_transformer::flow::sync::macros::*;
#[allow(unused)]
use result_transformer::flow::sync::*;

#[test]
fn ok_flow_chain_type_macro() {
    fn inc(x: i32) -> i32 {
        x + 1
    }
    fn dbl(x: i32) -> i32 {
        x * 2
    }

    type FlowTy = chain_ok_flow_type!(
        input_ok = i32,
        steps = [
            OkMapStep<i32, i32, fn(i32) -> i32>,
            OkMapStep<i32, i32, fn(i32) -> i32>
        ]
    );

    const FLOW: FlowTy = chain_ok_flow!(
        OkMapStep::new(inc as fn(i32) -> i32),
        OkMapStep::new(dbl as fn(i32) -> i32),
    );

    assert_eq!(FLOW.apply_ok(3), 8);
}

#[test]
fn err_flow_chain_type_macro() {
    fn inc(x: i32) -> i32 {
        x + 1
    }
    fn dbl(x: i32) -> i32 {
        x * 2
    }

    type FlowTy = chain_err_flow_type!(
        input_err = i32,
        steps = [
            ErrMapStep<i32, i32, fn(i32) -> i32>,
            ErrMapStep<i32, i32, fn(i32) -> i32>
        ]
    );

    const FLOW: FlowTy = chain_err_flow!(
        ErrMapStep::new(inc as fn(i32) -> i32),
        ErrMapStep::new(dbl as fn(i32) -> i32),
    );

    assert_eq!(FLOW.apply_err(3), 8);
}

#[test]
fn result_flow_chain_type_macro() {
    fn inc_map(r: Result<i32, &'static str>) -> Result<i32, &'static str> {
        r.map(|x| x + 1)
    }
    fn dbl_map(r: Result<i32, &'static str>) -> Result<i32, String> {
        r.map(|x| x * 2).map_err(|e| format!("E:{e}"))
    }

    type FlowTy = chain_result_flow_type!(
        input_ok = i32,
        input_err = &'static str,
        steps = [
            ResultMapStep<i32, &'static str, i32, &'static str, fn(Result<i32, &'static str>) -> Result<i32, &'static str>>,
            ResultMapStep<i32, &'static str, i32, String, fn(Result<i32, &'static str>) -> Result<i32, String>>
        ]
    );

    const FLOW: FlowTy = chain_result_flow!(
        ResultMapStep::new(inc_map as fn(Result<i32, &'static str>) -> Result<i32, &'static str>),
        ResultMapStep::new(dbl_map as fn(Result<i32, &'static str>) -> Result<i32, String>),
    );

    assert_eq!(FLOW.apply_result(Ok(4)), Ok(10));
    assert_eq!(FLOW.apply_result(Err("oops")), Err("E:oops".to_string()));
}
