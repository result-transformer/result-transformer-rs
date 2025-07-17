/// Defines a const [`OkTapStep`].
///
/// Shorthand syntax: `($name, [$input_ok => $output_ok], $tap)`.
///
/// # Parameters
/// - `name` - Identifier for the constant.
/// - `input_ok` - Success type consumed by the tap function.
/// - `output_ok` - Success type returned by the tap function.
/// - `tap` - Function called with the success value.
#[macro_export]
macro_rules! define_const_ok_tap_step {
    (
        name = $name:ident,
        input_ok = $input_ok:ty,
        output_ok = $output_ok:ty,
        tap = $tap:expr $(,)?
    ) => {
        const $name: result_transformer::flow::sync::step::OkTapStep<
            $input_ok,
            $output_ok,
            fn($input_ok) -> $output_ok,
        > = result_transformer::flow::sync::step::OkTapStep::new($tap);
    };

    (
        $name:ident,
        [$input_ok:ty => $output_ok:ty $(,)?],
        $tap:expr $(,)?
    ) => {
        result_transformer::flow::sync::macros::define_const_ok_tap_step!(
            name = $name,
            input_ok = $input_ok,
            output_ok = $output_ok,
            tap = $tap
        );
    };
}
pub use define_const_ok_tap_step;
