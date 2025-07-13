/// Implements [`OkTransformer`] using [`From`] to convert between the
/// specified input and output types.
///
/// # Parameters
/// - `impl_for` - Type receiving the trait implementation.
/// - `input_ok` - Success type accepted by the transformer.
/// - `output_ok` - Success type produced by the transformer. Must implement
///   `From<input_ok>`.
#[macro_export]
macro_rules! impl_ok_transformer_via_output_from {
    (
        impl_for = $impl_for:ty,
        input_ok = $input_ok:ty,
        output_ok = $output_ok:ty $(,)?
    ) => {
        const _: fn() = || {
            fn _assert_from<T: From<$input_ok>>() {}
            _assert_from::<$output_ok>();
        };

        impl result_transformer::sync::OkTransformer<$input_ok> for $impl_for
        where
            $output_ok: From<$input_ok>,
        {
            type OutputOk = $output_ok;
            fn transform_ok(&self, ok: $input_ok) -> Self::OutputOk {
                let _ = self;
                <$output_ok as From<$input_ok>>::from(ok)
            }
        }
    };
}
pub use impl_ok_transformer_via_output_from;
