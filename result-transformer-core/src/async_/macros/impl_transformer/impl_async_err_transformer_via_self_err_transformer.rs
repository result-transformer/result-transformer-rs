/// Implements [`AsyncErrTransformer`] by delegating to an existing
/// [`ErrTransformer`] implementation.
///
/// Shorthand syntax: `($ty, $impl_via, $input_err)`.
#[macro_export]
macro_rules! impl_async_err_transformer_via_self_err_transformer {
    (
        impl_for = $ty:ty,
        impl_via = $impl_via:ty,
        input_err = $input_err:ty,
    ) => {
        const _: fn() = || {
            fn _trait_check<T: result_transformer::sync::ErrTransformer<$input_err>>() {}
            _trait_check::<$impl_via>();
        };

        impl result_transformer::async_::AsyncErrTransformer<$input_err> for $ty {
            type OutputErr = <$impl_via as result_transformer::sync::ErrTransformer<$input_err>>::OutputErr;

            fn transform_err_async<'a>(
                &'a self,
                err: $input_err,
            ) -> impl ::core::future::Future<Output = Self::OutputErr> + Send + 'a {
                async move {
                    <$impl_via as result_transformer::sync::ErrTransformer<$input_err>>::transform_err(self, err)
                }
            }
        }
    };

    (
        $ty:ty,
        $impl_via:ty,
        $input_err:ty $(,)?
    ) => {
        result_transformer::core::async_::macros::impl_async_err_transformer_via_self_err_transformer!(
            impl_for = $ty,
            impl_via = $impl_via,
            input_err = $input_err
        );
    };
}
pub use impl_async_err_transformer_via_self_err_transformer;
