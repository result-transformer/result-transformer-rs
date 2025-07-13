/// Implements [`OkTransformer`] using [`Into`] to convert from the
/// specified input type to the output type.
///
/// # Parameters
/// - `impl_for` - Type receiving the trait implementation.
/// - `input_ok` - Success type accepted by the transformer. Must implement
///   `Into<output_ok>`.
/// - `output_ok` - Success type produced by the transformer.
#[macro_export]
macro_rules! impl_ok_transformer_via_input_into {
    (
        impl_for = $impl_for:ty,
        input_ok = $input_ok:ty,
        output_ok = $output_ok:ty $(,)?
    ) => {
        const _: fn() = || {
            fn _assert_into<T: Into<$output_ok>>() {}
            _assert_into::<$input_ok>();
        };

        impl result_transformer::sync::OkTransformer<$input_ok> for $impl_for
        where
            $input_ok: Into<$output_ok>,
        {
            type OutputOk = $output_ok;
            fn transform_ok(&self, ok: $input_ok) -> Self::OutputOk {
                let _ = self;
                <$input_ok as Into<$output_ok>>::into(ok)
            }
        }
    };
}
pub use impl_ok_transformer_via_input_into;
