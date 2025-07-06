/// Trait for converting a success value into another type.
///
/// The `InputOk` type parameter describes the success value that will be
/// provided to [`transform_ok`].  `OutputOk` is the type produced after the
/// transformation.
pub trait OkTransformer<InputOk> {
    /// The resulting success type after the conversion.
    type OutputOk;

    /// Performs the conversion of the `ok` value.
    ///
    /// # Parameters
    /// * `ok` - The original success value to be transformed.
    ///
    /// # Returns
    /// A value of type [`Self::OutputOk`] representing the transformed result.
    fn transform_ok(&self, ok: InputOk) -> Self::OutputOk;
}
