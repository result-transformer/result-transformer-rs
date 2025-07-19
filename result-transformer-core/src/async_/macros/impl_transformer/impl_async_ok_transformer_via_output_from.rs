/// Implements [`AsyncOkTransformer`] using [`From`] to convert between the
/// specified input and output types.
///
/// Shorthand syntax: `($impl_for, [$input_ok => $output_ok])`.
///
/// # Parameters
/// - `impl_for` - The type that receives the trait implementation.
/// - `input_ok` - Success type accepted by the transformer.
/// - `output_ok` - Success type produced by the transformer. Must implement
///   `From<input_ok>`.
#[macro_export]
macro_rules! impl_async_ok_transformer_via_output_from {
    (
        impl_for = $impl_for:ty,
        input_ok = $input_ok:ty,
        output_ok = $output_ok:ty $(,)?
    ) => {
        const _: fn() = || {
            fn _assert_from<T: From<$input_ok>>() {}
            _assert_from::<$output_ok>();
        };

        impl result_transformer::async_::AsyncOkTransformer<$input_ok> for $impl_for
        where
            $output_ok: From<$input_ok>,
        {
            type OutputOk = $output_ok;
            fn transform_ok_async<'a>(
                &'a self,
                ok: $input_ok,
            ) -> impl ::core::future::Future<Output = Self::OutputOk> + Send + 'a {
                let _ = self;
                async move { <$output_ok as From<$input_ok>>::from(ok) }
            }
        }
    };

    (
        $impl_for:ty,
        [$input_ok:ty => $output_ok:ty $(,)?]
    ) => {
        result_transformer::core::async_::macros::impl_async_ok_transformer_via_output_from!(
            impl_for = $impl_for,
            input_ok = $input_ok,
            output_ok = $output_ok
        );
    };
}
pub use impl_async_ok_transformer_via_output_from;
