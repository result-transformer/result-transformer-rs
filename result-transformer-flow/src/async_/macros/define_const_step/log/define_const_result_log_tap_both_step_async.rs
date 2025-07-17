/// Defines a const [`ResultLogTapBothStep`].
///
/// Shorthand syntax: `($name, [$ok_type, $err_type], $ok_log_level, $ok_log_format, $err_log_level, $err_log_format)`.
///
/// # Parameters
/// - `name` - Identifier for the constant.
/// - `ok_type` - Success type logged by the step.
/// - `err_type` - Error type logged by the step.
/// - `ok_log_level` - Log level for success values.
/// - `ok_log_format` - Format string for success logs.
/// - `err_log_level` - Log level for error values.
/// - `err_log_format` - Format string for error logs.
#[macro_export]
macro_rules! define_const_result_log_tap_both_step_async {
    (
        name = $name:ident,
        ok_type = $ok_type:ty,
        err_type = $err_type:ty,
        ok_log_level = $ok_log_level:expr,
        ok_log_format = $ok_log_format:expr,
        err_log_level = $err_log_level:expr,
        err_log_format = $err_log_format:expr $(,)?
    ) => {
        const $name: result_transformer::flow::async_::step::ResultLogTapBothStep<
            $ok_type,
            $err_type,
        > = result_transformer::flow::async_::step::ResultLogTapBothStep::new(
            $ok_log_level,
            $ok_log_format,
            $err_log_level,
            $err_log_format,
        );
    };

    (
        $name:ident,
        [$ok_type:ty, $err_type:ty],
        $ok_log_level:expr,
        $ok_log_format:expr,
        $err_log_level:expr,
        $err_log_format:expr $(,)?
    ) => {
        result_transformer::flow::async_::macros::define_const_result_log_tap_both_step_async!(
            name = $name,
            ok_type = $ok_type,
            err_type = $err_type,
            ok_log_level = $ok_log_level,
            ok_log_format = $ok_log_format,
            err_log_level = $err_log_level,
            err_log_format = $err_log_format
        );
    };
}
pub use define_const_result_log_tap_both_step_async;
