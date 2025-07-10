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
            fn _trait_check<
                T: result_transformer::async_::AsyncOkTransformer<$ok>
                    + result_transformer::async_::AsyncErrTransformer<$err>,
            >() {
            }
            _trait_check::<$ty>();
        };

        impl result_transformer::async_::AsyncResultTransformer<$ok, $err> for $ty
        where
            $ty: result_transformer::async_::AsyncOkTransformer<$ok>
                + result_transformer::async_::AsyncErrTransformer<$err>,
        {
            type OutputOk = <Self as result_transformer::async_::AsyncOkTransformer<$ok>>::OutputOk;
            type OutputErr = <Self as result_transformer::async_::AsyncErrTransformer<$err>>::OutputErr;

            fn transform_async<'a>(
                &'a self,
                result: Result<$ok, $err>,
            ) -> impl Future<Output = Result<Self::OutputOk, Self::OutputErr>> + Send + 'a{
                async move
                {
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
        }
    };
}
