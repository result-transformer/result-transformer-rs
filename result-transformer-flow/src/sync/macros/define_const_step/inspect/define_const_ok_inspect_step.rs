/// Defines a const [`OkInspectStep`].
///
/// Shorthand syntax: `($name, $ok_type, $inspector)`.
///
/// # Parameters
/// - `name` - Identifier for the constant.
/// - `ok_type` - Success type inspected by the step.
/// - `inspector` - Function receiving a reference to the success value.
#[macro_export]
macro_rules! define_const_ok_inspect_step {
    (
        name = $name:ident,
        ok_type = $ok_type:ty,
        inspector = $inspector:expr $(,)?
    ) => {
        const $name: result_transformer::flow::sync::step::OkInspectStep<$ok_type, fn(&$ok_type)> =
            result_transformer::flow::sync::step::OkInspectStep::new($inspector);
    };

    (
        $name:ident,
        $ok_type:ty,
        $inspector:expr $(,)?
    ) => {
        result_transformer::flow::sync::macros::define_const_ok_inspect_step!(
            name = $name,
            ok_type = $ok_type,
            inspector = $inspector
        );
    };
}
pub use define_const_ok_inspect_step;
