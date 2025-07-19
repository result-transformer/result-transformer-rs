/// Implements [`AsyncOkTransformer`] using a custom asynchronous function.
///
/// Shorthand syntax: `($ty, [$input_ok => $output_ok], $transform_ok)`.
///
/// # Parameters
/// - `impl_for` - Type on which to implement the trait.
/// - `input_ok` - The incoming success type.
/// - `output_ok` - The resulting success type.
/// - `transform_ok` - Function performing the conversion.
#[macro_export]
macro_rules! impl_async_ok_transformer {
    (
        impl_for = $ty:ty,
        input_ok = $input_ok:ty,
        output_ok = $output_ok:ty,
        transform_ok = $transform_ok:expr $(,)?
    ) => {
        const _: fn() = || {
            fn _type_check<F, Fut>(_f: &F)
            where
                F: Fn($input_ok) -> Fut,
                Fut: ::core::future::Future<Output = $output_ok> + Send,
            {
            }
            _type_check(&$transform_ok);
        };

        impl result_transformer::async_::AsyncOkTransformer<$input_ok> for $ty {
            type OutputOk = $output_ok;
            fn transform_ok_async<'a>(
                &'a self,
                ok: $input_ok,
            ) -> impl Future<Output = Self::OutputOk> + Send + 'a {
                ($transform_ok)(ok)
            }
        }
    };

    (
        $ty:ty,
        [$input_ok:ty => $output_ok:ty $(,)?],
        $transform_ok:expr $(,)?
    ) => {
        result_transformer::core::async_::macros::impl_async_ok_transformer!(
            impl_for = $ty,
            input_ok = $input_ok,
            output_ok = $output_ok,
            transform_ok = $transform_ok
        );
    };
}
pub use impl_async_ok_transformer;
