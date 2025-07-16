/// Creates an [`AsyncErrTransformer`] from an [`AsyncErrFlow`].
///
/// # Parameters
/// - `impl_for` - Type that will receive the generated implementation.
/// - `input_err` - Error type accepted by the flow.
/// - `output_err` - Error type produced by the flow and transformer.
/// - `flow` - Expression evaluating to the flow instance.
#[macro_export]
macro_rules! impl_async_err_transformer_via_err_flow {
    (
        impl_for = $impl_for:ty,
        input_err = $input_err:ty,
        output_err = $output_err:ty,
        flow = $flow:expr $(,)?
    ) => {
        const _: fn() = || {
            fn _flow_type_check<F>(_: &F)
            where
                F: result_transformer::flow::async_::AsyncErrFlow<
                        $input_err,
                        OutputErr = $output_err,
                    > + Send,
            {
            }
            _flow_type_check(&$flow);
        };

        result_transformer::core::async_::macros::impl_async_err_transformer! {
            impl_for = $impl_for,
            input_err = $input_err,
            output_err = $output_err,
            transform_err = |err: $input_err| {
                async move{
                    result_transformer::flow::async_::AsyncErrFlow::apply_err_async(&$flow, err).await
                }
            }
        }
    };
}
pub use impl_async_err_transformer_via_err_flow;
