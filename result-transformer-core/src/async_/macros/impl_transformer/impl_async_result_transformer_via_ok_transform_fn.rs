/// Implements [`AsyncResultTransformer`] when only the success branch requires mapping.
///
/// Shorthand syntax: `($impl_for, [$input_ok, $input_err => $output_ok, $output_err], $transform_ok)`.
#[macro_export]
macro_rules! impl_async_result_transformer_via_ok_transform_fn {
    (
        impl_for = $impl_for:ty,
        input_ok = $input_ok:ty,
        input_err = $input_err:ty,
        output_ok = $output_ok:ty,
        output_err = $output_err:ty,
        transform_ok = $transform_ok:expr $(,)?
    ) => {
        const _: fn() = || {
            fn _type_check<F, Fut>(_f: &F)
            where
                F: Fn($input_ok) -> Fut,
                Fut: ::core::future::Future<Output = Result<$output_ok, $output_err>> + Send,
            {
            }
            _type_check(&$transform_ok);
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
                        Ok(ok) => ($transform_ok)(ok).await,
                        Err(err) => Err(err),
                    }
                }
            }
        }
    };

    (
        $impl_for:ty,
        [$input_ok:ty, $input_err:ty => $output_ok:ty, $output_err:ty $(,)?],
        $transform_ok:expr $(,)?
    ) => {
        result_transformer::core::async_::macros::impl_async_result_transformer_via_ok_transform_fn!(
            impl_for = $impl_for,
            input_ok = $input_ok,
            input_err = $input_err,
            output_ok = $output_ok,
            output_err = $output_err,
            transform_ok = $transform_ok
        );
    };
}
pub use impl_async_result_transformer_via_ok_transform_fn;
