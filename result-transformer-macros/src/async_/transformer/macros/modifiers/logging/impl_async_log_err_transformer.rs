#[macro_export]
macro_rules! impl_async_log_err_transformer {
    ($ty:ty, $input_err:ty, $output_err:ty, $level:ident) => {
        #[async_trait::async_trait]
        impl $crate::async_mod::ErrTransformer<$input_err> for $ty
        where
            $input_err: Into<$output_err>,
        {
            type OutputErr = $output_err;

            async fn transform_err(&self, err: $input_err) -> Self::OutputErr {
                log::$level!("transform_err: {:?}", err);
                err.into()
            }
        }
    };
}
