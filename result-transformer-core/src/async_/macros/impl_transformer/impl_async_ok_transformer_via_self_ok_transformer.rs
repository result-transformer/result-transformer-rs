/// Implements [`AsyncOkTransformer`] by delegating to an existing
/// [`OkTransformer`] implementation.
///
/// Shorthand syntax: `($ty, $impl_via, $input_ok)`.
#[macro_export]
macro_rules! impl_async_ok_transformer_via_self_ok_transformer {
    (
        impl_for = $ty:ty,
        impl_via = $impl_via:ty,
        input_ok = $input_ok:ty,
    ) => {
        const _: fn() = || {
            fn _trait_check<T: result_transformer::sync::OkTransformer<$input_ok>>() {}
            _trait_check::<$impl_via>();
        };

        impl result_transformer::async_::AsyncOkTransformer<$input_ok> for $ty {
            type OutputOk =
                <$impl_via as result_transformer::sync::OkTransformer<$input_ok>>::OutputOk;

            fn transform_ok_async<'a>(
                &'a self,
                ok: $input_ok,
            ) -> impl ::core::future::Future<Output = Self::OutputOk> + Send + 'a {
                async move {
                    <$impl_via as result_transformer::sync::OkTransformer<$input_ok>>::transform_ok(
                        self, ok,
                    )
                }
            }
        }
    };

    (
        $ty:ty,
        $impl_via:ty,
        $input_ok:ty $(,)?
    ) => {
        result_transformer::core::async_::macros::impl_async_ok_transformer_via_self_ok_transformer!(
            impl_for = $ty,
            impl_via = $impl_via,
            input_ok = $input_ok
        );
    };
}
pub use impl_async_ok_transformer_via_self_ok_transformer;
