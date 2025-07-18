/// Produces the nested [`ResultFlowChain`] type for a list of step types.
///
/// Shorthand syntax: `($input_ok, $input_err, $first, $($rest),+)`.
///
/// # Parameters
/// - `input_ok` - The input success type for the entire chain.
/// - `input_err` - The input error type for the entire chain.
/// - `steps` - Types implementing [`ResultFlow`] to chain.
#[macro_export]
macro_rules! chain_result_flow_type {
    (
        input_ok = $input_ok:ty,
        input_err = $input_err:ty,
        steps = [$single:ty $(,)?]
    ) => {
        $single
    };

    (
        input_ok = $input_ok:ty,
        input_err = $input_err:ty,
        steps = [$first:ty, $($rest:ty),+ $(,)?]
    ) => {
        result_transformer::flow::sync::ResultFlowChain<
            $first,
            result_transformer::flow::sync::macros::chain_result_flow_type!(
                input_ok = $input_ok,
                input_err = $input_err,
                steps = [$($rest),+]
            ),
            $input_ok,
            $input_err,
        >
    };

    ($input_ok:ty, $input_err:ty, $($step:ty),+ $(,)?) => {
        result_transformer::flow::sync::macros::chain_result_flow_type!(
            input_ok = $input_ok,
            input_err = $input_err,
            steps = [$($step),+]
        )
    };
}
pub use chain_result_flow_type;
