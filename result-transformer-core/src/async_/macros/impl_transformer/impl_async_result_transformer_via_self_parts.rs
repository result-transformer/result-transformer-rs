/// Implements [`AsyncResultTransformer`] when `AsyncOkTransformer` and
/// `AsyncErrTransformer` are already implemented for the target type.
///
/// Shorthand syntax: `($ty, [$ok, $err])`.
#[macro_export]
macro_rules! impl_async_result_transformer_via_self_parts {
    (
        impl_for = $impl_for:ty,
        input_ok = $input_ok:ty,
        input_err = $input_err:ty $(,)?
    ) => {
        const _: fn() = || {
            fn _trait_check<
                T: result_transformer::async_::AsyncOkTransformer<$input_ok>
                    + result_transformer::async_::AsyncErrTransformer<$input_err>,
            >() {
            }
            _trait_check::<$impl_for>();
        };

        impl result_transformer::async_::AsyncResultTransformer<$input_ok, $input_err> for $impl_for
        where
            $impl_for: result_transformer::async_::AsyncOkTransformer<$input_ok>
                + result_transformer::async_::AsyncErrTransformer<$input_err>,
        {
            type OutputOk =
                <Self as result_transformer::async_::AsyncOkTransformer<$input_ok>>::OutputOk;
            type OutputErr =
                <Self as result_transformer::async_::AsyncErrTransformer<$input_err>>::OutputErr;

            fn transform_async<'a>(
                &'a self,
                result: Result<$input_ok, $input_err>,
            ) -> impl Future<Output = Result<Self::OutputOk, Self::OutputErr>> + Send + 'a {
                async move {
                    match result {
                        Ok(ok) => Ok(<Self as result_transformer::async_::AsyncOkTransformer<
                            $input_ok,
                        >>::transform_ok_async(self, ok)
                        .await),
                        Err(err) => {
                            Err(<Self as result_transformer::async_::AsyncErrTransformer<
                                $input_err,
                            >>::transform_err_async(self, err)
                            .await)
                        }
                    }
                }
            }
        }
    };

    (
        $impl_for:ty,
        [$input_ok:ty, $input_err:ty $(,)?]
    ) => {
        result_transformer::core::async_::macros::impl_async_result_transformer_via_self_parts!(
            impl_for = $impl_for,
            input_ok = $input_ok,
            input_err = $input_err
        );
    };
}
pub use impl_async_result_transformer_via_self_parts;
