/// Implements [`OkTransformer`] using an [`OkFlow`].
///
/// Shorthand syntax: `($impl_for, [$input_ok => $output_ok], $flow)`.
///
/// # Parameters
/// - `impl_for` - Type on which to implement the trait.
/// - `input_ok` - Success type accepted by the flow.
/// - `output_ok` - Success type produced by the flow.
/// - `flow` - Expression evaluating to the flow instance.
#[macro_export]
macro_rules! impl_ok_transformer_via_ok_flow {
    (
        impl_for = $impl_for:ty,
        input_ok = $input_ok:ty,
        output_ok = $output_ok:ty,
        flow = $flow:expr $(,)?
    ) => {
        const _: fn() = || {
            fn _flow_type_check<F>(_: &F)
            where
                F: result_transformer::flow::sync::OkFlow<$input_ok, OutputOk = $output_ok>,
            {
            }
            _flow_type_check(&$flow);
        };

        result_transformer::core::sync::macros::impl_ok_transformer! {
            impl_for = $impl_for,
            input_ok = $input_ok,
            output_ok = $output_ok,
            transform_ok = |ok: $input_ok| {
                result_transformer::flow::sync::OkFlow::apply_ok(&$flow, ok)
            }
        }
    };

    (
        $impl_for:ty,
        [$input_ok:ty => $output_ok:ty $(,)?],
        $flow:expr $(,)?
    ) => {
        result_transformer::flow::sync::macros::impl_ok_transformer_via_ok_flow!(
            impl_for = $impl_for,
            input_ok = $input_ok,
            output_ok = $output_ok,
            flow = $flow
        );
    };
}
pub use impl_ok_transformer_via_ok_flow;
