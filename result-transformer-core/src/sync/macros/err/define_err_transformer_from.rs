/// Implements [`ErrTransformer`] for a type that already implements `From`.
///
/// # Parameters
/// - `impl_for` - The target type implementing `From<from>`.
/// - `from` - The source error type to convert from.
#[macro_export]
macro_rules! define_err_transformer_from {
    (
        impl_for = $impl_for:ty,
        from = $from:ty $(,)?
    ) => {
        const _: fn() = || {
            fn _assert_from<T: From<$from>>() {}
            _assert_from::<$impl_for>();
        };

        impl result_transformer::sync::ErrTransformer<$from> for $impl_for
        where
            $impl_for: From<$from>,
        {
            type OutputErr = $impl_for;
            fn transform_err(&self, err: $from) -> Self::OutputErr {
                let _ = self;
                <$impl_for as From<$from>>::from(err)
            }
        }
    };
}
pub use define_err_transformer_from;
