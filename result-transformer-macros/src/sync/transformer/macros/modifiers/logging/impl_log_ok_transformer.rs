#[macro_export]
macro_rules! impl_log_ok_transformer {
    ($ty:ty, $input_ok:ty, $output_ok:ty, $level:ident) => {
        const _: fn() = || {
            let _ = || {
                log::$level!("log level `{}` is valid", stringify!($level));
            };
        };

        impl result_transformer_core::sync::OkTransformer<$input_ok> for $ty
        where
            $input_ok: Into<$output_ok>,
        {
            type OutputOk = $output_ok;

            fn transform_ok(&self, ok: $input_ok) -> Self::OutputOk {
                log::$level!("transform_ok: {:?}", ok);
                ok.into()
            }
        }
    };
}
