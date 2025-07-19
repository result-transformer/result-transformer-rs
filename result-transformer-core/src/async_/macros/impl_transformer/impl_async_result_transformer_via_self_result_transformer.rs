/// Implements [`AsyncResultTransformer`] using an existing [`ResultTransformer`] implementation.
///
/// Shorthand syntax: `($impl_for [$input_ok, $input_err])`.
///
/// # Parameters
/// - `impl_for` - The type to implement [`AsyncResultTransformer`] for.
/// - `input_ok` / `input_err` - Input types accepted by the synchronous transformer.
#[macro_export]
macro_rules! impl_async_result_transformer_via_self_result_transformer {
    (
        impl_for = $impl_for:ty,
        input_ok = $input_ok:ty,
        input_err = $input_err:ty $(,)?
    ) => {
        const _: fn() = || {
            fn _trait_check<T: result_transformer::sync::ResultTransformer<$input_ok, $input_err>>() {}
            _trait_check::<$impl_for>();
        };

        impl result_transformer::async_::AsyncResultTransformer<$input_ok, $input_err> for $impl_for
        where
            $impl_for: result_transformer::sync::ResultTransformer<$input_ok, $input_err>,
        {
            type OutputOk = <Self as result_transformer::sync::ResultTransformer<$input_ok, $input_err>>::OutputOk;
            type OutputErr = <Self as result_transformer::sync::ResultTransformer<$input_ok, $input_err>>::OutputErr;

            fn transform_async<'a>(
                &'a self,
                result: Result<$input_ok, $input_err>,
            ) -> impl ::core::future::Future<Output = Result<Self::OutputOk, Self::OutputErr>> + Send + 'a {
                async move { result_transformer::sync::ResultTransformer::transform(self, result) }
            }
        }
    };

    (
        $impl_for:ty,
        [$input_ok:ty, $input_err:ty $(,)?]
    ) => {
        result_transformer::core::async_::macros::impl_async_result_transformer_via_self_result_transformer!(
            impl_for = $impl_for,
            input_ok = $input_ok,
            input_err = $input_err
        );
    };
}
pub use impl_async_result_transformer_via_self_result_transformer;
