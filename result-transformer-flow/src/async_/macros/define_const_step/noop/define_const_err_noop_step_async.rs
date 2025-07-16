#[macro_export]
macro_rules! define_const_err_noop_step_async {
    (
        name = $name:ident,
        err_type = $err_type:ty $(,)?
    ) => {
        const $name: result_transformer::flow::async_::step::ErrNoopStep<$err_type> =
            result_transformer::flow::async_::step::ErrNoopStep::new();
    };

    (
        $name:ident,
        $err_type:ty $(,)?
    ) => {
        result_transformer::flow::async_::macros::define_const_err_noop_step_async!(
            name = $name,
            err_type = $err_type
        );
    };
}
pub use define_const_err_noop_step_async;
