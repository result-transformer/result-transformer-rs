/// Defines an asynchronous [`AsyncErrTransformer`] implementation.
///
/// # Parameters
/// - `impl_for` - Type on which to implement the trait.
/// - `input_err` - The incoming error type.
/// - `output_err` - The resulting error type.
/// - `transform_err` - Function performing the conversion.
#[macro_export]
macro_rules! define_async_err_transformer {
    (
        impl_for = $ty:ty,
        input_err = $input_err:ty,
        output_err = $output_err:ty,
        transform_err = $transform_err:expr $(,)?
    ) => {
        const _: fn() = || {
            fn _type_check<F, Fut>(_f: &F)
            where
                F: Fn($input_err) -> Fut,
                Fut: ::core::future::Future<Output = $output_err> + Send,
            {
            }
            _type_check(&$transform_err);
        };

        impl result_transformer::async_::AsyncErrTransformer<$input_err> for $ty {
            type OutputErr = $output_err;
            fn transform_err_async<'a>(
                &'a self,
                err: $input_err,
            ) -> impl Future<Output = Self::OutputErr> + Send + 'a {
                ($transform_err)(err)
            }
        }
    };
}
pub use define_async_err_transformer;
