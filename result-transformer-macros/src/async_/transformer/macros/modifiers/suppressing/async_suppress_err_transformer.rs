#[macro_export]
macro_rules! async_suppress_err_transformer {
    ($ty:ty, $ok:ty, $err:ty, $log_level:ident) => {
        #[async_trait::async_trait]
        impl $crate::async_mod::ResultTransformer<$ok, $err> for $ty
        where
            $ty: $crate::async_mod::OkTransformer<$ok>,
            $ok: Default,
            $err: std::fmt::Debug,
        {
            type OutputOk = <Self as $crate::async_mod::OkTransformer<$ok>>::OutputOk;
            type OutputErr = ();

            async fn transform(
                &self,
                result: Result<$ok, $err>,
            ) -> Result<Self::OutputOk, Self::OutputErr> {
                match result {
                    Ok(v) => Ok(self.transform_ok(v).await),
                    Err(e) => {
                        log::$log_level!("Suppressed error: {:?}", e);
                        Ok(Default::default())
                    }
                }
            }
        }
    };
}
