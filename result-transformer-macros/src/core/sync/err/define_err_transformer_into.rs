/// Implements [`ErrTransformer`] for a type that already provides an `Into` conversion.
///
/// # Parameters
/// - `impl_for` - The target type to implement the trait for.
/// - `from` - The source error type which implements `Into<impl_for>`.
#[macro_export]
macro_rules! define_err_transformer_into {
    (
        impl_for = $impl_for:ty,
        from = $from:ty $(,)?
    ) => {
        const _: fn() = || {
            fn _assert_into<T: Into<$impl_for>>() {}
            _assert_into::<$from>();
        };

        impl result_transformer::sync::ErrTransformer<$from> for $impl_for
        where
            $from: Into<$impl_for>,
        {
            type OutputErr = $impl_for;
            fn transform_err(&self, err: $from) -> Self::OutputErr {
                let _ = self;
                <$from as Into<$impl_for>>::into(err)
            }
        }
    };
}