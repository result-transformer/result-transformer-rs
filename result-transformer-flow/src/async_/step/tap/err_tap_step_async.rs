use core::future::Future;
use std::marker::PhantomData;

use crate::async_::AsyncErrFlow;

/// Step that passes the error value to a closure and returns its result.
#[derive(Debug, Clone, Copy)]
pub struct ErrTapStepAsync<InputErr, OutputErr, TapFn, TapFut>
where
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
    TapFn: Fn(InputErr) -> TapFut + Send + Sync,
    TapFut: Future<Output = OutputErr> + Send,
{
    tap: TapFn,
    _phantom: PhantomData<InputErr>,
}

impl<InputErr, OutputErr, TapFn, TapFut> ErrTapStepAsync<InputErr, OutputErr, TapFn, TapFut>
where
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
    TapFn: Fn(InputErr) -> TapFut + Send + Sync,
    TapFut: Future<Output = OutputErr> + Send,
{
    /// Creates a new [`ErrTapStepAsync`].
    ///
    /// * `func` - closure that processes the error value
    pub fn new(func: TapFn) -> Self {
        Self {
            tap: func,
            _phantom: PhantomData,
        }
    }
}

impl<InputErr, OutputErr, TapFn, TapFut> AsyncErrFlow<InputErr>
    for ErrTapStepAsync<InputErr, OutputErr, TapFn, TapFut>
where
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
    TapFn: Fn(InputErr) -> TapFut + Send + Sync,
    TapFut: Future<Output = OutputErr> + Send,
{
    type OutputErr = OutputErr;

    fn apply_err_async<'a>(
        &'a self,
        input_err: InputErr,
    ) -> impl Future<Output = Self::OutputErr> + Send + 'a {
        (self.tap)(input_err)
    }
}
