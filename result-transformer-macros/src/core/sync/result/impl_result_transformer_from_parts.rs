#[macro_export]
macro_rules! impl_result_transformer_from_parts {
    (
        impl_for = $ty:ty,
        input_ok = $ok:ty,
        input_err = $err:ty $(,)?
    ) => {
        const _: fn() = || {
            fn assert_bounds<
                T: result_transformer::sync::OkTransformer<$ok>
                    + result_transformer::sync::ErrTransformer<$err>,
            >() {
            }
            assert_bounds::<$ty>();
        };

        impl result_transformer::sync::ResultTransformer<$ok, $err> for $ty
        where
            $ty: result_transformer::sync::OkTransformer<$ok>
                + result_transformer::sync::ErrTransformer<$err>,
        {
            type OutputOk = <Self as result_transformer::sync::OkTransformer<$ok>>::OutputOk;
            type OutputErr = <Self as result_transformer::sync::ErrTransformer<$err>>::OutputErr;

            fn transform(
                &self,
                result: Result<$ok, $err>,
            ) -> Result<Self::OutputOk, Self::OutputErr> {
                match result {
                    Ok(o) => Ok(
                        <Self as result_transformer::sync::OkTransformer<$ok>>::transform_ok(
                            self, o,
                        ),
                    ),
                    Err(e) => Err(
                        <Self as result_transformer::sync::ErrTransformer<$err>>::transform_err(
                            self, e,
                        ),
                    ),
                }
            }
        }
    };
}
