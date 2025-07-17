/// Defines a const [`ResultInspectBothStep`].
///
/// Shorthand syntax: `($name, [$ok_type, $err_type], $ok_inspector, $err_inspector)`.
///
/// # Parameters
/// - `name` - Identifier for the constant.
/// - `ok_type` - Success type inspected by `ok_inspector`.
/// - `err_type` - Error type inspected by `err_inspector`.
/// - `ok_inspector` - Function inspecting the success value.
/// - `err_inspector` - Function inspecting the error value.
#[macro_export]
macro_rules! define_const_result_inspect_both_step {
    (
        name = $name:ident,
        ok_type = $ok_type:ty,
        err_type = $err_type:ty,
        ok_inspector = $ok_inspector:expr,
        err_inspector = $err_inspector:expr $(,)?
    ) => {
        const $name: result_transformer::flow::sync::step::ResultInspectBothStep<
            $ok_type,
            $err_type,
            fn(&$ok_type),
            fn(&$err_type),
        > = result_transformer::flow::sync::step::ResultInspectBothStep::new(
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
        result_transformer::flow::sync::macros::define_const_result_inspect_both_step!(
            name = $name,
            ok_type = $ok_type,
            err_type = $err_type,
            ok_inspector = $ok_inspector,
            err_inspector = $err_inspector
        );
    };
}
pub use define_const_result_inspect_both_step;
