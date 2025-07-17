/// Implements [`AsyncResultTransformer`] using an [`AsyncResultFlow`].
///
/// Shorthand syntax: `($impl_for, [$input_ok, $input_err => $output_ok, $output_err], $flow)`.
///
/// # Parameters
/// - `impl_for` - Target type for the implementation.
/// - `input_ok` / `input_err` - Input types accepted by the flow.
/// - `output_ok` / `output_err` - Output types produced by the flow.
/// - `flow` - Expression yielding the flow instance.
#[macro_export]
macro_rules! impl_async_result_transformer_via_async_result_flow {
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
                F: result_transformer::flow::async_::AsyncResultFlow<
                        $input_ok,
                        $input_err,
                        OutputOk = $output_ok,
                        OutputErr = $output_err,
                    >,
            {
            }
            _flow_type_check(&$flow);
        };

        result_transformer::core::async_::macros::impl_async_result_transformer! {
            impl_for = $impl_for,
            input_ok = $input_ok,
            input_err = $input_err,
            output_ok = $output_ok,
            output_err = $output_err,
            transform_result = |result: Result<$input_ok, $input_err>| {
                async move{
                    result_transformer::flow::async_::AsyncResultFlow::apply_result_async(&$flow, result).await
                }
            }
        }
    };

    (
        $impl_for:ty,
        [$input_ok:ty, $input_err:ty => $output_ok:ty, $output_err:ty $(,)?],
        $flow:expr $(,)?
    ) => {
        result_transformer::flow::async_::macros::impl_async_result_transformer_via_result_flow!(
            impl_for = $impl_for,
            input_ok = $input_ok,
            input_err = $input_err,
            output_ok = $output_ok,
            output_err = $output_err,
            flow = $flow
        );
    };
}
pub use impl_async_result_transformer_via_async_result_flow;
