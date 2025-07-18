//! chain_flow_macros_test.rs – Const chain macros for async flows
//!
//! Located at `result-transformer-test/src/flow/macros/async_/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features flow-async-macros,flow-sync-macros -- flow::macros::async_::chain_flow_macros_test --show-output
//! ────────────────────────────────────────────────────────────────

use result_transformer::__internal as deps;

#[allow(unused)]
use result_transformer::flow::async_::macros::*;
#[allow(unused)]
use result_transformer::flow::async_::*;

use deps::tokio;

#[tokio::test]
async fn async_ok_flow_const_chain_macro() {
    fn inc(x: i32) -> std::pin::Pin<Box<dyn core::future::Future<Output = i32> + Send>> {
        Box::pin(async move { x + 1 })
    }
    fn dbl(x: i32) -> std::pin::Pin<Box<dyn core::future::Future<Output = i32> + Send>> {
        Box::pin(async move { x * 2 })
    }

    const FLOW: AsyncOkFlowChain<
        i32,
        OkMapStepAsync<
            i32,
            i32,
            fn(i32) -> std::pin::Pin<Box<dyn core::future::Future<Output = i32> + Send>>,
            std::pin::Pin<Box<dyn core::future::Future<Output = i32> + Send>>,
        >,
        OkMapStepAsync<
            i32,
            i32,
            fn(i32) -> std::pin::Pin<Box<dyn core::future::Future<Output = i32> + Send>>,
            std::pin::Pin<Box<dyn core::future::Future<Output = i32> + Send>>,
        >,
    > = chain_async_ok_flow!(
        OkMapStepAsync::new(
            inc as fn(i32) -> std::pin::Pin<Box<dyn core::future::Future<Output = i32> + Send>>
        ),
        OkMapStepAsync::new(
            dbl as fn(i32) -> std::pin::Pin<Box<dyn core::future::Future<Output = i32> + Send>>
        ),
    );

    assert_eq!(FLOW.apply_ok_async(3).await, 8);
}

#[tokio::test]
async fn async_err_flow_const_chain_macro() {
    fn inc(x: i32) -> std::pin::Pin<Box<dyn core::future::Future<Output = i32> + Send>> {
        Box::pin(async move { x + 1 })
    }
    fn dbl(x: i32) -> std::pin::Pin<Box<dyn core::future::Future<Output = i32> + Send>> {
        Box::pin(async move { x * 2 })
    }

    const FLOW: AsyncErrFlowChain<
        i32,
        ErrMapStepAsync<
            i32,
            i32,
            fn(i32) -> std::pin::Pin<Box<dyn core::future::Future<Output = i32> + Send>>,
            std::pin::Pin<Box<dyn core::future::Future<Output = i32> + Send>>,
        >,
        ErrMapStepAsync<
            i32,
            i32,
            fn(i32) -> std::pin::Pin<Box<dyn core::future::Future<Output = i32> + Send>>,
            std::pin::Pin<Box<dyn core::future::Future<Output = i32> + Send>>,
        >,
    > = chain_async_err_flow!(
        ErrMapStepAsync::new(
            inc as fn(i32) -> std::pin::Pin<Box<dyn core::future::Future<Output = i32> + Send>>
        ),
        ErrMapStepAsync::new(
            dbl as fn(i32) -> std::pin::Pin<Box<dyn core::future::Future<Output = i32> + Send>>
        ),
    );

    assert_eq!(FLOW.apply_err_async(3).await, 8);
}

#[tokio::test]
async fn async_result_flow_const_chain_macro() {
    fn inc_map(
        r: Result<i32, &'static str>,
    ) -> std::pin::Pin<Box<dyn core::future::Future<Output = Result<i32, &'static str>> + Send>>
    {
        Box::pin(async move { r.map(|x| x + 1) })
    }
    fn dbl_map(
        r: Result<i32, &'static str>,
    ) -> std::pin::Pin<Box<dyn core::future::Future<Output = Result<i32, String>> + Send>> {
        Box::pin(async move { r.map(|x| x * 2).map_err(|e| format!("E:{e}")) })
    }

    const FLOW: AsyncResultFlowChain<
        i32,
        &str,
        ResultMapStepAsync<
            i32,
            &'static str,
            i32,
            &'static str,
            fn(
                Result<i32, &'static str>,
            ) -> std::pin::Pin<
                Box<dyn core::future::Future<Output = Result<i32, &'static str>> + Send>,
            >,
            std::pin::Pin<Box<dyn core::future::Future<Output = Result<i32, &'static str>> + Send>>,
        >,
        ResultMapStepAsync<
            i32,
            &str,
            i32,
            String,
            fn(
                Result<i32, &'static str>,
            )
                -> std::pin::Pin<Box<dyn core::future::Future<Output = Result<i32, String>> + Send>>,
            std::pin::Pin<Box<dyn core::future::Future<Output = Result<i32, String>> + Send>>,
        >,
    > = chain_async_result_flow!(
        ResultMapStepAsync::new(
            inc_map
                as fn(
                    Result<i32, &'static str>,
                ) -> std::pin::Pin<
                    Box<dyn core::future::Future<Output = Result<i32, &'static str>> + Send>,
                >
        ),
        ResultMapStepAsync::new(
            dbl_map
                as fn(
                    Result<i32, &'static str>,
                ) -> std::pin::Pin<
                    Box<dyn core::future::Future<Output = Result<i32, String>> + Send>,
                >
        ),
    );

    assert_eq!(FLOW.apply_result_async(Ok(4)).await, Ok(10));
    assert_eq!(
        FLOW.apply_result_async(Err("oops")).await,
        Err("E:oops".to_string())
    );
}
