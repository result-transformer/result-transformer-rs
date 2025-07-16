#[macro_export]
macro_rules! define_const_err_tap_step {
    (
        name = $name:ident,
        input_err = $input_err:ty,
        output_err = $output_err:ty,
        tap = $tap:expr $(,)?
    ) => {
        const $name: result_transformer::flow::sync::step::ErrTapStep<
            $input_err,
            $output_err,
            fn($input_err) -> $output_err,
        > = result_transformer::flow::sync::step::ErrTapStep::new($tap);
    };

    (
        $name:ident,
        [$input_err:ty => $output_err:ty $(,)?],
        $tap:expr $(,)?
    ) => {
        result_transformer::flow::sync::macros::define_const_err_tap_step!(
            name = $name,
            input_err = $input_err,
            output_err = $output_err,
            tap = $tap
        );
    };
}
pub use define_const_err_tap_step;
