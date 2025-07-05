#[macro_export]
macro_rules! define_async_err_transformer {
    (
        impl_for = $ty:ty,
        input_err = $input_err:ty,
        output_err = $output_err:ty,
        transform_err = $transform_err:expr
    ) => {
        const _: fn() = || {
            fn _type_check(transform_err: fn($input_err) -> $output_err) {}
            _type_check($transform_err);
        };

        #[result_transformer::__internal::async_trait::async_trait]
        impl result_transformer::async_::AsyncErrTransformer<$input_err> for $ty {
            type OutputErr = $output_err;
            async fn transform_err_async(&self, err: $input_err) -> Self::OutputErr {
                ($transform_err)(err)
            }
        }
    };
}
