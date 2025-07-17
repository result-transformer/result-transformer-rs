/// Defines a const [`ResultTapBothStep`].
///
/// Shorthand syntax: `($name, [$input_ok, $input_err => $output_ok, $output_err], $ok_tap, $err_tap)`.
///
/// # Parameters
/// - `name` - Identifier for the constant.
/// - `input_ok` - Success type consumed by `ok_tap`.
/// - `input_err` - Error type consumed by `err_tap`.
/// - `output_ok` - Success type returned by `ok_tap`.
/// - `output_err` - Error type returned by `err_tap`.
/// - `ok_tap` - Function called with the success value.
/// - `err_tap` - Function called with the error value.
#[macro_export]
macro_rules! define_const_result_tap_both_step {
    (
        name = $name:ident,
        input_ok = $input_ok:ty,
        input_err = $input_err:ty,
        output_ok = $output_ok:ty,
        output_err = $output_err:ty,
        ok_tap = $ok_tap:expr,
        err_tap = $err_tap:expr $(,)?
    ) => {
        const $name: result_transformer::flow::sync::step::ResultTapBothStep<
            $input_ok,
            $input_err,
            $output_ok,
            $output_err,
            fn($input_ok) -> $output_ok,
            fn($input_err) -> $output_err,
        > = result_transformer::flow::sync::step::ResultTapBothStep::new($ok_tap, $err_tap);
    };

    (
        $name:ident,
        [$input_ok:ty, $input_err:ty => $output_ok:ty, $output_err:ty $(,)?],
        $ok_tap:expr,
        $err_tap:expr $(,)?
    ) => {
        result_transformer::flow::sync::macros::define_const_result_tap_both_step!(
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
pub use define_const_result_tap_both_step;
