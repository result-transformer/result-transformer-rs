use std::{marker::PhantomData, pin::Pin};

use crate::async_::AsyncResultFlow;

/// Step that passes the entire `Result` to a closure and returns its result.
#[derive(Debug, Clone, Copy)]
pub struct ResultTapStepAsync<TapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    TapFn: Fn(
            Result<InputOk, InputErr>,
        ) -> Pin<Box<dyn Future<Output = Result<OutputOk, OutputErr>> + Send + Sync>>
        + Send
        + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
{
    tap: TapFn,
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<TapFn, InputOk, InputErr, OutputOk, OutputErr>
    ResultTapStepAsync<TapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    TapFn: Fn(
            Result<InputOk, InputErr>,
        ) -> Pin<Box<dyn Future<Output = Result<OutputOk, OutputErr>> + Send + Sync>>
        + Send
        + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
{
    /// Creates a new [`ResultTapStepAsync`].
    ///
    /// * `tap` - closure that processes the entire `Result`
    pub fn new(tap: TapFn) -> Self {
        Self {
            tap,
            _phantom: PhantomData,
        }
    }
}

impl<TapFn, InputOk, InputErr, OutputOk, OutputErr> AsyncResultFlow<InputOk, InputErr>
    for ResultTapStepAsync<TapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    TapFn: Fn(
            Result<InputOk, InputErr>,
        ) -> Pin<Box<dyn Future<Output = Result<OutputOk, OutputErr>> + Send + Sync>>
        + Send
        + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    fn apply_result_async<'a>(
        &'a self,
        input_result: Result<InputOk, InputErr>,
    ) -> impl Future<Output = Result<Self::OutputOk, Self::OutputErr>> + Send + 'a {
        (self.tap)(input_result)
    }
}
