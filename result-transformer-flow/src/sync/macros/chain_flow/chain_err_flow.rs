/// Macro to chain multiple [`ErrFlow`] implementations at compile time.
///
/// The macro expands to nested [`ErrFlowChain::new_const`] calls so it can
/// be used in const contexts.
#[macro_export]
macro_rules! chain_err_flow {
    ($single:expr $(,)?) => { $single };
    ($first:expr, $($rest:expr),+ $(,)?) => {
        result_transformer::flow::sync::ErrFlowChain::new_const(
            $first,
            result_transformer::flow::sync::macros::chain_err_flow!($($rest),+),
        )
    };
}
pub use chain_err_flow;
