#[macro_export]
macro_rules! define_const_result_noop_step_async {
    (
        name = $name:ident,
        ok_type = $ok_type:ty,
        err_type = $err_type:ty $(,)?
    ) => {
        const $name: result_transformer::flow::async_::step::ResultNoopStep<$ok_type, $err_type> =
            result_transformer::flow::async_::step::ResultNoopStep::new();
    };

    (
        $name:ident,
        [$ok_type:ty, $err_type:ty $(,)?]
    ) => {
        result_transformer::flow::async_::macros::define_const_result_noop_step_async!(
            name = $name,
            ok_type = $ok_type,
            err_type = $err_type
        );
    };
}
pub use define_const_result_noop_step_async;
