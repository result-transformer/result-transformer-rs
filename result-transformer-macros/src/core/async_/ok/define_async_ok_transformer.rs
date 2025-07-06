/// Defines an asynchronous [`AsyncOkTransformer`] implementation.
///
/// # Parameters
/// - `impl_for` - Type on which to implement the trait.
/// - `input_ok` - The incoming success type.
/// - `output_ok` - The resulting success type.
/// - `transform_ok` - Function performing the conversion.
#[macro_export]
macro_rules! define_async_ok_transformer {
    (
        impl_for = $ty:ty,
        input_ok = $input_ok:ty,
        output_ok = $output_ok:ty,
        transform_ok = $transform_ok:expr $(,)?
    ) => {
        const _: fn() = || {
            fn _type_check(transform_ok: fn($input_ok) -> $output_ok) {}
            _type_check($transform_ok);
        };

        #[result_transformer::__internal::async_trait::async_trait]
        impl result_transformer::async_::AsyncOkTransformer<$input_ok> for $ty {
            type OutputOk = $output_ok;
            async fn transform_ok_async(&self, ok: $input_ok) -> Self::OutputOk {
                ($transform_ok)(ok)
            }
        }
    };
}
