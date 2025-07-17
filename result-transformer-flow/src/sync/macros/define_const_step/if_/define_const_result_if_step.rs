/// Defines a const [`ResultIfStep`].
///
/// Shorthand syntax: `($name, [$input_ok, $input_err => $output_ok, $output_err], $condition, $then_flow, $else_flow, $then_ty, $else_ty)`.
///
/// # Parameters
/// - `name` - Identifier for the constant.
/// - `input_ok` - Success type accepted by the flows.
/// - `input_err` - Error type accepted by the flows.
/// - `output_ok` - Success type returned by the flows.
/// - `output_err` - Error type returned by the flows.
/// - `condition` - Predicate evaluated on the [`Result`] value.
/// - `then_flow` - Flow executed when the predicate is `true`.
/// - `else_flow` - Flow executed when the predicate is `false`.
/// - `then_flow_ty` - Type of `then_flow`.
/// - `else_flow_ty` - Type of `else_flow`.
#[macro_export]
macro_rules! define_const_result_if_step {
    (
        name = $name:ident,
        input_ok = $input_ok:ty,
        input_err = $input_err:ty,
        output_ok = $output_ok:ty,
        output_err = $output_err:ty,
        condition = $condition:expr,
        then_flow = $then_flow:expr,
        else_flow = $else_flow:expr,
        then_flow_ty = $then_ty:ty,
        else_flow_ty = $else_ty:ty $(,)?
    ) => {
        const $name: result_transformer::flow::sync::step::ResultIfStep<
            $input_ok,
            $input_err,
            $output_ok,
            $output_err,
            fn(&Result<$input_ok, $input_err>) -> bool,
            $then_ty,
            $else_ty,
        > = result_transformer::flow::sync::step::ResultIfStep::new(
            $condition, $then_flow, $else_flow,
        );
    };

    (
        $name:ident,
        [$input_ok:ty, $input_err:ty => $output_ok:ty, $output_err:ty $(,)?],
        $condition:expr,
        $then_flow:expr,
        $else_flow:expr,
        $then_ty:ty,
        $else_ty:ty $(,)?
    ) => {
        result_transformer::flow::sync::macros::define_const_result_if_step!(
            name = $name,
            input_ok = $input_ok,
            input_err = $input_err,
            output_ok = $output_ok,
            output_err = $output_err,
            condition = $condition,
            then_flow = $then_flow,
            else_flow = $else_flow,
            then_flow_ty = $then_ty,
            else_flow_ty = $else_ty
        );
    };
}
pub use define_const_result_if_step;
