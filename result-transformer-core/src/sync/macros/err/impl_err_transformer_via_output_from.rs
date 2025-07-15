/// Implements [`ErrTransformer`] using [`From`] to convert between the
/// specified input and output error types.
///
/// # Parameters
/// - `impl_for` - Type receiving the trait implementation.
/// - `input_err` - Error type accepted by the transformer.
/// - `output_err` - Error type produced by the transformer. Must implement
///   `From<input_err>`.
#[macro_export]
macro_rules! impl_err_transformer_via_output_from {
    (
        impl_for = $impl_for:ty,
        input_err = $input_err:ty,
        output_err = $output_err:ty $(,)?
    ) => {
        const _: fn() = || {
            fn _assert_from<T: From<$input_err>>() {}
            _assert_from::<$output_err>();
        };

        impl result_transformer::sync::ErrTransformer<$input_err> for $impl_for
        where
            $output_err: From<$input_err>,
        {
            type OutputErr = $output_err;
            fn transform_err(&self, err: $input_err) -> Self::OutputErr {
                let _ = self;
                <$output_err as From<$input_err>>::from(err)
            }
        }
    };
}
pub use impl_err_transformer_via_output_from;
