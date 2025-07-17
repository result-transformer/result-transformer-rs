/// Defines a const [`ErrMapStep`].
///
/// Shorthand syntax: `($name, [$input_err => $output_err], $mapper)`.
///
/// # Parameters
/// - `name` - Identifier for the constant.
/// - `input_err` - Input error type for the mapping function.
/// - `output_err` - Output error type produced by the mapping function.
/// - `mapper` - Function converting the error value.
#[macro_export]
macro_rules! define_const_err_map_step {
    (
        name = $name:ident,
        input_err = $input_err:ty,
        output_err = $output_err:ty,
        mapper = $mapper:expr $(,)?
    ) => {
        const $name: result_transformer::flow::sync::step::ErrMapStep<
            $input_err,
            $output_err,
            fn($input_err) -> $output_err,
        > = result_transformer::flow::sync::step::ErrMapStep::new($mapper);
    };

    (
        $name:ident,
        [$input_err:ty => $output_err:ty $(,)?],
        $mapper:expr $(,)?
    ) => {
        result_transformer::flow::sync::macros::define_const_err_map_step!(
            name = $name,
            input_err = $input_err,
            output_err = $output_err,
            mapper = $mapper
        );
    };
}
pub use define_const_err_map_step;
