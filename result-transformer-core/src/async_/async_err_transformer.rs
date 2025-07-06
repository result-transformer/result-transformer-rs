use result_transformer_dependencies::*;

/// Asynchronously converts an error value from `InputErr` to `OutputErr`.
///
/// Similar to [`crate::sync::ErrTransformer`] but returns a future so that
/// the transformation can be awaited.
#[async_trait::async_trait]
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
    async fn transform_err_async(&self, err: InputErr) -> Self::OutputErr;
}
