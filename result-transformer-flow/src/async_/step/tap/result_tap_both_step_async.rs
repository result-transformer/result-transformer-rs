use std::{marker::PhantomData, pin::Pin};

use crate::async_::AsyncResultFlow;

/// Step that applies separate closures to the success and error values.
#[derive(Debug, Clone, Copy)]
pub struct ResultTapBothStepAsync<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkTapFn: Fn(InputOk) -> Pin<Box<dyn Future<Output = OutputOk> + Send + Sync>> + Send + Sync,
    ErrTapFn: Fn(InputErr) -> Pin<Box<dyn Future<Output = OutputErr> + Send + Sync>> + Send + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
{
    ok_tap: OkTapFn,
    err_tap: ErrTapFn,
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr>
    ResultTapBothStepAsync<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkTapFn: Fn(InputOk) -> Pin<Box<dyn Future<Output = OutputOk> + Send + Sync>> + Send + Sync,
    ErrTapFn: Fn(InputErr) -> Pin<Box<dyn Future<Output = OutputErr> + Send + Sync>> + Send + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
{
    /// Creates a new [`ResultTapBothStepAsync`].
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

impl<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr> AsyncResultFlow<InputOk, InputErr>
    for ResultTapBothStepAsync<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkTapFn: Fn(InputOk) -> Pin<Box<dyn Future<Output = OutputOk> + Send + Sync>> + Send + Sync,
    ErrTapFn: Fn(InputErr) -> Pin<Box<dyn Future<Output = OutputErr> + Send + Sync>> + Send + Sync,
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
        async {
            match input_result {
                Ok(ok) => Ok((self.ok_tap)(ok).await),
                Err(err) => Err((self.err_tap)(err).await),
            }
        }
    }
}
