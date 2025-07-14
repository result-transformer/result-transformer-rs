use result_transformer::__internal::*;
use result_transformer::async_::macros::*;
use result_transformer::flow::async_::macros::*;
use result_transformer::flow::async_::{ErrMapStepAsync, OkMapStepAsync, ResultMapStepAsync};

struct FlowPartsTransformer;

impl_async_ok_transformer_via_ok_flow!(
    impl_for = FlowPartsTransformer,
    input_ok = i32,
    output_ok = i32,
    flow = OkMapStepAsync::new(|x| Box::pin(async move { x + 1 })),
);

impl_async_err_transformer_via_err_flow!(
    impl_for = FlowPartsTransformer,
    input_err = &'static str,
    output_err = String,
    flow = ErrMapStepAsync::new(|e| Box::pin(async move { format!("E:{e}") })),
);

impl_async_result_transformer_via_self_parts!(impl_for = FlowPartsTransformer, input_ok = i32, input_err = &'static str);

#[allow(unused)]
struct ResultFlowTransformer;

impl_async_result_transformer_via_result_flow!(
    impl_for = ResultFlowTransformer,
    input_ok = i32,
    input_err = i32,
    output_ok = String,
    output_err = String,
    flow = ResultMapStepAsync::new(|r: Result<i32, i32>| Box::pin(async move {
        r.map(|o| o.to_string()).map_err(|e| format!("E{e}"))
    })),
);

#[allow(unused_imports)]
use result_transformer::async_::AsyncResultTransformer as _;

#[tokio::test]
async fn ok_and_err_flow_macros_work_async() {
    let t = FlowPartsTransformer;
    assert_eq!(t.transform_async(Ok(2)).await, Ok(3));
    assert_eq!(t.transform_async(Err("a")).await, Err("E:a".to_string()));
}

#[tokio::test]
async fn result_flow_macro_works_async() {
    let t = ResultFlowTransformer;
    assert_eq!(t.transform_async(Ok(1)).await, Ok("1".to_string()));
    assert_eq!(t.transform_async(Err(5)).await, Err("E5".to_string()));
}
