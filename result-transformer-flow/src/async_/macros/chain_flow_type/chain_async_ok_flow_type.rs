/// Produces the nested [`AsyncOkFlowChain`] type for a list of step types.
///
/// Shorthand syntax: `($input_ok, $first, $($rest),+)`.
///
/// # Parameters
/// - `input_ok` - The input success type for the entire chain.
/// - `steps` - Types implementing [`AsyncOkFlow`] to chain.
#[macro_export]
macro_rules! chain_async_ok_flow_type {
    (
        input_ok = $input_ok:ty,
        steps = [$single:ty $(,)?]
    ) => {
        $single
    };

    (
        input_ok = $input_ok:ty,
        steps = [$first:ty, $($rest:ty),+ $(,)?]
    ) => {
        result_transformer::flow::async_::AsyncOkFlowChain<
            $first,
            result_transformer::flow::async_::macros::chain_async_ok_flow_type!(
                input_ok = $input_ok,
                steps = [$($rest),+]
            ),
            $input_ok,
        >
    };

    ($input_ok:ty, $($step:ty),+ $(,)?) => {
        result_transformer::flow::async_::macros::chain_async_ok_flow_type!(
            input_ok = $input_ok,
            steps = [$($step),+]
        )
    };
}
pub use chain_async_ok_flow_type;
