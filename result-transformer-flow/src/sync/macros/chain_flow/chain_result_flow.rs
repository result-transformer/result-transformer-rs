/// Macro to chain multiple [`ResultFlow`] implementations at compile time.
///
/// The macro expands to nested [`ResultFlowChain::new_const`] calls so it can
/// be used in const contexts.
#[macro_export]
macro_rules! chain_result_flow {
    ($single:expr $(,)?) => { $single };
    ($first:expr, $($rest:expr),+ $(,)?) => {
        result_transformer::flow::sync::ResultFlowChain::new_const(
            $first,
            result_transformer::flow::sync::macros::chain_result_flow!($($rest),+)
        )
    };
}
pub use chain_result_flow;
