/// Implements [`ResultTransformer`] for a given type that already implements both
/// [`OkTransformer`] and [`ErrTransformer`] for the same input types.
///
/// Shorthand syntax: `($impl_for, [$input_ok, $input_err])`.
///
/// This macro is intended for simple cases where a single type is responsible for
/// transforming both `Ok` and `Err` values of a [`Result`].
///
/// # Example
///
/// ```rust
/// impl_result_transformer_via_self_parts! {
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
macro_rules! impl_result_transformer_via_self_parts {
    (
        impl_for = $impl_for:ty,
        input_ok = $input_ok:ty,
        input_err = $input_err:ty $(,)?
    ) => {
        const _: fn() = || {
            fn assert_bounds<
                T: result_transformer::sync::OkTransformer<$input_ok>
                    + result_transformer::sync::ErrTransformer<$input_err>,
            >() {
            }
            assert_bounds::<$impl_for>();
        };

        impl result_transformer::sync::ResultTransformer<$input_ok, $input_err> for $impl_for
        where
            $impl_for: result_transformer::sync::OkTransformer<$input_ok>
                + result_transformer::sync::ErrTransformer<$input_err>,
        {
            type OutputOk = <Self as result_transformer::sync::OkTransformer<$input_ok>>::OutputOk;
            type OutputErr =
                <Self as result_transformer::sync::ErrTransformer<$input_err>>::OutputErr;

            fn transform(
                &self,
                result: Result<$input_ok, $input_err>,
            ) -> Result<Self::OutputOk, Self::OutputErr> {
                match result {
                    Ok(o) => Ok(<Self as result_transformer::sync::OkTransformer<
                        $input_ok,
                    >>::transform_ok(self, o)),
                    Err(e) => Err(<Self as result_transformer::sync::ErrTransformer<
                        $input_err,
                    >>::transform_err(self, e)),
                }
            }
        }
    };

    (
        $impl_for:ty,
        [$input_ok:ty, $input_err:ty $(,)?]
    ) => {
        result_transformer::core::sync::macros::impl_result_transformer_via_self_parts!(
            impl_for = $impl_for,
            input_ok = $input_ok,
            input_err = $input_err
        );
    };
}
pub use impl_result_transformer_via_self_parts;
