#[macro_export]
macro_rules! define_async_result_transformer_suppressing_err {
    (
        impl_for = $type:ty,
        input_ok = $input_ok:ty,
        output_ok = $output_ok:ty,
        input_err = $input_err:ty,
        output_err = $output_err:ty,
        should_suppress = $should_suppress:expr,
        err_to_ok = $err_to_ok:expr,
        err_map = $err_map:expr
    ) => {
        const _: fn() = || {
            fn _check_should_suppress(f: fn(&$input_err) -> bool) {}
            fn _check_err_to_ok(f: fn($input_err) -> $output_ok) {}
            fn _check_err_map(f: fn($input_err) -> $output_err) {}

            _check_should_suppress($should_suppress);
            _check_err_to_ok($err_to_ok);
            _check_err_map($err_map);
        };

        result_transformer::macros::define_async_result_transformer_from_err! {
            impl_for = $type,
            input_ok = $input_ok,
            output_ok = $output_ok,
            input_err = $input_err,
            output_err = $output_err,
            transform_err = |err: $input_err| {
                if ($should_suppress)(&err) {
                    Ok(($err_to_ok)(err))
                } else {
                    Err(($err_map)(err))
                }
            }
        }
    };
}
