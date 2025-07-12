/// Implements [`ResultTransformer`] for a given type that already implements both
/// [`OkTransformer`] and [`ErrTransformer`] for the same input types.
///
/// This macro is intended for simple cases where a single type is responsible for
/// transforming both `Ok` and `Err` values of a [`Result`].
///
/// # Example
///
/// ```rust
/// impl_result_transformer_from_parts! {
///     impl_for = MyTransformer,
///     input_ok = Success,
///     input_err = Failure,
/// }
/// ```
///
/// The macro generates an implementation like the following:
///
/// ```rust
/// impl ResultTransformer<Success, Failure> for MyTransformer {
///     type OutputOk = ...;
///     type OutputErr = ...;
///
///     fn transform(&self, result: Result<Success, Failure>) -> Result<Self::OutputOk, Self::OutputErr> {
///         match result {
///             Ok(ok) => Ok(self.transform_ok(ok)),
///             Err(err) => Err(self.transform_err(err)),
///         }
///     }
/// }
/// ```
///
/// # Parameters
///
/// - `impl_for`: The type to implement [`ResultTransformer`] for.  
///               It must implement both [`OkTransformer<InputOk>`] and [`ErrTransformer<InputErr>`].
///
/// - `input_ok`: The type of the `Ok` value in the input [`Result`] (`Result<InputOk, InputErr>`).
///
/// - `input_err`: The type of the `Err` value in the input [`Result`] (`Result<InputOk, InputErr>`).
///
/// # Compile-time check
///
/// The macro includes a zero-cost compile-time assertion to ensure that the target
/// type implements both [`OkTransformer`] and [`ErrTransformer`] for the given input types.
///
/// # See also
///
/// - [`OkTransformer`]
/// - [`ErrTransformer`]
/// - [`ResultTransformer`]
#[macro_export]
macro_rules! impl_result_transformer_from_parts {
    (
        impl_for = $ty:ty,
        input_ok = $ok:ty,
        input_err = $err:ty $(,)?
    ) => {
        const _: fn() = || {
            fn assert_bounds<
                T: result_transformer::sync::OkTransformer<$ok>
                    + result_transformer::sync::ErrTransformer<$err>,
            >() {
            }
            assert_bounds::<$ty>();
        };

        impl result_transformer::sync::ResultTransformer<$ok, $err> for $ty
        where
            $ty: result_transformer::sync::OkTransformer<$ok>
                + result_transformer::sync::ErrTransformer<$err>,
        {
            type OutputOk = <Self as result_transformer::sync::OkTransformer<$ok>>::OutputOk;
            type OutputErr = <Self as result_transformer::sync::ErrTransformer<$err>>::OutputErr;

            fn transform(
                &self,
                result: Result<$ok, $err>,
            ) -> Result<Self::OutputOk, Self::OutputErr> {
                match result {
                    Ok(o) => Ok(
                        <Self as result_transformer::sync::OkTransformer<$ok>>::transform_ok(
                            self, o,
                        ),
                    ),
                    Err(e) => Err(
                        <Self as result_transformer::sync::ErrTransformer<$err>>::transform_err(
                            self, e,
                        ),
                    ),
                }
            }
        }
    };
}
pub use impl_result_transformer_from_parts;
