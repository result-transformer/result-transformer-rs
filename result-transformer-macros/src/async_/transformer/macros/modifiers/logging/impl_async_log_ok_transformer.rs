#[macro_export]
macro_rules! impl_async_log_ok_transformer {
    ($ty:ty, $input_ok:ty, $output_ok:ty, $level:ident) => {
        #[async_trait::async_trait]
        impl $crate::async_mod::OkTransformer<$input_ok> for $ty
        where
            $input_ok: Into<$output_ok>,
        {
            type OutputOk = $output_ok;

            async fn transform_ok(&self, ok: $input_ok) -> Self::OutputOk {
                log::$level!("transform_ok: {:?}", ok);
                ok.into()
            }
        }
    };
}
