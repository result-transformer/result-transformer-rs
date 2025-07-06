#[macro_export]
macro_rules! define_err_transformer_from_err_flow {
    (
        impl_for = $impl_for:ty,
        input_err = $input_err:ty,
        output_err = $output_err:ty,
        flow = $flow:expr $(,)?
    ) => {
        const _: fn() = || {
            fn _flow_type_check<F>(_: &F)
            where
                F: result_transformer::flow::sync::ErrFlow<$input_err, OutputErr = $output_err>,
            {
            }
            _flow_type_check(&$flow);
        };

        result_transformer::macros::define_err_transformer! {
            impl_for = $impl_for,
            input_err = $input_err,
            output_err = $output_err,
            transform_err = |err: $input_err| {
                result_transformer::flow::sync::ErrFlow::apply_err(&$flow, err)
            }
        }
    };
}
