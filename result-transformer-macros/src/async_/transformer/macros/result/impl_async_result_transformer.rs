#[macro_export]
macro_rules! impl_async_result_transformer {
    ($ty:ty, $ok:ty, $err:ty) => {
        const _: fn() = || {
            fn assert_bounds<
                T: $crate::async_mod::AsyncOkTransformer<$ok> + $crate::async_mod::AsyncErrTransformer<$err>,
            >() {
            }
            assert_bounds::<$ty>();
        };

        #[async_trait::async_trait]
        impl $crate::async_mod::AsyncResultTransformer<$ok, $err> for $ty
        where
            $ty: $crate::async_mod::AsyncOkTransformer<$ok>
                + $crate::async_mod::AsyncErrTransformer<$err>,
        {
            type OutputOk = <Self as $crate::async_mod::AsyncOkTransformer<$ok>>::OutputOk;
            type OutputErr = <Self as $crate::async_mod::AsyncErrTransformer<$err>>::OutputErr;

            async fn transform_async(
                &self,
                result: Result<$ok, $err>,
            ) -> Result<Self::OutputOk, Self::OutputErr> {
                match result {
                    Ok(o) => Ok(self.transform_ok_async(o).await),
                    Err(e) => Err(self.transform_err_async(e).await),
                }
            }
        }
    };
}
