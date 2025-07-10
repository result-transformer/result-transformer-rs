/// Asynchronously transforms a success value from `InputOk` to `OutputOk`.
///
/// This trait is the asynchronous counterpart to [`crate::sync::OkTransformer`].
/// It allows implementations to perform potentially long running conversions
/// without blocking the current thread.
pub trait AsyncOkTransformer<InputOk> {
    /// Resulting success type emitted by [`transform_ok_async`].
    type OutputOk;

    /// Asynchronously convert the input into the output success type.
    ///
    /// # Parameters
    /// * `ok` - The success value to convert.
    ///
    /// # Returns
    /// A future resolving to the converted success value.
    fn transform_ok_async<'a>(
        &'a self,
        ok: InputOk,
    ) -> impl Future<Output = Self::OutputOk> + Send + 'a;
}
