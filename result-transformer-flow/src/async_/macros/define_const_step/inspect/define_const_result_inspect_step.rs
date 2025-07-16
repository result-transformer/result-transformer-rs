#[macro_export]
macro_rules! define_const_result_inspect_step_async {
    (
        name = $name:ident,
        ok_type = $ok_type:ty,
        err_type = $err_type:ty,
        inspector = $inspector:expr $(,)?
    ) => {
        const $name: result_transformer::flow::async_::step::ResultInspectStepAsync<
            $ok_type,
            $err_type,
            fn(
                &Result<$ok_type, $err_type>,
            ) -> std::pin::Pin<Box<dyn core::future::Future<Output = ()> + Send>>,
            std::pin::Pin<Box<dyn core::future::Future<Output = ()> + Send>>,
        > = result_transformer::flow::async_::step::ResultInspectStepAsync::new($inspector);
    };

    (
        $name:ident,
        [$ok_type:ty, $err_type:ty],
        $inspector:expr $(,)?
    ) => {
        result_transformer::flow::async_::macros::define_const_result_inspect_step_async!(
            name = $name,
            ok_type = $ok_type,
            err_type = $err_type,
            inspector = $inspector
        );
    };
}
pub use define_const_result_inspect_step_async;
