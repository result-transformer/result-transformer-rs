/// Implements [`ResultTransformer`] using a [`ResultFlow`].
///
/// Shorthand syntax: `($impl_for, [$input_ok, $input_err => $output_ok, $output_err], $flow)`.
///
/// # Parameters
/// - `impl_for` - Type on which to implement the trait.
/// - `input_ok` / `input_err` - Input types accepted by the flow.
/// - `output_ok` / `output_err` - Output types produced by the flow.
/// - `flow` - Expression evaluating to the flow instance.
#[macro_export]
macro_rules! impl_result_transformer_via_result_flow {
    (
        impl_for = $impl_for:ty,
        input_ok = $input_ok:ty,
        input_err = $input_err:ty,
        output_ok = $output_ok:ty,
        output_err = $output_err:ty,
        flow = $flow:expr $(,)?
    ) => {
        const _: fn() = || {
            fn _flow_type_check<F>(_: &F)
            where
                F: result_transformer::flow::sync::ResultFlow<
                        $input_ok,
                        $input_err,
                        OutputOk = $output_ok,
                        OutputErr = $output_err,
                    >,
            {
            }
            _flow_type_check(&$flow);
        };

        result_transformer::core::sync::macros::impl_result_transformer! {
            impl_for = $impl_for,
            input_ok = $input_ok,
            input_err = $input_err,
            output_ok = $output_ok,
            output_err = $output_err,
            transform_result = |result: Result<$input_ok, $input_err>| {
                result_transformer::flow::sync::ResultFlow::apply_result(&$flow, result)
            }
        }
    };

    (
        $impl_for:ty,
        [$input_ok:ty, $input_err:ty => $output_ok:ty, $output_err:ty $(,)?],
        $flow:expr $(,)?
    ) => {
        result_transformer::flow::sync::macros::impl_result_transformer_via_result_flow!(
            impl_for = $impl_for,
            input_ok = $input_ok,
            input_err = $input_err,
            output_ok = $output_ok,
            output_err = $output_err,
            flow = $flow
        );
    };
}
pub use impl_result_transformer_via_result_flow;
