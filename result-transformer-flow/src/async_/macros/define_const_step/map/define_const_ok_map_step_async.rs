#[macro_export]
macro_rules! define_const_ok_map_step_async {
    (
        name = $name:ident,
        input_ok = $input_ok:ty,
        output_ok = $output_ok:ty,
        mapper = $mapper:expr $(,)?
    ) => {
        const $name: result_transformer::flow::async_::step::OkMapStepAsync<
            $input_ok,
            $output_ok,
            fn(
                $input_ok,
            ) -> std::pin::Pin<Box<dyn core::future::Future<Output = $output_ok> + Send>>,
            std::pin::Pin<Box<dyn core::future::Future<Output = $output_ok> + Send>>,
        > = result_transformer::flow::async_::step::OkMapStepAsync::new($mapper);
    };

    (
        $name:ident,
        [$input_ok:ty => $output_ok:ty $(,)?],
        $mapper:expr $(,)?
    ) => {
        result_transformer::flow::async_::macros::define_const_ok_map_step_async!(
            name = $name,
            input_ok = $input_ok,
            output_ok = $output_ok,
            mapper = $mapper
        );
    };
}
pub use define_const_ok_map_step_async;
