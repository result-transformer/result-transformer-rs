/// Defines a const [`ResultMapStep`].
///
/// Shorthand syntax: `($name, [$input_ok, $input_err => $output_ok, $output_err], $mapper)`.
///
/// # Parameters
/// - `name` - Identifier for the constant.
/// - `input_ok` - Success type accepted by the mapper.
/// - `input_err` - Error type accepted by the mapper.
/// - `output_ok` - Success type returned by the mapper.
/// - `output_err` - Error type returned by the mapper.
/// - `mapper` - Function mapping the result value.
#[macro_export]
macro_rules! define_const_result_map_step {
    (
        name = $name:ident,
        input_ok = $input_ok:ty,
        input_err = $input_err:ty,
        output_ok = $output_ok:ty,
        output_err = $output_err:ty,
        mapper = $mapper:expr $(,)?
    ) => {
        const $name: result_transformer::flow::sync::step::ResultMapStep<
            $input_ok,
            $input_err,
            $output_ok,
            $output_err,
            fn(Result<$input_ok, $input_err>) -> Result<$output_ok, $output_err>,
        > = result_transformer::flow::sync::step::ResultMapStep::new($mapper);
    };

    (
        $name:ident,
        [$input_ok:ty, $input_err:ty => $output_ok:ty, $output_err:ty $(,)?],
        $mapper:expr $(,)?
    ) => {
        result_transformer::flow::sync::macros::define_const_result_map_step!(
            name = $name,
            input_ok = $input_ok,
            input_err = $input_err,
            output_ok = $output_ok,
            output_err = $output_err,
            mapper = $mapper
        );
    };
}
pub use define_const_result_map_step;
