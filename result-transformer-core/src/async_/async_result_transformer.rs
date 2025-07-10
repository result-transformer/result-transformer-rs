/// Asynchronously transforms an entire `Result<InputOk, InputErr>` into
/// `Result<OutputOk, OutputErr>`.
///
/// This is the asynchronous analogue to [`crate::sync::ResultTransformer`].
/// Implementations may perform any async operations required to create the new
/// `Result`.
pub trait AsyncResultTransformer<InputOk, InputErr> {
    /// Output type for successful results.
    type OutputOk;
    /// Output type for error results.
    type OutputErr;

    /// Convert the provided result asynchronously.
    ///
    /// # Parameters
    /// * `result` - The result value to transform.
    ///
    /// # Returns
    /// A future resolving to a new `Result` with transformed `Ok` and `Err`
    /// values.
    fn transform_async<'a>(
        &'a self,
        result: Result<InputOk, InputErr>,
    ) -> impl Future<Output = Result<Self::OutputOk, Self::OutputErr>> + Send + 'a;
}
