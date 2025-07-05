#[macro_export]
macro_rules! define_log_ok_transformer_simple {
    (
        impl_for = $ty:ty,
        input_ok = $input_ok:ty,
        output_ok = $output_ok:ty,
        transform_ok = $transform_ok:expr,
        log_level = $log_level:ident
    ) => {
        result_transformer::macros::define_log_ok_transformer! {
            impl_for = $ty,
            input_ok = $input_ok,
            output_ok = $output_ok,
            transform_ok = $transform_ok,
            log_level = $log_level,
            log_expr = |ok| format!("transform_ok: {:?}", ok)
        }
    };
}
