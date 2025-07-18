/// Macro to chain multiple [`AsyncResultFlow`] implementations at compile time.
///
/// The macro expands to nested [`AsyncResultFlowChain::new_const`] calls so it can
/// be used in const contexts.
#[macro_export]
macro_rules! chain_async_result_flow {
    ($single:expr $(,)?) => { $single };
    ($first:expr, $($rest:expr),+ $(,)?) => {
        result_transformer::flow::async_::AsyncResultFlowChain::new_const(
            $first,
            result_transformer::flow::async_::macros::chain_async_result_flow!($($rest),+)
        )
    };
}
pub use chain_async_result_flow;
