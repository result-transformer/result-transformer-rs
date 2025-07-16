#[macro_export]
macro_rules! define_const_result_map_both_bind_step {
    (
        name = $name:ident,
        input_ok = $input_ok:ty,
        input_err = $input_err:ty,
        output_ok = $output_ok:ty,
        output_err = $output_err:ty,
        ok_mapper = $ok_mapper:expr,
        err_mapper = $err_mapper:expr $(,)?
    ) => {
        const $name: result_transformer::flow::sync::step::ResultMapBothBindStep<
            $input_ok,
            $input_err,
            $output_ok,
            $output_err,
            fn($input_ok) -> Result<$output_ok, $output_err>,
            fn($input_err) -> Result<$output_ok, $output_err>,
        > = result_transformer::flow::sync::step::ResultMapBothBindStep::new(
            $ok_mapper,
            $err_mapper,
        );
    };

    (
        $name:ident,
        [$input_ok:ty, $input_err:ty => $output_ok:ty, $output_err:ty $(,)?],
        $ok_mapper:expr,
        $err_mapper:expr $(,)?
    ) => {
        result_transformer::flow::sync::macros::define_const_result_map_both_bind_step!(
            name = $name,
            input_ok = $input_ok,
            input_err = $input_err,
            output_ok = $output_ok,
            output_err = $output_err,
            ok_mapper = $ok_mapper,
            err_mapper = $err_mapper
        );
    };
}
pub use define_const_result_map_both_bind_step;
