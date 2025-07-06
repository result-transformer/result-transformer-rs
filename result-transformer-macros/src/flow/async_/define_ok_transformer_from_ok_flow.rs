#[macro_export]
macro_rules! define_async_ok_transformer_from_ok_flow {
    (
        impl_for = $impl_for:ty,
        input_ok = $input_ok:ty,
        output_ok = $output_ok:ty,
        flow = $flow:expr $(,)?
    ) => {
        const _: fn() = || {
            fn _assert<F>(_: &F)
            where
                F: result_transformer::flow::async_::AsyncOkFlow<$input_ok, OutputOk = $output_ok>,
            {
            }
            _assert(&$flow);
        };

        result_transformer::macros::define_async_ok_transformer! {
            impl_for = $impl_for,
            input_ok = $input_ok,
            output_ok = $output_ok,
            transform_ok = |ok: $input_ok| {
                result_transformer::flow::async_::AsyncOkFlow::apply_ok(&$flow, ok)
            }
        }
    };
}
