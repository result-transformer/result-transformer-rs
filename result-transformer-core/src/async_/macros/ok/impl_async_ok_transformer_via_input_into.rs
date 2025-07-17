/// Implements [`AsyncOkTransformer`] using [`Into`] to convert from the
/// specified input type to the output type.
///
/// Shorthand syntax: `($impl_for, [$input_ok => $output_ok])`.
///
/// # Parameters
/// - `impl_for` - The type that receives the trait implementation.
/// - `input_ok` - Success type accepted by the transformer. Must implement
///   `Into<output_ok>`.
/// - `output_ok` - Success type produced by the transformer.
#[macro_export]
macro_rules! impl_async_ok_transformer_via_input_into {
    (
        impl_for = $impl_for:ty,
        input_ok = $input_ok:ty,
        output_ok = $output_ok:ty $(,)?
    ) => {
        const _: fn() = || {
            fn _assert_into<T: Into<$output_ok>>() {}
            _assert_into::<$input_ok>();
        };

        impl result_transformer::async_::AsyncOkTransformer<$input_ok> for $impl_for
        where
            $input_ok: Into<$output_ok>,
        {
            type OutputOk = $output_ok;
            fn transform_ok_async<'a>(
                &'a self,
                ok: $input_ok,
            ) -> impl ::core::future::Future<Output = Self::OutputOk> + Send + 'a {
                let _ = self;
                async move { <$input_ok as Into<$output_ok>>::into(ok) }
            }
        }
    };

    (
        $impl_for:ty,
        [$input_ok:ty => $output_ok:ty $(,)?]
    ) => {
        result_transformer::core::async_::macros::impl_async_ok_transformer_via_input_into!(
            impl_for = $impl_for,
            input_ok = $input_ok,
            output_ok = $output_ok
        );
    };
}
pub use impl_async_ok_transformer_via_input_into;
