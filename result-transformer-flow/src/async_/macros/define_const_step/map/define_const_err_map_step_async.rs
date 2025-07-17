/// Defines a const [`ErrMapStepAsync`].
///
/// Shorthand syntax: `($name, [$input_err => $output_err], $mapper)`.
///
/// # Parameters
/// - `name` - Identifier for the constant.
/// - `input_err` - Error type accepted by the async mapper.
/// - `output_err` - Error type produced by the async mapper.
/// - `mapper` - Asynchronous function converting the error value.
#[macro_export]
macro_rules! define_const_err_map_step_async {
    (
        name = $name:ident,
        input_err = $input_err:ty,
        output_err = $output_err:ty,
        mapper = $mapper:expr $(,)?
    ) => {
        const $name: result_transformer::flow::async_::step::ErrMapStepAsync<
            $input_err,
            $output_err,
            fn(
                $input_err,
            ) -> std::pin::Pin<Box<dyn core::future::Future<Output = $output_err> + Send>>,
            std::pin::Pin<Box<dyn core::future::Future<Output = $output_err> + Send>>,
        > = result_transformer::flow::async_::step::ErrMapStepAsync::new($mapper);
    };

    (
        $name:ident,
        [$input_err:ty => $output_err:ty $(,)?],
        $mapper:expr $(,)?
    ) => {
        result_transformer::flow::async_::macros::define_const_err_map_step_async!(
            name = $name,
            input_err = $input_err,
            output_err = $output_err,
            mapper = $mapper
        );
    };
}
pub use define_const_err_map_step_async;
