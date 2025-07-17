/// Defines a const [`OkMapStep`].
///
/// Shorthand syntax: `($name, [$input_ok => $output_ok], $mapper)`.
///
/// # Parameters
/// - `name` - Identifier for the constant.
/// - `input_ok` - Input success type for the mapping function.
/// - `output_ok` - Output success type produced by the mapping function.
/// - `mapper` - Function converting the success value.
#[macro_export]
macro_rules! define_const_ok_map_step {
    (
        name = $name:ident,
        input_ok = $input_ok:ty,
        output_ok = $output_ok:ty,
        mapper = $mapper:expr $(,)?
    ) => {
        const $name: result_transformer::flow::sync::step::OkMapStep<
            $input_ok,
            $output_ok,
            fn($input_ok) -> $output_ok,
        > = result_transformer::flow::sync::step::OkMapStep::new($mapper);
    };

    (
        $name:ident,
        [$input_ok:ty => $output_ok:ty $(,)?],
        $mapper:expr $(,)?
    ) => {
        result_transformer::flow::sync::macros::define_const_ok_map_step!(
            name = $name,
            input_ok = $input_ok,
            output_ok = $output_ok,
            mapper = $mapper
        );
    };
}
pub use define_const_ok_map_step;
