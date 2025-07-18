/// Produces the nested [`AsyncErrFlowChain`] type for a list of step types.
///
/// Shorthand syntax: `($input_err, $first, $($rest),+)`.
///
/// # Parameters
/// - `input_err` - The input error type for the entire chain.
/// - `steps` - Types implementing [`AsyncErrFlow`] to chain.
#[macro_export]
macro_rules! chain_async_err_flow_type {
    (
        input_err = $input_err:ty,
        steps = [$single:ty $(,)?]
    ) => {
        $single
    };

    (
        input_err = $input_err:ty,
        steps = [$first:ty, $($rest:ty),+ $(,)?]
    ) => {
        result_transformer::flow::async_::AsyncErrFlowChain<
            $input_err,
            $first,
            result_transformer::flow::async_::macros::chain_async_err_flow_type!(
                input_err = $input_err,
                steps = [$($rest),+]
            ),
        >
    };

    ($input_err:ty, $($step:ty),+ $(,)?) => {
        result_transformer::flow::async_::macros::chain_async_err_flow_type!(
            input_err = $input_err,
            steps = [$($step),+]
        )
    };
}
pub use chain_async_err_flow_type;
