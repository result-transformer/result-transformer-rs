/// Defines a const [`ResultTapStepAsync`].
///
/// Shorthand syntax: `($name, [$input_ok, $input_err => $output_ok, $output_err], $tap)`.
///
/// # Parameters
/// - `name` - Identifier for the constant.
/// - `input_ok` - Success type consumed by the async tap function.
/// - `input_err` - Error type consumed by the async tap function.
/// - `output_ok` - Success type returned by the async tap function.
/// - `output_err` - Error type returned by the async tap function.
/// - `tap` - Asynchronous function called with the [`Result`] value.
#[macro_export]
macro_rules! define_const_result_tap_step_async {
    (
        name = $name:ident,
        input_ok = $input_ok:ty,
        input_err = $input_err:ty,
        output_ok = $output_ok:ty,
        output_err = $output_err:ty,
        tap = $tap:expr $(,)?
    ) => {
        const $name: result_transformer::flow::async_::step::ResultTapStepAsync<
            $input_ok,
            $input_err,
            $output_ok,
            $output_err,
            fn(
                Result<$input_ok, $input_err>,
            ) -> std::pin::Pin<
                Box<dyn core::future::Future<Output = Result<$output_ok, $output_err>> + Send>,
            >,
            std::pin::Pin<
                Box<dyn core::future::Future<Output = Result<$output_ok, $output_err>> + Send>,
            >,
        > = result_transformer::flow::async_::step::ResultTapStepAsync::new($tap);
    };

    (
        $name:ident,
        [$input_ok:ty, $input_err:ty => $output_ok:ty, $output_err:ty $(,)?],
        $tap:expr $(,)?
    ) => {
        result_transformer::flow::async_::macros::define_const_result_tap_step_async!(
            name = $name,
            input_ok = $input_ok,
            input_err = $input_err,
            output_ok = $output_ok,
            output_err = $output_err,
            tap = $tap
        );
    };
}
pub use define_const_result_tap_step_async;
