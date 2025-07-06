/// Implements [`AsyncResultTransformer`] when `AsyncOkTransformer` and
/// `AsyncErrTransformer` are already implemented for the target type.
#[macro_export]
macro_rules! impl_async_result_transformer_from_parts {
    (
        impl_for = $ty:ty,
        input_ok = $ok:ty,
        input_err = $err:ty $(,)?
    ) => {
        const _: fn() = || {
            fn assert_bounds<
                T: result_transformer::async_::AsyncOkTransformer<$ok>
                    + result_transformer::async_::AsyncErrTransformer<$err>,
            >() {
            }
            assert_bounds::<$ty>();
        };

        #[result_transformer::__internal::async_trait::async_trait]
        impl result_transformer::async_::AsyncResultTransformer<$ok, $err> for $ty
        where
            $ty: result_transformer::async_::AsyncOkTransformer<$ok>
                + result_transformer::async_::AsyncErrTransformer<$err>,
        {
            type OutputOk = <Self as result_transformer::async_::AsyncOkTransformer<$ok>>::OutputOk;
            type OutputErr = <Self as result_transformer::async_::AsyncErrTransformer<$err>>::OutputErr;

            async fn transform_async(
                &self,
                result: Result<$ok, $err>,
            ) -> Result<Self::OutputOk, Self::OutputErr> {
                match result {
                    Ok(ok) => Ok(
                        <Self as result_transformer::async_::AsyncOkTransformer<$ok>>::transform_ok_async(
                            self, ok,
                        ).await,
                    ),
                    Err(err) => Err(
                        <Self as result_transformer::async_::AsyncErrTransformer<$err>>::transform_err_async(
                            self, err,
                        ).await,
                    ),
                }
            }
        }
    };
}
