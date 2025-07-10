use std::{marker::PhantomData, pin::Pin};

use crate::async_::AsyncErrFlow;

/// Step that passes the error value to a closure and returns its result.
#[derive(Debug, Clone, Copy)]
pub struct ErrTapStepAsync<TapFn, InputErr, OutputErr>
where
    TapFn: Fn(InputErr) -> Pin<Box<dyn Future<Output = OutputErr> + Send + Sync>> + Send + Sync,
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
{
    tap: TapFn,
    _phantom: PhantomData<(InputErr, OutputErr)>,
}

impl<TapFn, InputErr, OutputErr> ErrTapStepAsync<TapFn, InputErr, OutputErr>
where
    TapFn: Fn(InputErr) -> Pin<Box<dyn Future<Output = OutputErr> + Send + Sync>> + Send + Sync,
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
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

impl<TapFn, InputErr, OutputErr> AsyncErrFlow<InputErr>
    for ErrTapStepAsync<TapFn, InputErr, OutputErr>
where
    TapFn: Fn(InputErr) -> Pin<Box<dyn Future<Output = OutputErr> + Send + Sync>> + Send + Sync,
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
{
    type OutputErr = OutputErr;

    fn apply_err_async<'a>(
        &'a self,
        input_err: InputErr,
    ) -> impl Future<Output = Self::OutputErr> + Send + 'a {
        (self.tap)(input_err)
    }
}
