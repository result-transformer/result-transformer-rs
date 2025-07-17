/// Defines a const [`OkTapStepAsync`].
///
/// Shorthand syntax: `($name, [$input_ok => $output_ok], $tap)`.
///
/// # Parameters
/// - `name` - Identifier for the constant.
/// - `input_ok` - Success type consumed by the async tap function.
/// - `output_ok` - Success type returned by the async tap function.
/// - `tap` - Asynchronous function called with the success value.
#[macro_export]
macro_rules! define_const_ok_tap_step_async {
    (
        name = $name:ident,
        input_ok = $input_ok:ty,
        output_ok = $output_ok:ty,
        tap = $tap:expr $(,)?
    ) => {
        const $name: result_transformer::flow::async_::step::OkTapStepAsync<
            $input_ok,
            $output_ok,
            fn(
                $input_ok,
            ) -> std::pin::Pin<Box<dyn core::future::Future<Output = $output_ok> + Send>>,
            std::pin::Pin<Box<dyn core::future::Future<Output = $output_ok> + Send>>,
        > = result_transformer::flow::async_::step::OkTapStepAsync::new($tap);
    };

    (
        $name:ident,
        [$input_ok:ty => $output_ok:ty $(,)?],
        $tap:expr $(,)?
    ) => {
        result_transformer::flow::async_::macros::define_const_ok_tap_step_async!(
            name = $name,
            input_ok = $input_ok,
            output_ok = $output_ok,
            tap = $tap
        );
    };
}
pub use define_const_ok_tap_step_async;
