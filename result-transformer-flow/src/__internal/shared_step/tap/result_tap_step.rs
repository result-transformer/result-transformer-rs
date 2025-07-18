use std::marker::PhantomData;

/// Step that passes the entire `Result` to a closure and returns its result.
#[derive(Debug, Clone, Copy)]
pub struct ResultTapStep<InputOk, InputErr, OutputOk, OutputErr, TapFn>
where
    TapFn: Fn(Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr>,
{
    tap: TapFn,
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<InputOk, InputErr, OutputOk, OutputErr, TapFn>
    ResultTapStep<InputOk, InputErr, OutputOk, OutputErr, TapFn>
where
    TapFn: Fn(Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr>,
{
    /// Creates a new [`ResultTapStep`].
    ///
    /// * `tap` - closure that processes the entire `Result`
    pub const fn new(tap: TapFn) -> Self {
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
