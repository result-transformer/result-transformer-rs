/// Helper macro for implementing [`ResultTransformer`] when only the error path
/// needs mapping.
///
/// # Parameters
/// - `impl_for` - Type on which to implement the trait.
/// - `input_ok` / `input_err` - Input types of the original result.
/// - `output_ok` / `output_err` - Output types to produce.
/// - `transform_err` - Function applied when the input result is `Err`.
#[macro_export]
macro_rules! impl_result_transformer_via_err_transform_fn {
    (
        impl_for = $type:ty,
        input_ok = $input_ok:ty,
        input_err = $input_err:ty,
        output_ok = $output_ok:ty,
        output_err = $output_err:ty,
        transform_err = $transform_err:expr $(,)?
    ) => {
        const _: fn() = || {
            fn _type_check(f: fn($input_err) -> Result<$output_ok, $output_err>) {}
            _type_check($transform_err);
        };

        result_transformer::core::sync::macros::impl_result_transformer! {
            impl_for = $type,
            input_ok = $input_ok,
            input_err = $input_err,
            output_ok = $output_ok,
            output_err = $output_err,
            transform_result = |result: Result<$input_ok, $input_err>| {
                match result {
                    Ok(ok) => Ok(ok),
                    Err(err) => ($transform_err)(err),
                }
            }
        }
    };
}
pub use impl_result_transformer_via_err_transform_fn;
