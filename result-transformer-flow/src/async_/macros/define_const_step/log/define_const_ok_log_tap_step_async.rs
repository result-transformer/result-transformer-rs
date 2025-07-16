#[macro_export]
macro_rules! define_const_ok_log_tap_step_async {
    (
        name = $name:ident,
        ok_type = $ok_type:ty,
        log_level = $log_level:expr,
        log_format = $log_format:expr $(,)?
    ) => {
        const $name: result_transformer::flow::async_::step::OkLogTapStep<$ok_type> =
            result_transformer::flow::async_::step::OkLogTapStep::new($log_level, $log_format);
    };

    (
        $name:ident,
        $ok_type:ty,
        $log_level:expr,
        $log_format:expr $(,)?
    ) => {
        result_transformer::flow::async_::macros::define_const_ok_log_tap_step_async!(
            name = $name,
            ok_type = $ok_type,
            log_level = $log_level,
            log_format = $log_format
        );
    };
}
pub use define_const_ok_log_tap_step_async;
