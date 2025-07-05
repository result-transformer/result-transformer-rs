#[macro_export]
macro_rules! define_async_log_err_transformer_simple {
    (
        impl_for = $ty:ty,
        input_err = $input_err:ty,
        output_err = $output_err:ty,
        transform_err = $transform_err:expr,
        log_level = $log_level:ident
    ) => {
        result_transformer::macros::define_async_log_err_transformer! {
            impl_for = $ty,
            input_err = $input_err,
            output_err = $output_err,
            transform_err = $transform_err,
            log_level = $log_level,
            log_expr = |err| format!("transform_err: {:?}", err)
        }
    };
}
