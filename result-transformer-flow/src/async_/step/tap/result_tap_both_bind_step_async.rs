use core::future::Future;
use std::marker::PhantomData;

use crate::async_::AsyncResultFlow;

/// Step that applies separate closures to success and error values,
/// each returning a `Result`.
#[derive(Debug, Clone, Copy)]
pub struct ResultTapBothBindStepAsync<
    InputOk,
    InputErr,
    OutputOk,
    OutputErr,
    OkTapFn,
    ErrTapFn,
    OkTapFut,
    ErrTapFut,
> where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    OkTapFn: Fn(InputOk) -> OkTapFut + Send + Sync,
    ErrTapFn: Fn(InputErr) -> ErrTapFut + Send + Sync,
    OkTapFut: Future<Output = Result<OutputOk, OutputErr>> + Send,
    ErrTapFut: Future<Output = Result<OutputOk, OutputErr>> + Send,
{
    ok_tap: OkTapFn,
    err_tap: ErrTapFn,
    _phantom: PhantomData<(InputOk, InputErr)>,
}

impl<InputOk, InputErr, OutputOk, OutputErr, OkTapFn, ErrTapFn, OkTapFut, ErrTapFut>
    ResultTapBothBindStepAsync<
        InputOk,
        InputErr,
        OutputOk,
        OutputErr,
        OkTapFn,
        ErrTapFn,
        OkTapFut,
        ErrTapFut,
    >
where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    OkTapFn: Fn(InputOk) -> OkTapFut + Send + Sync,
    ErrTapFn: Fn(InputErr) -> ErrTapFut + Send + Sync,
    OkTapFut: Future<Output = Result<OutputOk, OutputErr>> + Send,
    ErrTapFut: Future<Output = Result<OutputOk, OutputErr>> + Send,
{
    /// Creates a new [`ResultTapBothBindStepAsync`].
    ///
    /// * `ok_fn` - closure for processing the success value
    /// * `err_fn` - closure for processing the error value
    pub fn new(ok_fn: OkTapFn, err_fn: ErrTapFn) -> Self {
        Self {
            ok_tap: ok_fn,
            err_tap: err_fn,
            _phantom: PhantomData,
        }
    }
}

impl<InputOk, InputErr, OutputOk, OutputErr, OkTapFn, ErrTapFn, OkTapFut, ErrTapFut>
    AsyncResultFlow<InputOk, InputErr>
    for ResultTapBothBindStepAsync<
        InputOk,
        InputErr,
        OutputOk,
        OutputErr,
        OkTapFn,
        ErrTapFn,
        OkTapFut,
        ErrTapFut,
    >
where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    OkTapFn: Fn(InputOk) -> OkTapFut + Send + Sync,
    ErrTapFn: Fn(InputErr) -> ErrTapFut + Send + Sync,
    OkTapFut: Future<Output = Result<OutputOk, OutputErr>> + Send,
    ErrTapFut: Future<Output = Result<OutputOk, OutputErr>> + Send,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    fn apply_result_async<'a>(
        &'a self,
        input_result: Result<InputOk, InputErr>,
    ) -> impl Future<Output = Result<Self::OutputOk, Self::OutputErr>> + Send + 'a {
        async {
            match input_result {
                Ok(ok) => (self.ok_tap)(ok).await,
                Err(err) => (self.err_tap)(err).await,
            }
        }
    }
}
