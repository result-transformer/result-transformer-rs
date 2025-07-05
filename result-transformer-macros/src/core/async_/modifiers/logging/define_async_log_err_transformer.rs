#[macro_export]
macro_rules! define_async_log_err_transformer {
    (
        impl_for = $ty:ty,
        input_err = $input_err:ty,
        output_err = $output_err:ty,
        transform_err = $transform_err:expr,
        log_level = $log_level:ident,
        log_expr = $log_expr:expr
    ) => {
        const _: fn() = || {
            fn _check_display<T: std::fmt::Display>(_f: fn($input_err) -> T) {}
            _check_display($log_expr);

            fn _check_transform_err(f: fn($input_err) -> $output_err) {}
            _check_transform_err($transform_err);

            let _ = || {
                log::$log_level!("log level `{}` is valid", stringify!($log_level));
            };
        };

        result_transformer::macros::define_async_err_transformer! {
            impl_for = $ty,
            input_err = $input_err,
            output_err = $output_err,
            transform_err = |err: $input_err| {
                let msg = ($log_expr)(err);
                result_transformer::__internal::log::$log_level!("{}", msg);
                ($transform_err)(err)
            }
        }
    };
}
