use std::marker::PhantomData;

/// Step that passes the entire `Result` to a closure and returns its result.
#[derive(Debug, Clone, Copy)]
pub struct ResultTapStep<TapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    TapFn: Fn(Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr>,
{
    tap: TapFn,
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<TapFn, InputOk, InputErr, OutputOk, OutputErr>
    ResultTapStep<TapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    TapFn: Fn(Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr>,
{
    /// Creates a new [`ResultTapStep`].
    ///
    /// * `tap` - closure that processes the entire `Result`
    pub fn new(tap: TapFn) -> Self {
        Self {
            tap,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<OutputOk, OutputErr> {
        (self.tap)(input_result)
    }
}
