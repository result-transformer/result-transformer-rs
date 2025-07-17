/// Defines a const [`ResultTapBothBindStepAsync`].
///
/// Shorthand syntax: `($name, [$input_ok, $input_err => $output_ok, $output_err], $ok_tap, $err_tap)`.
///
/// # Parameters
/// - `name` - Identifier for the constant.
/// - `input_ok` - Success type consumed by `ok_tap`.
/// - `input_err` - Error type consumed by `err_tap`.
/// - `output_ok` - Success type returned by the functions.
/// - `output_err` - Error type returned by the functions.
/// - `ok_tap` - Async function called with the success value returning `Result`.
/// - `err_tap` - Async function called with the error value returning `Result`.
#[macro_export]
macro_rules! define_const_result_tap_both_bind_step_async {
    (
        name = $name:ident,
        input_ok = $input_ok:ty,
        input_err = $input_err:ty,
        output_ok = $output_ok:ty,
        output_err = $output_err:ty,
        ok_tap = $ok_tap:expr,
        err_tap = $err_tap:expr $(,)?
    ) => {
        const $name: result_transformer::flow::async_::step::ResultTapBothBindStepAsync<
            $input_ok,
            $input_err,
            $output_ok,
            $output_err,
            fn(
                $input_ok,
            ) -> std::pin::Pin<
                Box<dyn core::future::Future<Output = Result<$output_ok, $output_err>> + Send>,
            >,
            fn(
                $input_err,
            ) -> std::pin::Pin<
                Box<dyn core::future::Future<Output = Result<$output_ok, $output_err>> + Send>,
            >,
            std::pin::Pin<
                Box<dyn core::future::Future<Output = Result<$output_ok, $output_err>> + Send>,
            >,
            std::pin::Pin<
                Box<dyn core::future::Future<Output = Result<$output_ok, $output_err>> + Send>,
            >,
        > = result_transformer::flow::async_::step::ResultTapBothBindStepAsync::new(
            $ok_tap, $err_tap,
        );
    };

    (
        $name:ident,
        [$input_ok:ty, $input_err:ty => $output_ok:ty, $output_err:ty $(,)?],
        $ok_tap:expr,
        $err_tap:expr $(,)?
    ) => {
        result_transformer::flow::async_::macros::define_const_result_tap_both_bind_step_async!(
            name = $name,
            input_ok = $input_ok,
            input_err = $input_err,
            output_ok = $output_ok,
            output_err = $output_err,
            ok_tap = $ok_tap,
            err_tap = $err_tap
        );
    };
}
pub use define_const_result_tap_both_bind_step_async;
