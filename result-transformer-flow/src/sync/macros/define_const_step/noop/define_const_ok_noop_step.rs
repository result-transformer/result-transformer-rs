#[macro_export]
macro_rules! define_const_ok_noop_step {
    (
        name = $name:ident,
        ok_type = $ok_type:ty $(,)?
    ) => {
        const $name: result_transformer::flow::sync::step::OkNoopStep<$ok_type> =
            result_transformer::flow::sync::step::OkNoopStep::new();
    };

    (
        $name:ident,
        $ok_type:ty $(,)?
    ) => {
        result_transformer::flow::sync::macros::define_const_ok_noop_step!(
            name = $name,
            ok_type = $ok_type
        );
    };
}
pub use define_const_ok_noop_step;
