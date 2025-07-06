/// Trait for converting an error value into another type.
///
/// The `InputErr` type represents the incoming error value passed to
/// [`transform_err`]. Implementors decide how to map that value to an
/// [`OutputErr`].
pub trait ErrTransformer<InputErr> {
    /// The resulting error type after transformation.
    type OutputErr;

    /// Performs the conversion of the error value.
    ///
    /// # Parameters
    /// * `err` - The original error value to be converted.
    ///
    /// # Returns
    /// A value of type [`Self::OutputErr`] representing the converted error.
    fn transform_err(&self, err: InputErr) -> Self::OutputErr;
}
