/// Trait for converting a `Result` to another `Result` with different types.
///
/// `InputOk` and `InputErr` describe the incoming `Ok` and `Err` variants of the
/// original [`Result`].  Implementations produce values of type `OutputOk` and
/// `OutputErr` respectively when [`transform`] is invoked.
pub trait ResultTransformer<InputOk, InputErr> {
    /// Type returned when the input `Result` contains an `Ok` value.
    type OutputOk;
    /// Type returned when the input `Result` contains an `Err` value.
    type OutputErr;

    /// Performs the conversion.
    ///
    /// # Parameters
    /// * `result` - The original `Result` to transform.
    ///
    /// # Returns
    /// A new `Result` whose success and error values may have different types.
    fn transform(
        &self,
        result: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr>;
}
