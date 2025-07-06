#[macro_export]
macro_rules! define_result_transformer_from_ok {
    (
        impl_for = $type:ty,
        input_ok = $input_ok:ty,
        input_err = $input_err:ty,
        output_ok = $output_ok:ty,
        output_err = $output_err:ty,
        transform_ok = $transform_ok:expr $(,)?
    ) => {
        const _: fn() = || {
            fn _type_check(f: fn($input_ok) -> Result<$output_ok, $output_err>) {}
            _type_check($transform_ok);
        };

        result_transformer::macros::define_result_transformer! {
            impl_for = $type,
            input_ok = $input_ok,
            input_err = $input_err,
            output_ok = $output_ok,
            output_err = $output_err,
            transform_result = |result: Result<$input_ok, $input_err>| {
                match result {
                    Ok(ok) => ($transform_ok)(ok),
                    Err(err) => Err(err),
                }
            }
        }
    };
}
