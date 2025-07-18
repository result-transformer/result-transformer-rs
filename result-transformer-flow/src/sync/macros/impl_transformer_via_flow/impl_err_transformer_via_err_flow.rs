/// Implements [`ErrTransformer`] using an [`ErrFlow`].
///
/// Shorthand syntax: `($impl_for, [$input_err => $output_err], $flow)`.
///
/// # Parameters
/// - `impl_for` - Type that will implement the trait.
/// - `input_err` - Error type accepted by the flow.
/// - `output_err` - Error type produced by the flow.
/// - `flow` - Expression evaluating to the flow instance.
#[macro_export]
macro_rules! impl_err_transformer_via_err_flow {
    (
        impl_for = $impl_for:ty,
        input_err = $input_err:ty,
        output_err = $output_err:ty,
        flow = $flow:expr $(,)?
    ) => {
        const _: fn() = || {
            fn _flow_type_check<F>(_: &F)
            where
                F: result_transformer::flow::sync::ErrFlow<$input_err, OutputErr = $output_err>,
            {
            }
            _flow_type_check(&$flow);
        };

        result_transformer::core::sync::macros::impl_err_transformer! {
            impl_for = $impl_for,
            input_err = $input_err,
            output_err = $output_err,
            transform_err = |err: $input_err| {
                result_transformer::flow::sync::ErrFlow::apply_err(&$flow, err)
            }
        }
    };

    (
        $impl_for:ty,
        [$input_err:ty => $output_err:ty $(,)?],
        $flow:expr $(,)?
    ) => {
        result_transformer::flow::sync::macros::impl_err_transformer_via_err_flow!(
            impl_for = $impl_for,
            input_err = $input_err,
            output_err = $output_err,
            flow = $flow
        );
    };
}
pub use impl_err_transformer_via_err_flow;
