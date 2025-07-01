#[macro_export]
macro_rules! impl_result_transformer {
    ($ty:ty, $ok:ty, $err:ty) => {
        const _: fn() = || {
            fn assert_bounds<
                T: result_transformer_core::sync::OkTransformer<$ok>
                    + result_transformer_core::sync::ErrTransformer<$err>,
            >() {
            }
            assert_bounds::<$ty>();
        };

        impl result_transformer_core::sync::ResultTransformer<$ok, $err> for $ty
        where
            $ty: result_transformer_core::sync::OkTransformer<$ok>
                + result_transformer_core::sync::ErrTransformer<$err>,
        {
            type OutputOk = <Self as result_transformer_core::sync::OkTransformer<$ok>>::OutputOk;
            type OutputErr =
                <Self as result_transformer_core::sync::ErrTransformer<$err>>::OutputErr;

            fn transform(
                &self,
                result: Result<$ok, $err>,
            ) -> Result<Self::OutputOk, Self::OutputErr> {
                match result {
                    Ok(o) => Ok(self.transform_ok(o)),
                    Err(e) => Err(self.transform_err(e)),
                }
            }
        }
    };
}
