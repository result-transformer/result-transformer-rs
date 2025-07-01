#[macro_export]
macro_rules! suppress_err_transformer {
    ($ty:ty, $ok:ty, $err:ty, $log_level:ident) => {
        impl result_transformer_core::sync::ResultTransformer<$ok, $err> for $ty
        where
            $ty: result_transformer_core::sync::OkTransformer<$ok>,
            $ok: Default,
            $err: std::fmt::Debug,
        {
            type OutputOk = <Self as result_transformer_core::sync::OkTransformer<$ok>>::OutputOk;
            type OutputErr = ();

            fn transform(
                &self,
                result: Result<$ok, $err>,
            ) -> Result<Self::OutputOk, Self::OutputErr> {
                match result {
                    Ok(v) => Ok(self.transform_ok(v)),
                    Err(e) => {
                        log::$log_level!("Suppressed error: {:?}", e);
                        Ok(Default::default())
                    }
                }
            }
        }
    };
}
