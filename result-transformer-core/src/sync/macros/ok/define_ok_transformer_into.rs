/// Implements [`OkTransformer`] for a type that already provides an `Into` conversion.
///
/// # Parameters
/// - `impl_for` - The target type to implement the trait for.
/// - `from` - The source type which implements `Into<impl_for>`.
#[macro_export]
macro_rules! define_ok_transformer_into {
    (
        impl_for = $impl_for:ty,
        from = $from:ty $(,)?
    ) => {
        const _: fn() = || {
            fn _assert_into<T: Into<$impl_for>>() {}
            _assert_into::<$from>();
        };

        impl result_transformer::sync::OkTransformer<$from> for $impl_for
        where
            $from: Into<$impl_for>,
        {
            type OutputOk = $impl_for;
            fn transform_ok(&self, ok: $from) -> Self::OutputOk {
                let _ = self;
                <$from as Into<$impl_for>>::into(ok)
            }
        }
    };
}
pub use define_ok_transformer_into;
