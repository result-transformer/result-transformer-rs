#[cfg(feature = "logging")]
#[macro_export]
macro_rules! define_const_result_log_tap_both_step {
    (
        name = $name:ident,
        ok_type = $ok_type:ty,
        err_type = $err_type:ty,
        ok_log_level = $ok_log_level:expr,
        ok_log_format = $ok_log_format:expr,
        err_log_level = $err_log_level:expr,
        err_log_format = $err_log_format:expr $(,)?
    ) => {
        const $name: result_transformer::flow::sync::step::ResultLogTapBothStep<
            $ok_type,
            $err_type,
        > = result_transformer::flow::sync::step::ResultLogTapBothStep::new(
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
        result_transformer::flow::sync::macros::define_const_result_log_tap_both_step!(
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
#[cfg(feature = "logging")]
pub use define_const_result_log_tap_both_step;
