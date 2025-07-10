//! define_transformer_from_flow_test.rs – Tests for macros generating
//! transformers from flows
//!
//! Located at `result-transformer-test/src/flow_macros/sync/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-sync-macros -- flow_macros::sync::define_transformer_from_flow_test --show-output
//! ────────────────────────────────────────────────────────────────

use result_transformer::flow::sync::step::{ErrMapStep, OkMapStep, ResultMapStep};
use result_transformer::macros::*;

struct FlowPartsTransformer;

define_ok_transformer_from_ok_flow!(
    impl_for = FlowPartsTransformer,
    input_ok = i32,
    output_ok = i32,
    flow = OkMapStep::new(|x| x + 1),
);

define_err_transformer_from_err_flow!(
    impl_for = FlowPartsTransformer,
    input_err = &'static str,
    output_err = String,
    flow = ErrMapStep::new(|e| format!("E:{e}")),
);

impl_result_transformer_from_parts!(impl_for = FlowPartsTransformer, input_ok = i32, input_err = &'static str);

#[allow(unused)]
struct ResultFlowTransformer;

define_result_transformer_from_result_flow!(
    impl_for = ResultFlowTransformer,
    input_ok = i32,
    input_err = i32,
    output_ok = String,
    output_err = String,
    flow = ResultMapStep::new(|r: Result<i32, i32>| r
        .map(|o| o.to_string())
        .map_err(|e| format!("E{e}"))),
);

#[allow(unused_imports)]
use result_transformer::sync::ResultTransformer as _;

#[test]
fn ok_and_err_flow_macros_work() {
    let t = FlowPartsTransformer;
    assert_eq!(t.transform(Ok(2)), Ok(3));
    assert_eq!(t.transform(Err("a")), Err("E:a".to_string()));
}

#[test]
fn result_flow_macro_works() {
    let t = ResultFlowTransformer;
    assert_eq!(t.transform(Ok(1)), Ok("1".to_string()));
    assert_eq!(t.transform(Err(5)), Err("E5".to_string()));
}
