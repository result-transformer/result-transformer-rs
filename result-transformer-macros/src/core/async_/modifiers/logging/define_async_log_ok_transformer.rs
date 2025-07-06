#[macro_export]
macro_rules! define_async_log_ok_transformer {
    (
        impl_for = $ty:ty,
        input_ok = $input_ok:ty,
        output_ok = $output_ok:ty,
        transform_ok = $transform_ok:expr,
        log_level = $log_level:ident,
        log_expr = $log_expr:expr $(,)?
    ) => {
        const _: fn() = || {
            fn _check_display<T: std::fmt::Display>(_f: fn($input_ok) -> T) {}
            _check_display($log_expr);

            fn _check_transform_ok(f: fn($input_ok) -> $output_ok) {}
            _check_transform_ok($transform_ok);

            let _ = || {
                log::$log_level!("log level `{}` is valid", stringify!($log_level));
            };
        };

        result_transformer::macros::define_async_ok_transformer! {
            impl_for = $ty,
            input_ok = $input_ok,
            output_ok = $output_ok,
            transform_ok = |ok: $input_ok| {
                let msg = ($log_expr)(ok);
                result_transformer::__internal::log::$log_level!("{}", msg);
                ($transform_ok)(ok)
            }
        }
    };
}
