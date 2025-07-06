/// Generates an implementation of [`AsyncResultTransformer`] for a given type.
///
/// The macro expects a function capable of converting
/// `Result<input_ok, input_err>` into `Result<output_ok, output_err>` and uses
/// it to implement the trait.  This is particularly useful when you want to
/// adapt an existing async function into the trait without writing the boiler-
/// plate by hand.
///
/// # Parameters
/// - `impl_for` - The type on which to implement `AsyncResultTransformer`.
/// - `input_ok` / `input_err` - The incoming `Ok` and `Err` types.
/// - `output_ok` / `output_err` - The resulting `Ok` and `Err` types.
/// - `transform_result` - Function performing the actual conversion.
#[macro_export]
macro_rules! define_async_result_transformer {
    (
        impl_for = $ty:ty,
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

        #[result_transformer::__internal::async_trait::async_trait]
        impl result_transformer::async_::AsyncResultTransformer<$input_ok, $input_err> for $ty {
            type OutputOk = $output_ok;
            type OutputErr = $output_err;
            async fn transform_async(
                &self,
                result: Result<$input_ok, $input_err>,
            ) -> Result<Self::OutputOk, Self::OutputErr> {
                ($transform_result)(result)
            }
        }
    };
}
