/// Defines a const [`ErrInspectStep`].
///
/// Shorthand syntax: `($name, $err_type, $inspector)`.
///
/// # Parameters
/// - `name` - Identifier for the constant.
/// - `err_type` - Error type inspected by the step.
/// - `inspector` - Function receiving a reference to the error value.
#[macro_export]
macro_rules! define_const_err_inspect_step {
    (
        name = $name:ident,
        err_type = $err_type:ty,
        inspector = $inspector:expr $(,)?
    ) => {
        const $name: result_transformer::flow::sync::step::ErrInspectStep<
            $err_type,
            fn(&$err_type),
        > = result_transformer::flow::sync::step::ErrInspectStep::new($inspector);
    };

    (
        $name:ident,
        $err_type:ty,
        $inspector:expr $(,)?
    ) => {
        result_transformer::flow::sync::macros::define_const_err_inspect_step!(
            name = $name,
            err_type = $err_type,
            inspector = $inspector
        );
    };
}
pub use define_const_err_inspect_step;
