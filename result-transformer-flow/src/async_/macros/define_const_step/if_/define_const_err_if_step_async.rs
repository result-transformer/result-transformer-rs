/// Defines a const [`ErrIfStepAsync`].
///
/// Shorthand syntax: `($name, [$input_err => $output_err], $condition, $then_flow, $else_flow, $then_ty, $else_ty)`.
///
/// # Parameters
/// - `name` - Identifier for the constant.
/// - `input_err` - Error type accepted by the flows.
/// - `output_err` - Error type returned by the flows.
/// - `condition` - Async predicate evaluated on the error value.
/// - `then_flow` - Flow executed when the predicate is `true`.
/// - `else_flow` - Flow executed when the predicate is `false`.
/// - `then_flow_ty` - Type of `then_flow`.
/// - `else_flow_ty` - Type of `else_flow`.
#[macro_export]
macro_rules! define_const_err_if_step_async {
    (
        name = $name:ident,
        input_err = $input_err:ty,
        output_err = $output_err:ty,
        condition = $condition:expr,
        then_flow = $then_flow:expr,
        else_flow = $else_flow:expr,
        then_flow_ty = $then_ty:ty,
        else_flow_ty = $else_ty:ty $(,)?
    ) => {
        const $name: result_transformer::flow::async_::step::ErrIfStepAsync<
            $input_err,
            $output_err,
            fn(&$input_err) -> std::pin::Pin<Box<dyn core::future::Future<Output = bool> + Send>>,
            std::pin::Pin<Box<dyn core::future::Future<Output = bool> + Send>>,
            $then_ty,
            $else_ty,
        > = result_transformer::flow::async_::step::ErrIfStepAsync::new(
            $condition, $then_flow, $else_flow,
        );
    };

    (
        $name:ident,
        [$input_err:ty => $output_err:ty $(,)?],
        $condition:expr,
        $then_flow:expr,
        $else_flow:expr,
        $then_ty:ty,
        $else_ty:ty $(,)?
    ) => {
        result_transformer::flow::async_::macros::define_const_err_if_step_async!(
            name = $name,
            input_err = $input_err,
            output_err = $output_err,
            condition = $condition,
            then_flow = $then_flow,
            else_flow = $else_flow,
            then_flow_ty = $then_ty,
            else_flow_ty = $else_ty
        );
    };
}
pub use define_const_err_if_step_async;
