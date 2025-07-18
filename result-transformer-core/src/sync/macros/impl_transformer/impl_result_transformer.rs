/// Implements [`ResultTransformer`] using a custom function.
///
/// Shorthand syntax: `($impl_for, [$input_ok, $input_err => $output_ok, $output_err], $transform_result)`.
///
/// # Parameters
/// - `impl_for` - Type on which to implement the trait.
/// - `input_ok` / `input_err` - Input types for the transformation function.
/// - `output_ok` / `output_err` - Output types produced by the function.
/// - `transform_result` - Function performing the conversion.
#[macro_export]
macro_rules! impl_result_transformer {
    (
        impl_for = $impl_for:ty,
        input_ok = $input_ok:ty,
        input_err = $input_err:ty,
        output_ok = $output_ok:ty,
        output_err = $output_err:ty,
        transform_result = $transform_result:expr $(,)?
    ) => {
        const _: fn() = || {
            fn _type_check(
                transform_result: fn(
                    Result<$input_ok, $input_err>,
                ) -> Result<$output_ok, $output_err>,
            ) {
            }
            _type_check($transform_result);
        };

        impl result_transformer::sync::ResultTransformer<$input_ok, $input_err> for $impl_for {
            type OutputOk = $output_ok;
            type OutputErr = $output_err;
            fn transform(
                &self,
                result: Result<$input_ok, $input_err>,
            ) -> Result<Self::OutputOk, Self::OutputErr> {
                ($transform_result)(result)
            }
        }
    };

    (
        $impl_for:ty,
        [$input_ok:ty, $input_err:ty => $output_ok:ty, $output_err:ty $(,)?],
        $transform_result:expr $(,)?
    ) => {
        result_transformer::core::sync::macros::impl_result_transformer!(
            impl_for = $impl_for,
            input_ok = $input_ok,
            input_err = $input_err,
            output_ok = $output_ok,
            output_err = $output_err,
            transform_result = $transform_result
        );
    };
}
pub use impl_result_transformer;
