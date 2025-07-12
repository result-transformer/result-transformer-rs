/// Implements [`OkTransformer`] for a type that already implements `From`.
///
/// # Parameters
/// - `impl_for` - The target type implementing `From<from>`.
/// - `from` - The source type to convert from.
#[macro_export]
macro_rules! define_ok_transformer_from {
    (
        impl_for = $impl_for:ty,
        from = $from:ty $(,)?
    ) => {
        const _: fn() = || {
            fn _assert_from<T: From<$from>>() {}
            _assert_from::<$impl_for>();
        };

        impl result_transformer::sync::OkTransformer<$from> for $impl_for
        where
            $impl_for: From<$from>,
        {
            type OutputOk = $impl_for;
            fn transform_ok(&self, ok: $from) -> Self::OutputOk {
                let _ = self;
                <$impl_for as From<$from>>::from(ok)
            }
        }
    };
}
pub use define_ok_transformer_from;
