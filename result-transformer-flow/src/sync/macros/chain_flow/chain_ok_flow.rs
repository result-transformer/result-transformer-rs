/// Macro to chain multiple [`OkFlow`] implementations at compile time.
///
/// The macro expands to nested [`OkFlowChain::new_const`] calls so it can
/// be used in const contexts.
#[macro_export]
macro_rules! chain_ok_flow {
    ($single:expr $(,)?) => { $single };
    ($first:expr, $($rest:expr),+ $(,)?) => {
        result_transformer::flow::sync::OkFlowChain::new_const(
            $first,
            result_transformer::flow::sync::macros::chain_ok_flow!($($rest),+),
        )
    };
}
pub use chain_ok_flow;
