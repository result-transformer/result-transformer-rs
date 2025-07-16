#[macro_export]
macro_rules! define_const_err_log_tap_step_async {
    (
        name = $name:ident,
        err_type = $err_type:ty,
        log_level = $log_level:expr,
        log_format = $log_format:expr $(,)?
    ) => {
        const $name: result_transformer::flow::async_::step::ErrLogTapStep<$err_type> =
            result_transformer::flow::async_::step::ErrLogTapStep::new($log_level, $log_format);
    };

    (
        $name:ident,
        $err_type:ty,
        $log_level:expr,
        $log_format:expr $(,)?
    ) => {
        result_transformer::flow::async_::macros::define_const_err_log_tap_step_async!(
            name = $name,
            err_type = $err_type,
            log_level = $log_level,
            log_format = $log_format
        );
    };
}
pub use define_const_err_log_tap_step_async;
