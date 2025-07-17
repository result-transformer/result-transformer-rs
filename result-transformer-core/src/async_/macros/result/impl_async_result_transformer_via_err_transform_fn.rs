/// Implements [`AsyncResultTransformer`] when only the error branch requires mapping.
///
/// Shorthand syntax: `($impl_for, [$input_ok, $input_err => $output_ok, $output_err], $transform_err)`.
#[macro_export]
macro_rules! impl_async_result_transformer_via_err_transform_fn {
    (
        impl_for = $impl_for:ty,
        input_ok = $input_ok:ty,
        input_err = $input_err:ty,
        output_ok = $output_ok:ty,
        output_err = $output_err:ty,
        transform_err = $transform_err:expr $(,)?
    ) => {
        const _: fn() = || {
            fn _type_check<F, Fut>(_f: &F)
            where
                F: Fn($input_err) -> Fut,
                Fut: ::core::future::Future<Output = Result<$output_ok, $output_err>> + Send,
            {
            }
            _type_check(&$transform_err);
        };

        result_transformer::core::async_::macros::impl_async_result_transformer! {
            impl_for = $impl_for,
            input_ok = $input_ok,
            input_err = $input_err,
            output_ok = $output_ok,
            output_err = $output_err,
            transform_result = |result: Result<$input_ok, $input_err>| {
                async move{
                    match result {
                        Ok(ok) => Ok(ok),
                        Err(err) => ($transform_err)(err).await,
                    }
                }
            }
        }
    };

    (
        $impl_for:ty,
        [$input_ok:ty, $input_err:ty => $output_ok:ty, $output_err:ty $(,)?],
        $transform_err:expr $(,)?
    ) => {
        result_transformer::core::async_::macros::impl_async_result_transformer_via_err_transform_fn!(
            impl_for = $impl_for,
            input_ok = $input_ok,
            input_err = $input_err,
            output_ok = $output_ok,
            output_err = $output_err,
            transform_err = $transform_err
        );
    };
}
pub use impl_async_result_transformer_via_err_transform_fn;
