/// Implements [`AsyncErrTransformer`] using [`From`] to convert between the
/// specified input and output error types.
///
/// Shorthand syntax: `($impl_for, [$input_err => $output_err])`.
///
/// # Parameters
/// - `impl_for` - The type that receives the trait implementation.
/// - `input_err` - Error type accepted by the transformer.
/// - `output_err` - Error type produced by the transformer. Must implement
///   `From<input_err>`.
#[macro_export]
macro_rules! impl_async_err_transformer_via_output_from {
    (
        impl_for = $impl_for:ty,
        input_err = $input_err:ty,
        output_err = $output_err:ty $(,)?
    ) => {
        const _: fn() = || {
            fn _assert_from<T: From<$input_err>>() {}
            _assert_from::<$output_err>();
        };

        impl result_transformer::async_::AsyncErrTransformer<$input_err> for $impl_for
        where
            $output_err: From<$input_err>,
        {
            type OutputErr = $output_err;
            fn transform_err_async<'a>(
                &'a self,
                err: $input_err,
            ) -> impl ::core::future::Future<Output = Self::OutputErr> + Send + 'a {
                let _ = self;
                async move { <$output_err as From<$input_err>>::from(err) }
            }
        }
    };

    (
        $impl_for:ty,
        [$input_err:ty => $output_err:ty $(,)?]
    ) => {
        result_transformer::core::async_::macros::impl_async_err_transformer_via_output_from!(
            impl_for = $impl_for,
            input_err = $input_err,
            output_err = $output_err
        );
    };
}
pub use impl_async_err_transformer_via_output_from;
