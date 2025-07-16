#[macro_export]
macro_rules! define_const_err_noop_step {
    (
        name = $name:ident,
        err_type = $err_type:ty $(,)?
    ) => {
        const $name: result_transformer::flow::sync::step::ErrNoopStep<$err_type> =
            result_transformer::flow::sync::step::ErrNoopStep::new();
    };

    (
        $name:ident,
        $err_type:ty $(,)?
    ) => {
        result_transformer::flow::sync::macros::define_const_err_noop_step!(
            name = $name,
            err_type = $err_type
        );
    };
}
pub use define_const_err_noop_step;
