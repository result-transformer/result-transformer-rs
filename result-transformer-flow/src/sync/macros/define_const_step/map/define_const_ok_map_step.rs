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
