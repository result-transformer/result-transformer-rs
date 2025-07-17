/// Implements [`ErrTransformer`] using a custom function.
///
/// Shorthand syntax: `($impl_for, [$input_err => $output_err], $transform_err)`.
///
/// # Parameters
/// - `impl_for` - Type on which to implement the trait.
/// - `input_err` - The incoming error type.
/// - `output_err` - The resulting error type.
/// - `transform_err` - Function performing the conversion.
#[macro_export]
macro_rules! impl_err_transformer {
    (
        impl_for = $impl_for:ty,
        input_err = $input_err:ty,
        output_err = $output_err:ty,
        transform_err = $transform_err:expr $(,)?
    ) => {
        const _: fn() = || {
            fn _type_check(transform_err: fn($input_err) -> $output_err) {}
            _type_check($transform_err);
        };

        impl result_transformer::sync::ErrTransformer<$input_err> for $impl_for {
            type OutputErr = $output_err;
            fn transform_err(&self, err: $input_err) -> Self::OutputErr {
                ($transform_err)(err)
            }
        }
    };

    (
        $impl_for:ty,
        [$input_err:ty => $output_err:ty $(,)?],
        $transform_err:expr $(,)?
    ) => {
        result_transformer::core::sync::macros::impl_err_transformer!(
            impl_for = $impl_for,
            input_err = $input_err,
            output_err = $output_err,
            transform_err = $transform_err
        );
    };
}
pub use impl_err_transformer;
