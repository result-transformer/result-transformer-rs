#[macro_export]
macro_rules! define_const_result_inspect_both_step_async {
    (
        name = $name:ident,
        ok_type = $ok_type:ty,
        err_type = $err_type:ty,
        ok_inspector = $ok_inspector:expr,
        err_inspector = $err_inspector:expr $(,)?
    ) => {
        const $name: result_transformer::flow::async_::step::ResultInspectBothStepAsync<
            $ok_type,
            $err_type,
            fn(&$ok_type) -> std::pin::Pin<Box<dyn core::future::Future<Output = ()> + Send>>,
            fn(&$err_type) -> std::pin::Pin<Box<dyn core::future::Future<Output = ()> + Send>>,
            std::pin::Pin<Box<dyn core::future::Future<Output = ()> + Send>>,
            std::pin::Pin<Box<dyn core::future::Future<Output = ()> + Send>>,
        > = result_transformer::flow::async_::step::ResultInspectBothStepAsync::new(
            $ok_inspector,
            $err_inspector,
        );
    };

    (
        $name:ident,
        [$ok_type:ty, $err_type:ty],
        $ok_inspector:expr,
        $err_inspector:expr $(,)?
    ) => {
        result_transformer::flow::async_::macros::define_const_result_inspect_both_step_async!(
            name = $name,
            ok_type = $ok_type,
            err_type = $err_type,
            ok_inspector = $ok_inspector,
            err_inspector = $err_inspector
        );
    };
}
pub use define_const_result_inspect_both_step_async;
