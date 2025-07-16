#[macro_export]
macro_rules! define_const_result_map_step_async {
    (
        name = $name:ident,
        input_ok = $input_ok:ty,
        input_err = $input_err:ty,
        output_ok = $output_ok:ty,
        output_err = $output_err:ty,
        mapper = $mapper:expr $(,)?
    ) => {
        const $name: result_transformer::flow::async_::step::ResultMapStepAsync<
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
        > = result_transformer::flow::async_::step::ResultMapStepAsync::new($mapper);
    };

    (
        $name:ident,
        [$input_ok:ty, $input_err:ty => $output_ok:ty, $output_err:ty $(,)?],
        $mapper:expr $(,)?
    ) => {
        result_transformer::flow::async_::macros::define_const_result_map_step_async!(
            name = $name,
            input_ok = $input_ok,
            input_err = $input_err,
            output_ok = $output_ok,
            output_err = $output_err,
            mapper = $mapper
        );
    };
}
pub use define_const_result_map_step_async;
