/// Implements [`ErrTransformer`] using [`Into`] to convert from the
/// specified input error type to the output error type.
///
/// # Parameters
/// - `impl_for` - Type receiving the trait implementation.
/// - `input_err` - Error type accepted by the transformer. Must implement
///   `Into<output_err>`.
/// - `output_err` - Error type produced by the transformer.
#[macro_export]
macro_rules! impl_err_transformer_via_input_into {
    (
        impl_for = $impl_for:ty,
        input_err = $input_err:ty,
        output_err = $output_err:ty $(,)?
    ) => {
        const _: fn() = || {
            fn _assert_into<T: Into<$output_err>>() {}
            _assert_into::<$input_err>();
        };

        impl result_transformer::sync::ErrTransformer<$input_err> for $impl_for
        where
            $input_err: Into<$output_err>,
        {
            type OutputErr = $output_err;
            fn transform_err(&self, err: $input_err) -> Self::OutputErr {
                let _ = self;
                <$input_err as Into<$output_err>>::into(err)
            }
        }
    };
}
pub use impl_err_transformer_via_input_into;
