#[macro_export]
macro_rules! impl_log_err_transformer {
    ($ty:ty, $input_err:ty, $output_err:ty, $level:ident) => {
        const _: fn() = || {
            let _ = || {
                log::$level!("log level `{}` is valid", stringify!($level));
            };
        };

        impl result_transformer_core::sync::ErrTransformer<$input_err> for $ty
        where
            $input_err: Into<$output_err>,
        {
            type OutputErr = $output_err;

            fn transform_err(&self, err: $input_err) -> Self::OutputErr {
                log::$level!("transform_err: {:?}", err);
                err.into()
            }
        }
    };
}
