/// Defines a const [`ResultTapStep`].
///
/// Shorthand syntax: `($name, [$input_ok, $input_err => $output_ok, $output_err], $tap)`.
///
/// # Parameters
/// - `name` - Identifier for the constant.
/// - `input_ok` - Success type consumed by the tap function.
/// - `input_err` - Error type consumed by the tap function.
/// - `output_ok` - Success type returned by the tap function.
/// - `output_err` - Error type returned by the tap function.
/// - `tap` - Function called with the [`Result`] value.
#[macro_export]
macro_rules! define_const_result_tap_step {
    (
        name = $name:ident,
        input_ok = $input_ok:ty,
        input_err = $input_err:ty,
        output_ok = $output_ok:ty,
        output_err = $output_err:ty,
        tap = $tap:expr $(,)?
    ) => {
        const $name: result_transformer::flow::sync::step::ResultTapStep<
            $input_ok,
            $input_err,
            $output_ok,
            $output_err,
            fn(Result<$input_ok, $input_err>) -> Result<$output_ok, $output_err>,
        > = result_transformer::flow::sync::step::ResultTapStep::new($tap);
    };

    (
        $name:ident,
        [$input_ok:ty, $input_err:ty => $output_ok:ty, $output_err:ty $(,)?],
        $tap:expr $(,)?
    ) => {
        result_transformer::flow::sync::macros::define_const_result_tap_step!(
            name = $name,
            input_ok = $input_ok,
            input_err = $input_err,
            output_ok = $output_ok,
            output_err = $output_err,
            tap = $tap
        );
    };
}
pub use define_const_result_tap_step;
