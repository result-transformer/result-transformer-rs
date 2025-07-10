/// Asynchronously converts an error value from `InputErr` to `OutputErr`.
///
/// Similar to [`crate::sync::ErrTransformer`] but returns a future so that
/// the transformation can be awaited.
pub trait AsyncErrTransformer<InputErr> {
    /// Resulting error type emitted by [`transform_err_async`].
    type OutputErr;

    /// Asynchronously convert the input error into the output error.
    ///
    /// # Parameters
    /// * `err` - The error to convert.
    ///
    /// # Returns
    /// A future resolving to the converted error value.
    fn transform_err_async<'a>(
        &'a self,
        err: InputErr,
    ) -> impl Future<Output = Self::OutputErr> + Send + 'a;
}
