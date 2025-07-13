/// Implements [`AsyncResultTransformer`] using an existing [`ResultTransformer`] implementation.
///
/// # Parameters
/// - `impl_for` - The type to implement [`AsyncResultTransformer`] for.
/// - `input_ok` / `input_err` - Input types accepted by the synchronous transformer.
#[macro_export]
macro_rules! impl_async_result_transformer_via_self_result_transformer {
    (
        impl_for = $ty:ty,
        input_ok = $ok:ty,
        input_err = $err:ty $(,)?
    ) => {
        const _: fn() = || {
            fn _trait_check<T: result_transformer::sync::ResultTransformer<$ok, $err>>() {}
            _trait_check::<$ty>();
        };

        impl result_transformer::async_::AsyncResultTransformer<$ok, $err> for $ty
        where
            $ty: result_transformer::sync::ResultTransformer<$ok, $err>,
        {
            type OutputOk = <Self as result_transformer::sync::ResultTransformer<$ok, $err>>::OutputOk;
            type OutputErr = <Self as result_transformer::sync::ResultTransformer<$ok, $err>>::OutputErr;

            fn transform_async<'a>(
                &'a self,
                result: Result<$ok, $err>,
            ) -> impl ::core::future::Future<Output = Result<Self::OutputOk, Self::OutputErr>> + Send + 'a {
                async move { result_transformer::sync::ResultTransformer::transform(self, result) }
            }
        }
    };
}
pub use impl_async_result_transformer_via_self_result_transformer;
