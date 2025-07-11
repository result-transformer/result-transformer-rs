use core::future::Future;
use std::marker::PhantomData;

use crate::async_::AsyncResultFlow;

/// Step that passes the entire `Result` to a closure and returns its result.
#[derive(Debug, Clone, Copy)]
pub struct ResultTapStepAsync<InputOk, InputErr, OutputOk, OutputErr, TapFn, TapFut>
where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    TapFn: Fn(Result<InputOk, InputErr>) -> TapFut + Send + Sync,
    TapFut: Future<Output = Result<OutputOk, OutputErr>> + Send,
{
    tap: TapFn,
    _phantom: PhantomData<(InputOk, InputErr)>,
}

impl<InputOk, InputErr, OutputOk, OutputErr, TapFn, TapFut>
    ResultTapStepAsync<InputOk, InputErr, OutputOk, OutputErr, TapFn, TapFut>
where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    TapFn: Fn(Result<InputOk, InputErr>) -> TapFut + Send + Sync,
    TapFut: Future<Output = Result<OutputOk, OutputErr>> + Send,
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

impl<InputOk, InputErr, OutputOk, OutputErr, TapFn, TapFut> AsyncResultFlow<InputOk, InputErr>
    for ResultTapStepAsync<InputOk, InputErr, OutputOk, OutputErr, TapFn, TapFut>
where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    TapFn: Fn(Result<InputOk, InputErr>) -> TapFut + Send + Sync,
    TapFut: Future<Output = Result<OutputOk, OutputErr>> + Send,
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
