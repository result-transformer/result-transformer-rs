#[macro_export]
macro_rules! define_const_ok_if_step_async {
    (
        name = $name:ident,
        input_ok = $input_ok:ty,
        output_ok = $output_ok:ty,
        condition = $condition:expr,
        then_flow = $then_flow:expr,
        else_flow = $else_flow:expr,
        then_flow_ty = $then_ty:ty,
        else_flow_ty = $else_ty:ty $(,)?
    ) => {
        const $name: result_transformer::flow::async_::step::OkIfStepAsync<
            $input_ok,
            $output_ok,
            fn(&$input_ok) -> std::pin::Pin<Box<dyn core::future::Future<Output = bool> + Send>>,
            std::pin::Pin<Box<dyn core::future::Future<Output = bool> + Send>>,
            $then_ty,
            $else_ty,
        > = result_transformer::flow::async_::step::OkIfStepAsync::new(
            $condition, $then_flow, $else_flow,
        );
    };

    (
        $name:ident,
        [$input_ok:ty => $output_ok:ty $(,)?],
        $condition:expr,
        $then_flow:expr,
        $else_flow:expr,
        $then_ty:ty,
        $else_ty:ty $(,)?
    ) => {
        result_transformer::flow::async_::macros::define_const_ok_if_step_async!(
            name = $name,
            input_ok = $input_ok,
            output_ok = $output_ok,
            condition = $condition,
            then_flow = $then_flow,
            else_flow = $else_flow,
            then_flow_ty = $then_ty,
            else_flow_ty = $else_ty
        );
    };
}
pub use define_const_ok_if_step_async;
