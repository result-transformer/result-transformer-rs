/// Defines a const [`OkNoopStep`].
///
/// Shorthand syntax: `($name, $ok_type)`.
///
/// # Parameters
/// - `name` - Identifier for the constant.
/// - `ok_type` - Success type handled by the step.
#[macro_export]
macro_rules! define_const_ok_noop_step_async {
    (
        name = $name:ident,
        ok_type = $ok_type:ty $(,)?
    ) => {
        const $name: result_transformer::flow::async_::step::OkNoopStep<$ok_type> =
            result_transformer::flow::async_::step::OkNoopStep::new();
    };

    (
        $name:ident,
        $ok_type:ty $(,)?
    ) => {
        result_transformer::flow::async_::macros::define_const_ok_noop_step_async!(
            name = $name,
            ok_type = $ok_type
        );
    };
}
pub use define_const_ok_noop_step_async;
