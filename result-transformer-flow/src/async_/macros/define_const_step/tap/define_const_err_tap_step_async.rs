#[macro_export]
macro_rules! define_const_err_tap_step_async {
    (
        name = $name:ident,
        input_err = $input_err:ty,
        output_err = $output_err:ty,
        tap = $tap:expr $(,)?
    ) => {
        const $name: result_transformer::flow::async_::step::ErrTapStepAsync<
            $input_err,
            $output_err,
            fn(
                $input_err,
            ) -> std::pin::Pin<Box<dyn core::future::Future<Output = $output_err> + Send>>,
            std::pin::Pin<Box<dyn core::future::Future<Output = $output_err> + Send>>,
        > = result_transformer::flow::async_::step::ErrTapStepAsync::new($tap);
    };

    (
        $name:ident,
        [$input_err:ty => $output_err:ty $(,)?],
        $tap:expr $(,)?
    ) => {
        result_transformer::flow::async_::macros::define_const_err_tap_step_async!(
            name = $name,
            input_err = $input_err,
            output_err = $output_err,
            tap = $tap
        );
    };
}
pub use define_const_err_tap_step_async;
