/// Implements [`AsyncOkTransformer`] using an [`AsyncOkFlow`].
///
/// Shorthand syntax: `($impl_for, [$input_ok => $output_ok], $flow)`.
///
/// # Parameters
/// - `impl_for` - Target type for the implementation.
/// - `input_ok` - Success type accepted by the flow.
/// - `output_ok` - Success type produced by the flow and transformer.
/// - `flow` - Expression evaluating to the flow instance.
#[macro_export]
macro_rules! impl_async_ok_transformer_via_async_ok_flow {
    (
        impl_for = $impl_for:ty,
        input_ok = $input_ok:ty,
        output_ok = $output_ok:ty,
        flow = $flow:expr $(,)?
    ) => {
        const _: fn() = || {
            fn _flow_type_check<F>(_: &F)
            where
                F: result_transformer::flow::async_::AsyncOkFlow<$input_ok, OutputOk = $output_ok>
                    + Send,
            {
            }
            _flow_type_check(&$flow);
        };

        result_transformer::core::async_::macros::impl_async_ok_transformer! {
            impl_for = $impl_for,
            input_ok = $input_ok,
            output_ok = $output_ok,
            transform_ok = |ok: $input_ok| {
                async move{
                    result_transformer::flow::async_::AsyncOkFlow::apply_ok_async(&$flow, ok).await
                }
            }
        }
    };

    (
        $impl_for:ty,
        [$input_ok:ty => $output_ok:ty $(,)?],
        $flow:expr $(,)?
    ) => {
        result_transformer::flow::async_::macros::impl_async_ok_transformer_via_ok_flow!(
            impl_for = $impl_for,
            input_ok = $input_ok,
            output_ok = $output_ok,
            flow = $flow
        );
    };
}
pub use impl_async_ok_transformer_via_async_ok_flow;
